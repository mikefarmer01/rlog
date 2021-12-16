use wasm_bindgen::prelude::*;

use crate::demand_management::DemandManagement;
use statrs::distribution::Normal;

#[wasm_bindgen]
pub struct NormalDemandManagement {}

impl NormalDemandManagement {
    pub fn new(mean: f64, std_dev: f64, alpha: f32) -> DemandManagement<Normal> {
        let normal_distr = Normal::new(mean, std_dev).unwrap();
        DemandManagement::new(normal_distr, alpha)
    }
    pub fn load(alpha: f32, demands: Vec<f32>) -> DemandManagement<Normal> {
        let mut dm = NormalDemandManagement::new(30f64, 1f64, alpha);
        dm.demand_data.demands = demands;
        dm.demand_data.demands_estimated.truncate(1);
        dm
    }
}
