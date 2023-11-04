use wasm_bindgen::prelude::*;
use web_sys::Element;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn get_content(name: &str) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let val = document.create_element("p")?;
    val.set_text_content(Some(&format!("Hello {} from Rust!", name)));

    Ok(val)
}
