mod dm_mgmt;
mod utils;

use wasm_bindgen::prelude::*;

use crate::dm_mgmt::DemandManagement;
use dm_mgmt::DemandData;
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
pub fn smooth(mean: f64, std_dev: f64, alpha: f32, n: i32) -> JsValue{
    let normal_distr = Normal::new(mean, std_dev).unwrap();
    let alpha = alpha;
    let mut dm: DemandManagement<Normal> = DemandManagement::new(normal_distr, alpha);
    
    dm.period_zero();
    dm.run_periods(n-1);

    let dd: DemandData = dm.demand_data;
    JsValue::from_serde(&dd).unwrap()
}
