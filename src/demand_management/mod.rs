mod demand_data;
mod demand_generator;
pub mod demand_management_normal;
mod demand_predictor;

use statrs::statistics::Distribution;

use demand_data::DemandData;
use demand_generator::DemandGenerator;
use demand_predictor::DemandPredictor;

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
    fn period_zero(&mut self) {
        let demand = self.demand_generator.make_demand();
        self.demand_data.periods.push(0);
        self.demand_data.demands.push(demand);
        self.demand_data.demands_estimated.push(demand);
    }
    pub fn run_periods(&mut self, n: i32) {
        self.period_zero();
        for i in 1..n {
            self.run_period(i);
        }
    }
    pub fn smooth(&mut self) -> &DemandData {
        let de = &mut self.demand_data.demands_estimated;
        let ds = &mut self.demand_data.demands;
        let ps = &mut self.demand_data.periods;
        de.push(ds[0]);
        ps.push(0);
        for i in 1..ds.len() {
            ps.push(i as i32);
            let last_demand_estimated = de.get(i-1).cloned().unwrap();
            let demand = ds.get(i).cloned().unwrap();
            de.push(
                self.demand_predictor
                .smooth_exponentially(&demand, &last_demand_estimated)
            )
        }
        return &self.demand_data
    }
    fn run_period(&mut self, i: i32) {
        let last_demand_estimated = self.demand_data.demands_estimated.last().cloned().unwrap();

        let demand = self.demand_generator.make_demand();
        self.demand_data.periods.push(i);
        self.demand_data.demands.push(demand);

        self.demand_data.demands_estimated.push(
            self.demand_predictor
                .smooth_exponentially(&demand, &last_demand_estimated),
        );
    }
}
