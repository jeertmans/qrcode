mod utils;

use image::Luma;
use qrcode::QrCode;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
struct QRCode {
    inner: QrCode,
}

#[wasm_bindgen]
impl QRCode {
    pub fn new() -> Self {
        Self {
            inner: QrCode::new(b"1234").unwrap(),
        }
    }

    pub fn render(&self) -> String {
        self.inner.render().light_color('◻').dark_color('◼').quiet_zone(false).build()
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // Encode some data into bits.
    let code = QrCode::new(b"1234").unwrap();

    alert(&format!("Hello, {}!", name));
}
