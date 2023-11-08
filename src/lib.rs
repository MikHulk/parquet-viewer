use polars::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Element;

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
                "b" => &[6.0, 7.1, 8.2, 9.3, 0.4],
            )
            .unwrap(),
        }
    }

    pub fn to_html(&self) -> Result<Element, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let html_table = document.create_element("table")?;
        let thead = document.create_element("thead")?;
        html_table.append_child(&thead)?;
        let tr = document.create_element("tr")?;
        thead.append_child(&tr)?;
        for col_name in self.data.get_column_names().iter() {
            let th = document.create_element("th")?;
            th.set_text_content(Some(col_name));
            tr.append_child(&th)?;
        }
        html_table.set_attribute("id", "pl-series")?;
        // for s in self.data.iter() {
        for idx in 0..self.data.height() {
            let row = self.data.get_row(idx).unwrap();
            let tr = document.create_element("tr")?;
            html_table.append_child(&tr)?;
            for val in row.0.iter() {
                let td = document.create_element("td")?;
                tr.append_child(&td)?;
                td.set_text_content(Some(&format!("{}", val)));
            }
        }
        Ok(html_table)
    }
}

impl Container {
    pub fn to_str(&self) -> String {
        format!("{}", self.data)
    }

    pub fn get_data(&self) -> &DataFrame {
        &self.data
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
