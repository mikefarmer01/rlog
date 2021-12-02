pub struct DemandPredictor {
    alpha: f32,
}

impl DemandPredictor {
    pub fn new(alpha: f32) -> Self {
        Self { alpha }
    }
    pub fn smooth_exponentially(&self, demand: &f32, last_demand_estimated: &f32) -> f32 {
        let demand_estimated: f32 =
            self.alpha * demand + (1.0 - self.alpha) * last_demand_estimated;
        return demand_estimated;
    }
}
