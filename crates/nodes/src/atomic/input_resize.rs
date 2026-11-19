use crate::CookContext;
use craft_texture::{Image, Kind};
use materialcraft_core::OutputFormat;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputResize {
    Copy,
    Filter,
    Levels,
    Warp,
}

pub(crate) fn resized_input(context: &CookContext, port: usize, mode: InputResize) -> Option<Image> {
    resized_texture(context, port, mode).map(|(image, _)| image)
}

pub(super) fn blur_input(context: &CookContext, intensity: f32) -> (Image, OutputFormat) {
    // A zero intensity copies through the destination format. Even a subpixel blur uses the working precision.
    let mode = if intensity == 0.0 { InputResize::Copy } else { InputResize::Filter };
    resized_texture(context, 0, mode).unwrap_or_else(|| (Image::filled(context.w, context.h, Kind::Gray, [0.0; 4]), OutputFormat::F32))
}

pub(super) fn warp_input(context: &CookContext, port: usize) -> (Image, OutputFormat) {
    resized_texture(context, port, InputResize::Warp).unwrap_or_else(|| (Image::filled(context.w, context.h, Kind::Gray, [0.0; 4]), context.format))
}

fn resized_texture(context: &CookContext, port: usize, mode: InputResize) -> Option<(Image, OutputFormat)> {
    let mut image = context.input(port)?.clone();
    let mut source = context.input_formats.get(port).copied().unwrap_or(if image.integer_sampling() { OutputFormat::U16 } else { OutputFormat::F32 });
    let target = craft_texture::clamp_size(context.w, context.h);
    let size = (image.width(), image.height());
    // A warp can take its final taps directly from an input up to one level larger on either axis.
    let direct_warp =
        mode == InputResize::Warp && (target.0..=target.0.saturating_mul(2)).contains(&size.0) && (target.1..=target.1.saturating_mul(2)).contains(&size.1);
    if size == target || direct_warp {
        return Some((image, source));
    }
    let storage = if mode == InputResize::Copy {
        context.format
    } else {
        use OutputFormat::*;
        match (source, context.format) {
            (F32, _) | (_, F32) => F32,
            (F16, _) | (_, F16) => F16,
            (U8, U8) => U8,
            _ => U16,
        }
    };
    let advance = |now: u32, end: u32| {
        if now.saturating_mul(2) < end {
            now.saturating_mul(2)
        } else if end.saturating_mul(2) < now {
            now.div_ceil(2)
        } else {
            now
        }
    };
    loop {
        // Leave each axis within one doubling or halving for the final two-dimensional filter.
        let current = (image.width(), image.height());
        let intermediate = (advance(current.0, target.0), advance(current.1, target.1));
        let (w, h) = if intermediate == current { target } else { intermediate };
        image = crate::texture_sampler::resize_input_level(&image, w, h, source, context.tiling_axes());
        // A final reduction can feed the Levels curve directly. An enlargement stores the
        // filtered texels first, which matters when the working image uses half precision.
        if mode == InputResize::Levels && (w, h) == target && w <= current.0 && h <= current.1 {
            return Some((image, source));
        }
        image = crate::output_precision::apply(image, storage);
        if (w, h) == target {
            return Some((image, storage));
        }
        source = storage;
    }
}
