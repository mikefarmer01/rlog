use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Period {
    pub ind: i32,
    pub demand: f32,
    pub demand_estimated: f32,
}

#[derive(Serialize, Deserialize)]
pub struct DemandData {
    pub periods: Vec<Period>,
    pub demands: Vec<f32>,
    pub demands_estimated: Vec<f32>,
}
impl DemandData {
    pub fn new() -> Self {
        Self {
            periods: Vec::<Period>::new(),
            demands: Vec::<f32>::new(),
            demands_estimated: Vec::<f32>::new(),
        }
    }
    pub fn to_periods(&mut self) {
        let d = &self.demands;
        let e = &self.demands_estimated;

        let periods: Vec<Period> = d
            .into_iter()
            .enumerate()
            .zip(e.into_iter())
            .map(|((a, b), c)| Period {
                ind: a as i32,
                demand: *b,
                demand_estimated: *c,
            })
            .collect();
        self.periods = periods;
    }
}
