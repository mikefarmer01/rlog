mod utils;
mod dm_mgmt;
mod dm_mgmt_normal;
mod plotting;

use wasm_bindgen::prelude::*;

use crate::dm_mgmt_normal::NormalDemandManagement;
use dm_mgmt::DemandData;

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
    let alpha = alpha;
    let mut dm = NormalDemandManagement::new(mean, std_dev, alpha);
    
    dm.period_zero();
    dm.run_periods(n-1);

    let dd: DemandData = dm.demand_data;
    let ds = dd.demands.clone();
    plotting::plot(ds.into_iter().enumerate().map(|(a,b)| (a as i32, b)).collect());
    JsValue::from_serde(&dd).unwrap()
}
