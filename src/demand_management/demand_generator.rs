use rand::prelude::{thread_rng, ThreadRng};
use statrs::statistics::Distribution;

pub struct DemandGenerator<T: Distribution<f64>> {
    distr: T,
    rng: ThreadRng,
}

impl<T: Distribution<f64>> DemandGenerator<T> {
    pub fn new(distr: T) -> Self {
        Self {
            distr,
            rng: thread_rng(),
        }
    }
    pub fn make_demand(&mut self) -> f32 {
        return self.distr.sample(&mut self.rng).round() as f32;
    }
}
