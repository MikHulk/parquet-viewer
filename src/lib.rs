use wasm_bindgen::prelude::*;
use web_sys::Element;
use polars::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Container {
    data: DataFrame,
}

#[wasm_bindgen]
impl Container {
    pub fn new() -> Self {
        Self {
            data: df!(
                "a" => &[1, 2, 3, 4, 5],
                "b" => &[6, 7, 8, 9, 0],
            ).unwrap(),
        }
    }

    pub fn to_str(&self) -> String {
        format!("{}", self.data)
    }

    pub fn to_html(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        Ok(
            match self.data.clone().lazy().select([col("c")]).collect() {
                Err(e) => {
                    let error_msg = document.create_element("p")?;
                    error_msg.set_attribute("id", "pl-series")?;
                    error_msg.set_text_content(Some(&format!("{}", e)));
                    error_msg
                }
                Ok(serie) => {
                    let html_list = document.create_element("ul")?;
                    html_list.set_attribute("id", "pl-series")?;
                    for item in serie.iter() {
                        let li = document.create_element("li")?;
                        li.set_text_content(Some(&format!("{}", item)));
                        html_list.append_child(&li)?;
                    }
                    html_list
                }
            }
        )
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
