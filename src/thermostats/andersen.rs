use cgmath::prelude::*;
use rand::prelude::*;

use crate::thermostats::Thermostat;
use crate::TIMESTEP;
use crate::types::Particle;
use crate::Vector;

pub struct Andersen {
    n: rand_distr::Normal<f64>,
}

impl Andersen {
    pub fn new(temperature: f64) -> Self {
        Self{
            n: rand_distr::Normal::new(0.0, temperature.sqrt()).unwrap()
        }
    }
}

impl Thermostat for Andersen {
    fn run(&self, p: &mut Particle) {
        let mut rng = thread_rng();
        if rng.gen_range(0.0..1.0) < TIMESTEP {
            p.velocity = Vector::zero().map(|_| self.n.sample(&mut rng))
        }
    }
}

