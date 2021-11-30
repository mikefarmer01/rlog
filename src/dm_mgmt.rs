use rand::distributions::Distribution;
use rand::prelude::{thread_rng, ThreadRng};

use serde::{Deserialize, Serialize};

pub struct DemandManagement<T: Distribution<f64>> {
    pub demands: Vec<f32>,
    pub demands_estimated: Vec<f32>,
    pub distr: T,
    pub alpha: f32,
    pub rng: ThreadRng,
}

#[derive(Serialize, Deserialize)]
pub struct Period {
    ind: usize,
    demand: f32,
    demand_estimated: f32,
}

impl<T: Distribution<f64>> DemandManagement<T> {
    pub fn new(distr: T, alpha: f32) -> DemandManagement<T>{
        DemandManagement::<T> {
            demands : Vec::<f32>::new(),
            demands_estimated : Vec::<f32>::new(),
            distr,
            alpha,
            rng : thread_rng(),
        }
    }
    pub fn period_zero(&mut self) {
        self.make_demand();
        self.demands_estimated
            .push(self.demands.last().cloned().unwrap());
    }
    pub fn run_periods(&mut self, n: i32) {
        for _i in 0..n {
            self.run_period();
        }
    }
    fn run_period(&mut self) {
        self.make_demand();
        self.estimate_demand();
    }
    fn make_demand(&mut self) {
        let demand: f32 = self.distr.sample(&mut self.rng).round() as f32;
        self.demands.push(demand);
    }
    fn estimate_demand(&mut self) {
        let demand_smoothed = self.smooth_exponentially();
        self.demands_estimated.push(demand_smoothed);
    }

    fn smooth_exponentially(&self) -> f32 {
        let demand: f32 = self.demands.last().cloned().unwrap();
        let last_demand_estimated: f32 = self.demands_estimated.last().cloned().unwrap();
        let demand_estimated: f32 =
            self.alpha * demand + (1.0 - self.alpha) * last_demand_estimated;
        return demand_estimated;
    }

    pub fn to_periods(&self) -> Vec<Period> {
        let d = &self.demands;
        let e = &self.demands_estimated;

        let periods: Vec<Period> = d
            .into_iter()
            .enumerate()
            .zip(e.into_iter())
            .map(|((a, b), c)| Period {
                ind: a,
                demand: *b,
                demand_estimated: *c,
            })
            .collect();
        return periods;
    }
}
