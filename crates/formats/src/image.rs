use crate::{Depth, FormatError, Result, guard};
use craft_texture::{Image, Kind, Limits, MAX_SIDE, linear_to_srgb, srgb_to_linear};
use materialcraft_nodes::BitmapColorSpace;
use std::io::Cursor;

const MAX_DECODE_BYTES: u64 = 1 << 30;

/// Decode image file bytes into a linear float image (8-bit color is sRGB-decoded; grayscale files stay grayscale)
pub fn decode(bytes: &[u8], linear: bool) -> Result<Image> {
    decode_with_color_space(bytes, if linear { BitmapColorSpace::Linear } else { BitmapColorSpace::Srgb })
}

/// Decode a bitmap using its file transfer or an explicit interpretation of its stored channels.
pub fn decode_with_color_space(bytes: &[u8], space: BitmapColorSpace) -> Result<Image> {
    if bytes.starts_with(b"DDS ") {
        return guard(|| crate::dds_image::decode(bytes, space == BitmapColorSpace::Linear), || FormatError::Image("DDS decoder failed".into()));
    }

    const SVG_SNIFF_BYTES: usize = 4096;
    if std::str::from_utf8(bytes.get(..bytes.len().min(SVG_SNIFF_BYTES)).unwrap_or_default()).is_ok_and(|text| text.contains("<svg")) {
        return guard(|| crate::svg::decode(bytes), || FormatError::Image("SVG decoder failed".into()));
    }

    match decode_raster(bytes, space) {
        Err(error) if bytes.starts_with(&[0x76, 0x2f, 0x31, 0x01]) => {
            guard(|| decode_luminance_exr(bytes, error), || FormatError::Image("EXR decoder failed".into()))
        }
        result => result,
    }
}

fn decode_luminance_exr(bytes: &[u8], original_error: FormatError) -> Result<Image> {
    use craft_image_codecs::{ChannelLayout, DecodeOptions, Format};
    let limits = craft_image_codecs::Limits { max_width: MAX_SIDE, max_height: MAX_SIDE, max_pixels: u64::from(MAX_SIDE).pow(2), max_alloc: MAX_DECODE_BYTES };
    let decoded = craft_image_codecs::decode_as_with(Format::OpenExr, bytes, &DecodeOptions { limits, ..Default::default() })
        .map_err(|error| FormatError::Image(error.to_string()))?;
    let data = match decoded.layout() {
        ChannelLayout::Gray => decoded.to_normalized(),
        ChannelLayout::GrayA => (0..decoded.pixel_count()).map(|i| decoded.sample_normalized(2 * i)).collect(),
        _ => return Err(original_error),
    };
    Image::from_data(decoded.width(), decoded.height(), Kind::Gray, data, &Limits::default()).map_err(|error| FormatError::Image(error.to_string()))
}

fn raster_limits() -> image::Limits {
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_SIDE);
    limits.max_image_height = Some(MAX_SIDE);
    limits.max_alloc = Some(MAX_DECODE_BYTES);
    limits
}

fn png_channel_table(bytes: &[u8], maximum: u32) -> Option<Vec<f32>> {
    let metadata = craft_image_codecs::png_probe::color_chunks(bytes)?;
    if metadata.srgb {
        return None;
    }
    // Recover the five-place file gamma before rounding the correction exponent.
    let gamma = (f64::from(metadata.gamma?) * 100_000.0).round() / 100_000.0;
    let ratio = gamma * 2.2;
    let exponent = (100_000.0 / ratio).round() / 100_000.0;
    if (ratio * 100_000.0).round() >= 95_000.0 && exponent >= 0.95 {
        return None;
    }
    let levels = f64::from(maximum);
    Some((0..=maximum).map(|v| ((f64::from(v) / levels).powf(exponent) * levels).round() as f32 / maximum as f32).collect())
}

fn mapped_channel(table: &[f32], value: f32) -> f32 {
    let last = table.len().saturating_sub(1) as f32;
    table.get((value * last).round() as usize).copied().unwrap_or(value)
}

fn decode_raster(bytes: &[u8], space: BitmapColorSpace) -> Result<Image> {
    let open = || -> Result<image::ImageReader<Cursor<&[u8]>>> {
        let mut reader = image::ImageReader::new(Cursor::new(bytes)).with_guessed_format().map_err(|e| FormatError::Image(e.to_string()))?;
        if reader.format().is_none() {
            reader.set_format(image::ImageFormat::Tga);
        }
        reader.limits(raster_limits());
        Ok(reader)
    };
    let failed = || FormatError::Image("the decoder failed on this file".into());
    let image_error = |e| match e {
        image::ImageError::Limits(_) => FormatError::Image(format!("image is larger than {MAX_SIDE} pixels")),
        e => FormatError::Image(e.to_string()),
    };

    let (input_format, (w, h)) = guard(
        || {
            let reader = open()?;
            Ok((reader.format(), reader.into_dimensions().map_err(image_error)?))
        },
        failed,
    )?;
    if w == 0 || h == 0 || w > MAX_SIDE || h > MAX_SIDE {
        return Err(FormatError::Image(format!("{w}x{h} is outside 1..={MAX_SIDE} pixels")));
    }

    let png =
        if space == BitmapColorSpace::FromFile && input_format == Some(image::ImageFormat::Png) { materialcraft_core::png_raster_info(bytes) } else { None };
    let stored = match space {
        BitmapColorSpace::Linear => true,
        BitmapColorSpace::Srgb => false,
        BitmapColorSpace::FromFile => {
            png.is_some()
                || matches!(input_format, Some(image::ImageFormat::Jpeg | image::ImageFormat::Bmp | image::ImageFormat::Tiff | image::ImageFormat::Tga))
        }
    };
    let decoded = guard(|| open()?.decode().map_err(image_error), failed)?;
    let table = png.and_then(|info| {
        let maximum = if info.format == materialcraft_core::OutputFormat::U8 { 255 } else { 65535 };
        png_channel_table(bytes, maximum)
    });
    let gray = matches!(decoded.color(), image::ColorType::L8 | image::ColorType::L16 | image::ColorType::La8 | image::ColorType::La16);
    let float = matches!(decoded.color(), image::ColorType::Rgb32F | image::ColorType::Rgba32F);
    if gray {
        let luminance = decoded.into_luma16();
        let data: Vec<f32> = luminance
            .pixels()
            .map(|p| {
                let value = f32::from(p.0[0]) / 65535.0;
                table.as_ref().map_or(value, |levels| mapped_channel(levels, value))
            })
            .collect();
        return Image::from_data(w, h, Kind::Gray, data, &Limits::default())
            .map(|image| image.with_integer_sampling(true))
            .map_err(|e| FormatError::Image(e.to_string()));
    }

    let mut data = decoded.into_rgba32f().into_raw();
    if let Some(table) = table {
        for px in data.as_chunks_mut::<4>().0 {
            for channel in &mut px[..3] {
                *channel = mapped_channel(&table, *channel);
            }
        }
    } else if !(float || stored) {
        for px in data.as_chunks_mut::<4>().0 {
            for c in px.iter_mut().take(3) {
                *c = srgb_to_linear(*c);
            }
        }
    }

    Image::from_data(w, h, Kind::Color, data, &Limits::default())
        .map(|image| image.with_integer_sampling(stored && !float))
        .map_err(|e| FormatError::Image(e.to_string()))
}

/// Encode `image` as `ext` (see [`EXPORT_FORMATS`]). `srgb`: color values are written through the sRGB curve (base color); data maps (normal, roughness…) are written linear
pub fn encode(image: &Image, ext: &str, depth: Depth, srgb: bool) -> Result<Vec<u8>> {
    let ext = ext.trim_start_matches('.').to_ascii_lowercase();
    let encode_channel = |v: f32| if srgb && image.kind() == Kind::Color { linear_to_srgb(v) } else { v };
    let (w, h) = (image.width(), image.height());
    let color = image.kind() == Kind::Color;
    let pixel = |i: usize| -> [f32; 4] {
        if color {
            let s = image.data().get(i * 4..i * 4 + 4).unwrap_or(&[0.0, 0.0, 0.0, 1.0]);
            [encode_channel(s[0]), encode_channel(s[1]), encode_channel(s[2]), s[3]]
        } else {
            let v = image.data().get(i).copied().unwrap_or(0.0);
            [v, v, v, 1.0]
        }
    };

    let pixel_count = (w * h) as usize;
    let format = match ext.as_str() {
        "png" => image::ImageFormat::Png,
        "jpg" | "jpeg" => image::ImageFormat::Jpeg,
        "tga" => image::ImageFormat::Tga,
        "bmp" => image::ImageFormat::Bmp,
        "webp" => image::ImageFormat::WebP,
        "exr" => image::ImageFormat::OpenExr,
        "hdr" => image::ImageFormat::Hdr,
        other => return Err(FormatError::Image(format!("unsupported export format `{other}`"))),
    };

    let float_only = matches!(format, image::ImageFormat::OpenExr | image::ImageFormat::Hdr);
    let sixteen_ok = format == image::ImageFormat::Png;
    let quantize_u8 = |v: f32| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    let quantize_u16 = |v: f32| (v.clamp(0.0, 1.0) * 65535.0 + 0.5) as u16;
    let dynamic_image: image::DynamicImage = if float_only {
        let data: Vec<f32> = (0..pixel_count)
            .flat_map(|i| {
                let p = pixel(i);
                [p[0], p[1], p[2]]
            })
            .collect();
        let buffer = image::Rgb32FImage::from_raw(w, h, data).ok_or_else(|| FormatError::Image("bad buffer".into()))?;
        buffer.into()
    } else if depth != Depth::U8 && sixteen_ok {
        if color {
            let data: Vec<u16> = (0..pixel_count).flat_map(|i| pixel(i).map(quantize_u16)).collect();
            image::ImageBuffer::<image::Rgba<u16>, _>::from_raw(w, h, data).ok_or_else(|| FormatError::Image("bad buffer".into()))?.into()
        } else {
            let data: Vec<u16> = (0..pixel_count).map(|i| quantize_u16(pixel(i)[0])).collect();
            image::ImageBuffer::<image::Luma<u16>, _>::from_raw(w, h, data).ok_or_else(|| FormatError::Image("bad buffer".into()))?.into()
        }
    } else if color && !matches!(format, image::ImageFormat::Jpeg) {
        let data: Vec<u8> = (0..pixel_count).flat_map(|i| pixel(i).map(quantize_u8)).collect();
        image::RgbaImage::from_raw(w, h, data).ok_or_else(|| FormatError::Image("bad buffer".into()))?.into()
    } else if color {
        let data: Vec<u8> = (0..pixel_count)
            .flat_map(|i| {
                let p = pixel(i);
                [quantize_u8(p[0]), quantize_u8(p[1]), quantize_u8(p[2])]
            })
            .collect();
        image::RgbImage::from_raw(w, h, data).ok_or_else(|| FormatError::Image("bad buffer".into()))?.into()
    } else {
        let data: Vec<u8> = (0..pixel_count).map(|i| quantize_u8(pixel(i)[0])).collect();
        image::GrayImage::from_raw(w, h, data).ok_or_else(|| FormatError::Image("bad buffer".into()))?.into()
    };

    let mut output = Cursor::new(Vec::new());
    dynamic_image.write_to(&mut output, format).map_err(|e| FormatError::Image(e.to_string()))?;
    Ok(output.into_inner())
}
