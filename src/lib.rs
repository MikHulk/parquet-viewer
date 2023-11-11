use polars::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, Element};

macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

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
pub fn set_panic_hook() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    {
        log!("panic hook enabled");
        extern crate console_error_panic_hook;
        use std::panic;
        panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    Ok(())
}

#[wasm_bindgen]
impl Container {
    pub fn new() -> Self {
        Self {
            data: df!(
                "a" => &[1, 2, 3, 4, 5],
                "b" => &[6.0, 7.1, 8.2, 9.3, 0.4],
                "c" => &["un", "deux", "trois", "quatre", "cinq"],
            )
            .unwrap(),
        }
    }

    pub fn to_html(&self, wanted_columns: Vec<String>) -> Result<Element, JsValue> {
        log!("get {:?} columns", wanted_columns);

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let html_table = document.create_element("table")?;
        html_table.set_attribute("id", "pl-series")?;

        let thead = document.create_element("thead")?;
        html_table.append_child(&thead)?;
        let tr = document.create_element("tr")?;
        thead.append_child(&tr)?;
        let data_cols: Vec<&str> = self
            .data
            .get_column_names()
            .into_iter()
            .filter(|col| {
                wanted_columns.iter().take_while(|s| s != col).count() != wanted_columns.len()
            })
            .collect();
        for col_name in &data_cols {
            let th = document.create_element("th")?;
            th.set_text_content(Some(col_name));
            tr.append_child(&th)?;
        }

        // TODO: removes unwrap
        let selection = self.data.select(data_cols).unwrap();
        for idx in 0..selection.height() {
            // TODO: removes unwrap
            let row = selection.get_row(idx).unwrap();
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
