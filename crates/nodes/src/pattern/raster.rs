use craft_texture::{Image, Kind, Limits};
use materialcraft_core::OutputFormat;
use std::sync::Arc;

use super::{bitmap, blend, pattern_stamp::PatternStamp, region::Region, shape};
use crate::CookContext;
use crate::atomic::input_resize::{InputResize, resized_input};
use crate::function::flush_subnormal;

const MAX_SAMPLES: u64 = 256_000_000;
const MAX_STAMP_COPIES: u64 = 1_048_576;
const MIN_STAMP_SIZE: f32 = 1e-8;

pub(super) fn render(context: &CookContext, stamps: &[PatternStamp], max_depth: u32) -> Result<Image, String> {
    let (width, height) = craft_texture::clamp_size(context.w, context.h);
    let kind = if context.b("color_mode") { Kind::Color } else { Kind::Gray };
    let background = if context.input(0).is_some_and(|image| image.kind() != kind) {
        Image::filled(width, height, kind, [0.0; 4])
    } else {
        resized_input(context, 0, InputResize::PatternBackground).unwrap_or_else(|| {
            let mut color = context.v4("background");
            if kind == Kind::Gray {
                color[0] = (color[0] + color[1] + color[2]) / 3.0;
            }
            Image::filled(width, height, kind, color.map(flush_subnormal))
        })
    };
    if stamps.is_empty() {
        return Ok(background);
    }

    let channels = kind.channels();
    let opacities = depth_opacities(context.f("roughness"), max_depth, context.f("opacity"))?;
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
    let cull_outside = context.b("cull_outside");
    let mut tiling = context.v4("tiling_region");
    for (interval, enabled) in tiling.as_chunks_mut::<2>().0.iter_mut().zip(axes) {
        if !enabled && !cull_outside {
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

        // Check the original center before wrapping can move it back inside the tiling region.
        if (cull_outside && !tile_region.contains(stamp.center)) || stamp.size.iter().any(|size| size.abs() < MIN_STAMP_SIZE) {
            continue;
        }

        if !matches!(stamp.blend, 0..=2) {
            return Err(format!("unsupported pattern blending mode {}", stamp.blend));
        }

        if !(2..=18).contains(&stamp.pattern) && !(256..256 + crate::MAX_IMAGE_INPUTS as i32).contains(&stamp.pattern) {
            return Err(format!("unsupported pattern {}", stamp.pattern));
        }

        let opacity = opacities.get(stamp.depth as usize).copied().ok_or("invalid pattern depth")?;
        if !opacity.is_finite() {
            return Err("pattern roughness overflowed its depth opacity".into());
        }

        let [red, green, blue, alpha] = stamp.color.map(flush_subnormal);
        // Flush the stored alpha before a large tint could amplify a subnormal product.
        let alpha = flush_subnormal(alpha * flush_subnormal(opacity));
        let channel_tint = [red * alpha, green * alpha, blue * alpha, alpha].map(flush_subnormal);
        let tinted_offset = matches!(stamp.pattern, 17 | 18);

        let angle = stamp.rotation * std::f32::consts::TAU;
        if !angle.is_finite() {
            return Err("pattern rotation exceeds the finite angle range".into());
        }
        // Extended quarter turns can retain a small cross-axis component at the stored phase.
        let (sine, cosine) = crate::function::pixel_sin_cos(angle);
        let cardinal = sine == 0.0 || cosine == 0.0;
        let texture = if stamp.pattern >= 256 {
            let index = (stamp.pattern - 256) as usize + 1;
            let Some(input) = context.input(index).filter(|image| image.kind() == kind) else { continue };
            let format = context.input_formats.get(index).copied().unwrap_or(if input.integer_sampling() { OutputFormat::U16 } else { OutputFormat::F32 });
            let level = if stamp.filtering == 0 { bitmap::mip_level(input, stamp.size, pixel_size, sine, cosine) } else { 0 };
            Some((mipmaps.image(index, input, format, level)?, format))
        } else {
            None
        };
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
                            let sample = bitmap::sample(input, local, *format, stamp.filtering == 2).map(flush_subnormal);
                            if kind == Kind::Gray {
                                ([(sample[0] - 1.0).mul_add(alpha, alpha); 4], 1.0)
                            } else {
                                let coverage = sample[3] * alpha;
                                let image_alpha = if stamp.premultiplied { 1.0 } else { 1.0 - (1.0 - sample[3]) };
                                let mut color = std::array::from_fn(|i| flush_subnormal(sample[i] * image_alpha) * channel_tint[i]);
                                color[3] = coverage;
                                (color, coverage)
                            }
                        } else {
                            let (coverage, offset) = shape::profile(stamp.pattern, local[0], local[1], stamp.variation)?;
                            let coverage = flush_subnormal(coverage);
                            let offset = flush_subnormal(offset);
                            let color = channel_tint.map(|tint| if tinted_offset { coverage.mul_add(tint, offset) } else { coverage * tint });
                            (color, color[3])
                        };

                        let pixel = (y as usize * width as usize + x as usize) * channels;
                        let target = pixels.get_mut(pixel..pixel + channels).ok_or("invalid pattern pixel")?;
                        let stamp_alpha = flush_subnormal(stamp_alpha);
                        for (channel, target) in target.iter_mut().enumerate() {
                            let value = flush_subnormal(foreground[channel]);
                            let background = flush_subnormal(*target);
                            *target = flush_subnormal(match (kind, stamp.blend) {
                                (Kind::Color, _) => blend::color(background, value, stamp_alpha, stamp.blend, channel),
                                (Kind::Gray, 1 | 2) => background.max(value),
                                (Kind::Gray, _) if value == 0.0 => background,
                                (Kind::Gray, _) => background + value,
                            });
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

fn depth_opacities(roughness: f32, max_depth: u32, opacity: f32) -> Result<Vec<f32>, String> {
    // Match the depth allocation cap to the 4096-node acyclic traversal limit
    if max_depth >= 4096 || !roughness.is_finite() || roughness < 0.0 {
        return Err("pattern roughness must be finite and nonnegative, with depth below 4096".into());
    }

    let count = max_depth + 1;
    // A zero roughness gives every visited depth the full opacity.
    if roughness == 0.0 {
        return Ok(vec![opacity; count as usize]);
    }

    // Round the geometric endpoint before subtracting it, particularly near unit roughness.
    let first_weight = if roughness == 1.0 {
        1.0 / count as f32
    } else {
        let endpoint = roughness.powf(count as f32);
        (1.0 - roughness) / (1.0 - endpoint)
    };

    // Each child inherits its parent's stored opacity, including the global multiplier.
    Ok(std::iter::successors(Some(first_weight * opacity), |parent| Some(parent * roughness)).take(count as usize).collect())
}
