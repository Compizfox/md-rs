use cgmath::prelude::*;

use crate::integrators::Integrator;
use crate::pbc::image;
use crate::{BOX_SIZE, TIMESTEP};
use crate::types::Particle;

/// Velocity Verlet integrator
pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn integrate_a(&self, p: &mut Particle) {
        p.velocity += 0.5 * p.force * TIMESTEP;
        p.position += p.velocity * TIMESTEP;
    }

    fn integrate_b(&self, p: &mut Particle, limit: Option<f64>) {
        p.velocity += 0.5 * p.force * TIMESTEP;

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
