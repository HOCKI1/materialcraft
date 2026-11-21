use craft_texture::{Image, Kind};
use materialcraft_formats::{BITMAP_EXTENSIONS, BITMAP_FORMATS, Depth, decode, encode};
use std::io::Cursor;

#[test]
fn grayscale_png_file_gamma_uses_the_measured_word_boundaries() {
    let words = [1136, 3683, 4236];
    let field = Image::gray(3, 1, |x, _| words[x as usize] as f32 / 65535.0);
    let plain = encode(&field, "png", Depth::U16, false).unwrap();
    for (gamma, crc, expected) in
        [(43_181u32, 0x459c_89f9u32, [918, 3165, 3667]), (43_182, 0xdc95_d843, words), (47_847, 0xab61_51a4, words), (47_848, 0x3bde_4c35, [1391, 4253, 4858])]
    {
        let mut bytes = plain.clone();
        let chunk = [4u32.to_be_bytes(), *b"gAMA", gamma.to_be_bytes(), crc.to_be_bytes()].concat();
        bytes.splice(33..33, chunk);
        let image = materialcraft_formats::decode_with_color_space(&bytes, materialcraft_nodes::BitmapColorSpace::FromFile).unwrap();
        let stored: Vec<_> = image.data().iter().map(|v| (v * 65535.0).round() as u16).collect();
        assert_eq!(stored, expected, "file gamma {gamma}");
    }
}

#[test]
fn bitmap_filters_are_specific_and_match_supported_extensions() {
    let extensions: std::collections::BTreeSet<_> = BITMAP_FORMATS.iter().flat_map(|(_, extensions)| extensions.iter().copied()).collect();

    assert_eq!(extensions, BITMAP_EXTENSIONS.iter().copied().collect());

    for excluded in ["svg", "psd", "psb", "surface", "raw", "jng", "wbmp", "*"] {
        assert!(!extensions.contains(excluded));
    }

    assert_eq!(BITMAP_FORMATS[0], ("All formats", BITMAP_EXTENSIONS));
    assert_eq!(materialcraft_formats::VECTOR_FORMATS[0], ("All formats", materialcraft_formats::VECTOR_EXTENSIONS));
}

#[test]
fn luminance_exr_preserves_linear_signed_and_hdr_samples() {
    use craft_image_codecs::{ChannelLayout, EncodeOptions, Format, SampleType};
    for sample in [SampleType::F16, SampleType::F32] {
        for layout in [ChannelLayout::Gray, ChannelLayout::GrayA] {
            let values = [-0.25, 0.0, 0.5, 3.0];
            let input: Vec<_> = if layout == ChannelLayout::GrayA { values.into_iter().flat_map(|y| [y, 0.375]).collect() } else { values.to_vec() };
            let source = craft_image_codecs::Image::from_normalized(4, 1, layout, sample, &input).unwrap();
            let encoded = craft_image_codecs::encode(&source, Format::OpenExr, &EncodeOptions::default()).unwrap();
            for linear in [false, true] {
                let image = decode(&encoded, linear).unwrap();
                assert!(image.is_gray());
                assert_eq!(image.data(), values);
                assert!(!image.integer_sampling());
            }
        }
    }
}

#[test]
fn common_bitmap_formats_decode() {
    let pixels = Image::filled(4, 4, Kind::Color, [0.25, 0.5, 0.75, 1.0]);
    for extension in ["png", "jpeg", "tga", "bmp", "webp", "exr", "hdr"] {
        let bytes = encode(&pixels, extension, Depth::U8, true).unwrap();
        let image = decode(&bytes, false).unwrap();

        assert_eq!((image.width(), image.height()), (4, 4), "{extension}");
    }

    // GIF and TIFF import only, so another writer makes them
    for format in [image::ImageFormat::Gif, image::ImageFormat::Tiff] {
        let mut bytes = Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 255])).write_to(&mut bytes, format).unwrap();

        assert_eq!(decode(bytes.get_ref(), false).unwrap().width(), 2, "{format:?}");
    }
    assert!(encode(&pixels, "tif", Depth::U8, true).is_err(), "TIFF is not an export format");

    for pgm in [&b"P2\n2 1\n255\n0 255\n"[..], &b"P5\n2 1\n255\n\0\xff"[..]] {
        let image = decode(pgm, false).unwrap();

        assert_eq!(image.kind(), Kind::Gray);
        assert_eq!(image.data(), &[0.0, 1.0]);
    }
}

#[test]
fn dds_supports_compressed_uncompressed_and_float_textures() {
    for format in [
        dds::Format::BC1_UNORM,
        dds::Format::BC3_UNORM,
        dds::Format::BC5_UNORM,
        dds::Format::BC7_UNORM,
        dds::Format::R8G8B8A8_UNORM,
        dds::Format::R32G32B32A32_FLOAT,
    ] {
        let mut bytes = Vec::new();
        let header = dds::header::Header::new_image(4, 4, format);
        let mut encoder = dds::Encoder::new(&mut bytes, format, &header).unwrap();
        let pixels = [64, 128, 192, 255].repeat(16);
        encoder.write_surface(dds::ImageView::new(&pixels, dds::Size::new(4, 4), dds::ColorFormat::RGBA_U8).unwrap()).unwrap();
        encoder.finish().unwrap();
        let image = decode(&bytes, true).unwrap();

        assert_eq!((image.width(), image.height()), (4, 4));
        assert!(image.data().iter().all(|value| value.is_finite()));
        assert!((image.data()[0] - 64.0 / 255.0).abs() < 0.05, "{format:?}");

        if format == dds::Format::R32G32B32A32_FLOAT {
            assert_eq!(decode(&bytes, false).unwrap().data(), image.data(), "float DDS stays linear");
        }

        assert!(decode(&bytes[..bytes.len() / 2], true).is_err());
    }
}

#[test]
fn shared_dds_keeps_rgb_transfer_alpha_and_float_range() {
    let mut bytes = Vec::new();
    dds::header::Header::new_image(1, 1, dds::Format::R8G8B8A8_UNORM).write(&mut bytes).unwrap();
    bytes.extend_from_slice(&[128, 64, 255, 32]);
    let raw = decode(&bytes, true).unwrap();
    let converted = decode(&bytes, false).unwrap();
    for i in 0..3 {
        assert!((converted.data()[i] - craft_texture::srgb_to_linear(raw.data()[i])).abs() < 1e-6);
    }
    assert_eq!(converted.data()[3], raw.data()[3]);

    let mut floats = Vec::new();
    dds::header::Header::new_image(1, 1, dds::Format::R32G32B32A32_FLOAT).write(&mut floats).unwrap();
    for v in [-0.5_f32, 2.0, 0.25, 0.75] {
        floats.extend_from_slice(&v.to_le_bytes());
    }
    for linear in [false, true] {
        assert_eq!(decode(&floats, linear).unwrap().data(), &[-0.5, 2.0, 0.25, 0.75]);
    }
}
