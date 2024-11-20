mod format;

use std::io::Cursor;

use image::{load_from_memory, ImageFormat};
use wasm_bindgen::prelude::*;
use image::DynamicImage;
use crate::format::WasmImageFormat;

#[wasm_bindgen]
pub struct WasmImage {
    instance: DynamicImage,
}


#[wasm_bindgen]
impl WasmImage {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<WasmImage, JsValue> {
        let instance = load_from_memory(data).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        Ok(Self { instance })
    }
} 

#[wasm_bindgen]
impl WasmImage {
    #[wasm_bindgen(js_name = "toFormat")]
    pub fn to_format(&self, format: WasmImageFormat) -> Vec<u8> {
        let image_format = match format {
            WasmImageFormat::Png => ImageFormat::Png,
            WasmImageFormat::Jpeg => ImageFormat::Jpeg,
            WasmImageFormat::Gif => ImageFormat::Gif,
            WasmImageFormat::WebP => ImageFormat::WebP,
            WasmImageFormat::Tiff => ImageFormat::Tiff,
            WasmImageFormat::Bmp => ImageFormat::Bmp,
            WasmImageFormat::Ico => ImageFormat::Ico,
        };

        let mut buffer = Cursor::new(Vec::new());

        self.instance.write_to(&mut buffer, image_format)
            .expect("Failed to write image");

        buffer.into_inner()
    }
}