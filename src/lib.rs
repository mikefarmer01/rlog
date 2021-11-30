mod log;
mod utils;

use wasm_bindgen::prelude::*;

use crate::log::DemandManagement;
use statrs::distribution::Normal;

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
pub fn greet() {
    alert("Hello, rlog!");
}

#[wasm_bindgen]
pub fn demand() -> JsValue {
    let normal_distr = Normal::new(30.0, 2.0).unwrap();
    let alpha = 0.1;
    let mut dm: DemandManagement<Normal> = DemandManagement {
        demands: Vec::<f32>::new(),
        demands_estimated: Vec::<f32>::new(),
        distr: normal_distr,
        alpha: alpha,
        rng: rand::prelude::thread_rng()
    };
    //dm.init(normal_distr, alpha);
    
    let n = 50;
    dm.period_zero();
    dm.run_periods(n);
    let periods = dm.to_periods();

    JsValue::from_serde(&periods).unwrap()
}
