use statrs::statistics::Distribution;

use crate::demand_data::DemandData;
use crate::demand_generator::DemandGenerator;
use crate::demand_predictor::DemandPredictor;

pub struct DemandManagement<T: Distribution<f64>> {
    pub demand_data: DemandData,
    demand_generator: DemandGenerator<T>,
    demand_predictor: DemandPredictor,
}

impl<T: Distribution<f64>> DemandManagement<T> {
    pub fn new(distr: T, alpha: f32) -> Self {
        Self {
            demand_data: DemandData::new(),
            demand_generator: DemandGenerator::new(distr),
            demand_predictor: DemandPredictor::new(alpha),
        }
    }
    pub fn period_zero(&mut self) {
        let demand = self.demand_generator.make_demand();
        self.demand_data.demands.push(demand);
        self.demand_data.demands_estimated.push(demand);
    }
    pub fn run_periods(&mut self, n: i32) {
        for _i in 0..n {
            self.run_period();
        }
        //Expend memory in order to take a different perspective on the demand data without needing to fiddle around with references/lifetimes/etc.
        self.demand_data.to_periods();
    }
    fn run_period(&mut self) {
        let last_demand_estimated = self.demand_data.demands_estimated.last().cloned().unwrap();

        let demand = self.demand_generator.make_demand();
        self.demand_data.demands.push(demand);

        self.demand_data.demands_estimated.push(
            self.demand_predictor
                .smooth_exponentially(&demand, &last_demand_estimated),
        );
    }
}
