use rand::prelude::{ThreadRng, thread_rng};
use rand::distributions::Distribution;

use serde::{Serialize, Deserialize};

pub fn run(distr: &impl Distribution<f64>, alpha: f32) -> DemandManagement{
    let mut dm: DemandManagement = init(distr);
    for _i in 0..100{
        dm = period(dm, distr, alpha);
    }
    return dm;
}

fn period(mut dm: DemandManagement, distr: &impl Distribution<f64>, alpha: f32) -> DemandManagement{
    make_demand(&mut dm.demands, distr);
    estimate_demand(&mut dm, alpha);
    return dm;
}

fn make_demand(demands: &mut Vec<f32>, distr: &impl Distribution<f64>){
    let mut rng: ThreadRng = thread_rng();
    let demand: f32 = distr.sample(&mut rng).round() as f32;
    demands.push(demand);
}

fn estimate_demand(dm: &mut DemandManagement, alpha: f32){
    let demand_smoothed = smooth_exponentially(dm, alpha);
    dm.demands_estimated.push(demand_smoothed);
}

fn smooth_exponentially(dm: &mut DemandManagement, alpha: f32) -> f32{
    let demand: f32 = dm.demands.last().cloned().unwrap();
    let last_demand_estimated: f32 = dm.demands_estimated.last().cloned().unwrap();
    let demand_estimated: f32  = alpha * demand + (1.0-alpha) * last_demand_estimated;
    return demand_estimated;
}

fn init(distr: &impl Distribution<f64>) -> DemandManagement{
    let mut dm: DemandManagement = DemandManagement {
        demands: Vec::<f32>::new(),
        demands_estimated: Vec::<f32>::new()
    };
    make_demand(&mut dm.demands, distr);
    dm.demands_estimated.push(dm.demands.last().cloned().unwrap());
    return dm;
}

pub struct DemandManagement {
    pub demands: Vec<f32>,
    pub demands_estimated: Vec<f32>,
    
}

#[derive(Serialize, Deserialize)]
pub struct Period{
    ind: usize,
    demand: f32,
    demand_estimated: f32
}

impl DemandManagement {
    pub fn to_periods(&self) -> Vec<Period> {
        let d = &self.demands;
        let e = &self.demands_estimated;
        
        let periods: Vec<Period> = d.into_iter().enumerate().zip(e.into_iter()).map(|((a,b), c)| Period { ind: a, demand: *b, demand_estimated: *c}).collect();
        return periods;
    }
}
