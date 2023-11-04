use wasm_bindgen::prelude::*;
use web_sys::Element;
use polars::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Container {
    data: Series,
}

#[wasm_bindgen]
impl Container {
    pub fn new() -> Self {
        Self { data: Series::new("a", [1, 2, 3, 4, 5]) }
    }

    pub fn to_html(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let html_list = document.create_element("ul")?;
        html_list.set_attribute("id", "pl-series")?;
        for i in self.data.iter() {
            let item = document.create_element("li")?;
            item.set_text_content(Some(&format!("{}", i)));
            html_list.append_child(&item)?;
        }
        Ok(html_list)
    }
}

#[wasm_bindgen]
pub fn get_content(name: &str) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let html_node = document.create_element("p")?;
    html_node.set_text_content(Some(&format!("Hello {} from Rust!", name)));
    html_node.set_attribute("id", "greeting")?;
    Ok(html_node)
}
