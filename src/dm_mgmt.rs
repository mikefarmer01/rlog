use rand::distributions::Distribution;
use rand::prelude::{thread_rng, ThreadRng};

use serde::{Deserialize, Serialize};

pub struct DemandManagement<T: Distribution<f64>> {
    pub demand_data: DemandData,
    pub distr: T,
    pub alpha: f32,
    pub rng: ThreadRng,
}

#[derive(Serialize, Deserialize)]
pub struct Period {
    ind: i32,
    demand: f32,
    demand_estimated: f32
}

#[derive(Serialize, Deserialize)]
pub struct DemandData {
    periods: Vec<Period>,
    demands: Vec<f32>,
    demands_estimated: Vec<f32>
}
impl DemandData {
    pub fn to_periods(&mut self){
        let d = &self.demands;
        let e = &self.demands_estimated;
        
        let periods : Vec<Period> = d
            .into_iter().enumerate()
            .zip(e.into_iter())
            .map(|((a, b), c)| Period {
                ind: a as i32,
                demand: *b,
                demand_estimated: *c
            })
            .collect();
        self.periods = periods;
    }
}

impl<T: Distribution<f64>> DemandManagement<T> {
    pub fn new(distr: T, alpha: f32) -> DemandManagement< T> {
        DemandManagement::<T> {
            demand_data: DemandData {
                periods: Vec::<Period>::new(),
                demands: Vec::<f32>::new(),
                demands_estimated: Vec::<f32>::new(),
            },
            distr,
            alpha,
            rng: thread_rng(),
        }
    }
    pub fn period_zero(&mut self) {
        self.make_demand();
        self.demand_data.demands_estimated
            .push(self.demand_data.demands.last().cloned().unwrap());
    }
    pub fn run_periods(&mut self, n: i32) {
        for _i in 0..n {
            self.run_period();
        }
        //Expend memory in order to take a different perspective on the demand data without needing to fiddle around with references/lifetimes/etc.
        self.demand_data.to_periods();
    }
    fn run_period(&mut self) {
        self.make_demand();
        self.estimate_demand();
    }
    fn make_demand(&mut self) {
        let demand: f32 = self.distr.sample(&mut self.rng).round() as f32;
        self.demand_data.demands.push(demand);
    }
    fn estimate_demand(&mut self) {
        let demand_smoothed = self.smooth_exponentially();
        self.demand_data.demands_estimated.push(demand_smoothed);
    }

    fn smooth_exponentially(&self) -> f32 {
        let demand: f32 = self.demand_data.demands.last().cloned().unwrap();
        let last_demand_estimated: f32 = self.demand_data.demands_estimated.last().cloned().unwrap();
        let demand_estimated: f32 =
            self.alpha * demand + (1.0 - self.alpha) * last_demand_estimated;
        return demand_estimated;
    }
}
