use cgmath::prelude::*;
use cgmath::{Vector3};
use rand::prelude::*;

use crate::integrators::Integrator;
use crate::pbc::image;
use crate::{BOX_SIZE, TIMESTEP};
use crate::types::Particle;

/// Langevin dynamics integrator
pub struct Langevin {
    n: rand_distr::StandardNormal,
    y: f64,
    T: f64,
}

impl Langevin {
    /// Constructs a new `Langevin` integrator.
    /// * `gamma` - damping constant
    /// * `temperature` - temperature
    pub fn new(gamma: f64, temperature: f64) -> Self {
        Self{
            n: rand_distr::StandardNormal,
            y: gamma,
            T: temperature,
        }
    }
}

impl Integrator for Langevin {
    fn integrate_a(&self, p: &mut Particle) {
        let mut rng = thread_rng();
        let a = Vector3::new(
            self.n.sample(&mut rng),
            self.n.sample(&mut rng),
            self.n.sample(&mut rng)
        );
        p.velocity += 0.5 * p.force * TIMESTEP + (self.y * self.T * TIMESTEP).sqrt() * a;
        p.position += p.velocity * (2.0*TIMESTEP) / (2.0 + self.y * TIMESTEP);
    }

    fn integrate_b(&self, p: &mut Particle, limit: Option<f64>) {
        let mut rng = thread_rng();
        let a = Vector3::new(
            self.n.sample(&mut rng),
            self.n.sample(&mut rng),
            self.n.sample(&mut rng)
        );

        p.velocity = (2.0 - self.y*TIMESTEP) / (2.0 + self.y * TIMESTEP) * p.velocity +
                     (self.y * self.T * TIMESTEP).sqrt() * a + 0.5 * p.force * TIMESTEP;

        // Limit velocity
        if let Some(speed) = limit {
            if p.velocity.magnitude2() > speed*speed {
                p.velocity = p.velocity.normalize_to(speed);
            }
        }

        // Apply PBCs
        p.position -= BOX_SIZE*image(p.position, BOX_SIZE);
    }
}