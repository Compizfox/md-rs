use cgmath::prelude::*;
use cgmath::Point3;

use crate::integrators::Integrator;
use crate::utility::image;
use crate::{BOX_SIZE, TIMESTEP};
use crate::types::Particle;

/// Störmer–Verlet integrator
pub struct Verlet;

impl Integrator for Verlet {
    fn integrate_a(_: &mut Particle) {}

    fn integrate_b(p: &mut Particle, limit: Option<f64>) {
        let mut new_position: Point3<f64> = EuclideanSpace::from_vec(2.0*p.position - p.old_position + p.force*TIMESTEP*TIMESTEP);
        p.velocity = (p.position - p.old_position) / (2.0*TIMESTEP);

        // Limit velocity
        if let Some(speed) = limit {
            if p.velocity.magnitude2() > speed*speed {
                p.velocity = p.velocity.normalize_to(speed);
                new_position = p.old_position + 2.0*p.velocity*TIMESTEP;
            }
        }

        // Advance a timestep
        p.old_position = p.position;
        p.position = new_position;

        // Apply PBCs
        let image = image(p.position, BOX_SIZE);
        p.position -= BOX_SIZE*image;
        p.old_position -= BOX_SIZE*image;
    }
}
