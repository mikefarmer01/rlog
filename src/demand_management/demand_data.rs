use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DemandData {
    pub demands: Vec<f32>,
    pub demands_estimated: Vec<f32>,
    pub periods: Vec<i32>
}
impl DemandData {
    pub fn new() -> Self {
        Self {
            demands: Vec::default(),
            demands_estimated: Vec::default(),
            periods: Vec::default()
        }
    }
}
