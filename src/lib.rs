mod utils;
mod demand_data;
mod demand_management;
mod demand_management_normal;
mod demand_generator;
mod demand_predictor;
mod plotting;
mod types;

use utils::to_rgb;
use wasm_bindgen::prelude::*;
use crate::demand_management_normal::NormalDemandManagement;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IDemandData")]
    pub type DemandData;
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn smooth(mean: f64, std_dev: f64, alpha: f32, n: i32) -> JsValue{
    let alpha = alpha;
    let mut dm = NormalDemandManagement::new(mean, std_dev, alpha);
    
    dm.run_periods(n);
    
    let demand_data = dm.demand_data;
    JsValue::from_serde(&demand_data).unwrap()
}

#[wasm_bindgen]
pub fn resmooth(alpha: f32, demands: &JsValue) -> JsValue {
    let vec_demands = demands.into_serde().expect("Invalid demands data.");
    let mut dm = NormalDemandManagement::load(alpha, vec_demands);
    let demand_data = dm.smooth();
    JsValue::from_serde(demand_data).unwrap()
}

#[wasm_bindgen]
pub fn plot(periods_demands: &JsValue, canvas_id: &JsValue, line_color: &JsValue) {
    let vec_periods_demands: Vec<f32> = periods_demands.into_serde().expect("Invalid demand data.");
    let str_canvas_id: String = canvas_id.into_serde().expect("Invalid canvas id.");
    let color = to_rgb(line_color.into_serde().expect("Invalid color."));
    plotting::plot(&str_canvas_id, &vec_periods_demands, color);
}

#[wasm_bindgen]
pub fn clear(canvas_id: &JsValue) {
    let str_canvas_id: String = canvas_id.into_serde().expect("Invalid canvas id.");
    plotting::clear(&str_canvas_id);
}