mod demand_management;
mod plotting;
mod types;
mod utils;
mod error;

use demand_management::demand_management_normal::NormalDemandManagement;
use error::canvas_clear_error::CanvasClearError;
use utils::to_rgb;
use wasm_bindgen::prelude::*;

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
pub async fn smooth(mean: f64, std_dev: f64, alpha: f32, n: i32) -> Result<JsValue, JsError> {
    let alpha = alpha;
    let mut dm = NormalDemandManagement::new(mean, std_dev, alpha);

    dm.run_periods(n);

    let demand_data = dm.demand_data;
    let js_demand_data = JsValue::from_serde(&demand_data)?;
    Ok(js_demand_data)
}

#[wasm_bindgen]
pub async fn resmooth(alpha: f32, demands: JsValue) -> Result<JsValue, JsError> {
    let vec_demands = demands.into_serde()?;
    let mut dm = NormalDemandManagement::load(alpha, vec_demands);

    let demand_data = dm.smooth();
    let js_demand_data = JsValue::from_serde(demand_data)?;
    Ok(js_demand_data)
}

#[wasm_bindgen]
pub async fn plot(periods_demands: JsValue, canvas_id: JsValue, line_color: JsValue) -> Result<(), JsError>{
    let vec_periods_demands: Vec<f32> = periods_demands.into_serde().expect("Invalid demand data.");
    let str_canvas_id: String = canvas_id.into_serde().expect("Invalid canvas id.");
    let color = to_rgb(line_color.into_serde().expect("Invalid color."));

    plotting::plot(&str_canvas_id, &vec_periods_demands, color)?;
    Ok(())
}

#[wasm_bindgen]
pub async fn clear(canvas_id: JsValue) -> Result<(), JsError>{
    let str_canvas_id: Result<String, serde_json::Error> = canvas_id.into_serde();
    let str_canvas_id = match str_canvas_id {
        Ok(str) => Ok(str),
        Err(_) => Err(CanvasClearError::SerdeError)
    };
    let str_canvas_id = str_canvas_id?;
    plotting::clear(&str_canvas_id)?;
    Ok(())
}
