use image::ImageFormat;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum WasmImageFormat {
    /// An Image in PNG Format
    Png,

    /// An Image in JPEG Format
    Jpeg,

    /// An Image in GIF Format
    Gif,

    /// An Image in WEBP Format
    WebP,

    /// An Image in TIFF Format
    Tiff,

    /// An Image in BMP Format
    Bmp,

    /// An Image in ICO Format
    Ico,
}

#[wasm_bindgen(js_name = "isFormatSupported")]
pub fn is_format_supported(ext: &str) -> bool {
    let format = match ImageFormat::from_extension(ext) {
        Some(f) => f,
        None => return false,
    };
    match format {
        ImageFormat::Png
        | ImageFormat::Jpeg
        | ImageFormat::Gif
        | ImageFormat::WebP
        | ImageFormat::Tiff
        | ImageFormat::Bmp
        | ImageFormat::Ico => true,
        _ => false,
    }
}
