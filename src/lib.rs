mod utils;
mod log;

use wasm_bindgen::prelude::*;

use crate::log::{DemandManagement, run};
use statrs::distribution::Normal;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rlog!");
}

#[wasm_bindgen]
pub fn demand() -> JsValue{
    let normal_distr = Normal::new(30.0, 2.0).unwrap();
    let alpha = 0.1;
    let dm: DemandManagement = run(&normal_distr, alpha);
    let periods = dm.to_periods();

    JsValue::from_serde(&periods).unwrap()
}
