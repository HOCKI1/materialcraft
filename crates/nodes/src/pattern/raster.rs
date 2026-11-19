use craft_texture::{Image, Kind, Limits};
use materialcraft_core::OutputFormat;
use std::sync::Arc;

use super::{bitmap, pattern_stamp::PatternStamp, region::Region, shape};
use crate::CookContext;

const MAX_SAMPLES: u64 = 256_000_000;
const MAX_STAMP_COPIES: u64 = 1_048_576;
const MIN_STAMP_SIZE: f32 = 1e-8;

pub(super) fn render(context: &CookContext, stamps: &[PatternStamp], max_depth: u32) -> Result<Image, String> {
    let (width, height) = craft_texture::clamp_size(context.w, context.h);
    let kind = if context.b("color_mode") { Kind::Color } else { Kind::Gray };
    let mut background = context.input(0).map(|image| image.resize(width, height, context.tile)).unwrap_or_else(|| {
        let mut color = context.v4("background");
        if kind == Kind::Gray {
            color[0] = (color[0] + color[1] + color[2]) / 3.0;
        }
        Image::filled(width, height, kind, color)
    });
    background = if kind == Kind::Gray { background.to_gray() } else { background.to_color() };
    if stamps.is_empty() {
        return Ok(background);
    }

    let channels = kind.channels();
    let opacity = context.f("opacity").max(0.0);
    let weights = depth_weights(context.f("roughness"), max_depth)?;
    let integer_sampling = background.integer_sampling();
    let mut data = Arc::clone(background.shared_data());
    drop(background);

    let pixels = Arc::make_mut(&mut data);
    let mut budget = MAX_SAMPLES;
    let mut copies_left = MAX_STAMP_COPIES;
    let render_region = Region::new(context.v4("render_region"))?;
    let span = render_region.period();
    let pixel_size = [span[0] / width as f32, span[1] / height as f32];
    let axes = context.tiling_axes();
    let mut tiling = context.v4("tiling_region");
    for (interval, enabled) in tiling.as_chunks_mut::<2>().0.iter_mut().zip(axes) {
        if !enabled {
            *interval = [0.0, 1.0];
        }
    }
    let tile_region = Region::new(tiling)?;
    let period = tile_region.period();
    let mut mipmaps = bitmap::Mipmaps::new();

    for stamp in stamps {
        if !stamp.center.iter().chain(&stamp.size).chain(&stamp.color).chain([&stamp.rotation, &stamp.variation]).all(|value| value.is_finite()) {
            return Err("pattern parameters must be finite".into());
        }

        if stamp.size.iter().any(|size| size.abs() < MIN_STAMP_SIZE) {
            continue;
        }

        if !matches!(stamp.blend, 0..=2) {
            return Err(format!("unsupported pattern blending mode {}", stamp.blend));
        }

        if !(2..=18).contains(&stamp.pattern) && !(256..256 + crate::MAX_IMAGE_INPUTS as i32).contains(&stamp.pattern) {
            return Err(format!("unsupported pattern {}", stamp.pattern));
        }

        let opacity = opacity * weights.get(stamp.depth as usize).copied().ok_or("invalid pattern depth")?;
        if !opacity.is_finite() {
            return Err("pattern roughness overflowed its depth opacity".into());
        }

        // Scalar luminosity weights both RGB and alpha, unlike straight RGBA tint
        let source_alpha = if stamp.scalar_color { stamp.color[0] } else { stamp.color[3] };
        let tinted_offset = matches!(stamp.pattern, 17 | 18);

        let quarters = stamp.rotation * 4.0;
        let cardinal = quarters.fract() == 0.0;
        let (sine, cosine) = if cardinal {
            match quarters.rem_euclid(4.0) as u32 {
                0 => (0.0, 1.0),
                1 => (1.0, 0.0),
                2 => (0.0, -1.0),
                _ => (-1.0, 0.0),
            }
        } else {
            (stamp.rotation * std::f32::consts::TAU).sin_cos()
        };
        let texture = if stamp.pattern >= 256 {
            let index = (stamp.pattern - 256) as usize + 1;
            let Some(input) = context.input(index).filter(|image| image.kind() == kind) else { continue };
            let format = context.input_formats.get(index).copied().unwrap_or(if input.integer_sampling() { OutputFormat::U16 } else { OutputFormat::F32 });
            let level = if stamp.filtering == 0 { bitmap::mip_level(input, stamp.size, pixel_size, sine, cosine) } else { 0 };
            Some((mipmaps.image(index, input, format, level)?, format))
        } else {
            None
        };
        let gray_tint = (stamp.color[0] + stamp.color[1] + stamp.color[2]) / 3.0 * opacity;
        let channel_tint = stamp.color.map(|tint| tint * (stamp.color[3] * opacity));
        let radius = [
            (stamp.size[0] * cosine).abs() * 0.5 + (stamp.size[1] * sine).abs() * 0.5,
            (stamp.size[0] * sine).abs() * 0.5 + (stamp.size[1] * cosine).abs() * 0.5,
        ];
        let center = std::array::from_fn(|axis| if axes[axis] { stamp.center[axis].rem_euclid(period[axis]) } else { stamp.center[axis] });
        let repeats = tile_region.repeats(&render_region, center, radius, axes)?;
        let count = |axis: usize| (i64::from(repeats[axis][1]) - i64::from(repeats[axis][0]) + 1).max(0) as u64;
        let copies = count(0).checked_mul(count(1)).ok_or("pattern repetition exceeds its budget")?;
        if copies == 0 {
            continue;
        }
        copies_left = copies_left.checked_sub(copies).ok_or("pattern repetition exceeds its budget")?;
        for repeat_y in repeats[1][0]..=repeats[1][1] {
            for repeat_x in repeats[0][0]..=repeats[0][1] {
                let center = [center[0] + repeat_x as f32 * period[0], center[1] + repeat_y as f32 * period[1]];
                let [[left, right], [top, bottom]] = render_region.bounds(center, radius, [width, height]);
                let samples = u64::from(right.saturating_sub(left)) * u64::from(bottom.saturating_sub(top));
                budget = budget.checked_sub(samples).ok_or("pattern rendering exceeded its pixel budget")?;
                for y in top..bottom {
                    for x in left..right {
                        let point = render_region.point([x, y], [width, height]);
                        let delta = [point[0] - center[0], point[1] - center[1]];
                        let local = [(delta[0] * cosine + delta[1] * sine) * 2.0 / stamp.size[0], (-delta[0] * sine + delta[1] * cosine) * 2.0 / stamp.size[1]];

                        // On axis-aligned stamps the preceding image interval owns a shared sample center,
                        // including when a quarter turn reverses a local axis.
                        let outside = if cardinal {
                            delta.iter().zip(radius).any(|(&v, r)| v <= -r || v > r)
                        } else {
                            local.iter().any(|coordinate| *coordinate <= -1.0 || *coordinate > 1.0)
                        };
                        if outside {
                            continue;
                        }

                        let (foreground, stamp_alpha) = if let Some((input, format)) = &texture {
                            let sample = bitmap::sample(input, local, *format, stamp.filtering == 2);
                            if kind == Kind::Gray {
                                ([(sample[0] - 1.0).mul_add(gray_tint, gray_tint); 4], 1.0)
                            } else {
                                let coverage = sample[3] * (source_alpha * opacity);
                                let image_alpha = if stamp.premultiplied { 1.0 } else { 1.0 - (1.0 - sample[3]) };
                                let mut color = std::array::from_fn(|i| (sample[i] * image_alpha) * channel_tint[i]);
                                color[3] = coverage;
                                (color, coverage)
                            }
                        } else {
                            let (coverage, offset) = shape::profile(stamp.pattern, local[0], local[1], stamp.variation)?;
                            let alpha = if tinted_offset { coverage.mul_add(source_alpha * opacity, offset) } else { coverage * source_alpha * opacity };
                            let color = std::array::from_fn(|i| {
                                let tint_alpha = if kind == Kind::Color && i < 3 { stamp.color[3] } else { 1.0 };
                                let tint = if i == 3 { source_alpha } else { stamp.color[i] };
                                if tinted_offset { coverage.mul_add(tint * tint_alpha * opacity, offset) } else { coverage * tint * tint_alpha * opacity }
                            });
                            (color, alpha)
                        };

                        let pixel = (y as usize * width as usize + x as usize) * channels;
                        let target = pixels.get_mut(pixel..pixel + channels).ok_or("invalid pattern pixel")?;
                        for (channel, target) in target.iter_mut().enumerate() {
                            let value = foreground[channel];
                            *target = match (kind, stamp.blend) {
                                (Kind::Gray, 1 | 2) => target.max(value),
                                (Kind::Color, 0..=2) if texture.is_some() => bitmap::composite(*target, value, stamp_alpha, stamp.blend, channel),
                                (Kind::Color, 0) if channel == 3 => value.mul_add(1.0 - *target, *target),
                                (Kind::Color, 1 | 2) => target.mul_add(1.0 - stamp_alpha, value),
                                _ => *target + value,
                            };
                            if !target.is_finite() {
                                return Err("pattern accumulation overflowed".into());
                            }
                        }
                    }
                }
            }
        }
    }

    Image::from_data(width, height, kind, data, &Limits::default()).map(|image| image.with_integer_sampling(integer_sampling)).map_err(|e| e.to_string())
}

fn depth_weights(roughness: f32, max_depth: u32) -> Result<Vec<f32>, String> {
    // Match the depth allocation cap to the 4096-node acyclic traversal limit
    if max_depth >= 4096 || !roughness.is_finite() || roughness < 0.0 {
        return Err("pattern roughness must be finite and nonnegative, with depth below 4096".into());
    }

    let mut weights = vec![1.0_f64; max_depth as usize + 1];

    // Zero is the native no-attenuation mode, not the limit of the normalized geometric weights
    if roughness == 0.0 {
        return Ok(weights.into_iter().map(|value| value as f32).collect());
    }

    // Normalize depth weights across drawing and non-drawing ancestors
    // Start at the largest weight to avoid overflow
    let mut weight = 1.0;
    let ratio = f64::from(roughness);
    if roughness <= 1.0 {
        for value in &mut weights {
            *value = weight;
            weight *= ratio;
        }
    } else {
        for value in weights.iter_mut().rev() {
            *value = weight;
            weight /= ratio;
        }
    }

    let sum: f64 = weights.iter().sum();
    Ok(weights.into_iter().map(|value| (value / sum) as f32).collect())
}
