use super::ATOMIC;
use crate::{CookContext, CookResult, NodeDefinition, node};
use craft_texture::Kind;
use materialcraft_core::OutputFormat;

pub(super) fn definition() -> NodeDefinition {
    node("bitmap", "Bitmap", ATOMIC)
        .desc("A bitmap resource of the package, resampled to the node size.")
        .atomic()
        .generator()
        .resource("resource", "Bitmap resource")
        .choice("colour_space", "Color space", &["sRGB", "Linear"], 0)
        .boolean("color_mode", "Color output", true)
        .any_out()
        .cook(bitmap)
}

fn bitmap(context: &CookContext) -> CookResult {
    let name = context.text("resource");
    let image = match (context.resource)(&craft_texture::resource_key(name, context.i("colour_space") == 1)) {
        Some(image)
            if image.kind() == Kind::Gray
                && image.integer_sampling()
                && (image.width(), image.height()) != (context.w, context.h)
                && [image.width(), image.height(), context.w, context.h].into_iter().all(u32::is_power_of_two) =>
        {
            // Bitmap filtering clamps at the resource edge and stores each level at the node precision.
            crate::resize_bound_image(image, (context.w, context.h), Kind::Gray, OutputFormat::U16, context.format, [false; 2])
        }
        Some(image) => image.resize(context.w, context.h, context.tile),
        None if name.is_empty() => context.gen_color(|u, v| {
            let c = if ((u * 8.0) as i32 + (v * 8.0) as i32) % 2 == 0 { 0.8 } else { 0.2 };
            [c, c, c, 1.0]
        }),
        None => return Err(format!("no bitmap resource `{name}`")),
    };

    Ok(vec![if context.b("color_mode") { image.to_color() } else { image.to_gray() }])
}
