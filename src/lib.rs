mod utils;
mod demand_data;
mod demand_management;
mod demand_management_normal;
mod demand_generator;
mod demand_predictor;
mod plotting;

use wasm_bindgen::prelude::*;
use crate::demand_management_normal::NormalDemandManagement;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn smooth(mean: f64, std_dev: f64, alpha: f32, n: i32) -> JsValue{
    let alpha = alpha;
    let mut dm = NormalDemandManagement::new(mean, std_dev, alpha);
    
    dm.run_periods(n);
    
    let demand_data = dm.demand_data;
    JsValue::from_serde(&demand_data).unwrap()
}

#[wasm_bindgen]
pub fn plot(periods_demands: &JsValue) -> () {
    let vec_periods_demands: Vec<f32> = periods_demands.into_serde().unwrap();
    plotting::plot(&vec_periods_demands);
}
