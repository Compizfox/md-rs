use crate::{BOX_SIZE, TIMESTEP};
use crate::types::Particle;
use crate::utility::image;
use cgmath::prelude::*;
use cgmath::{Point3, Vector3};

/// velocity Verlet integration
///
pub fn velocity_verlet(p: &mut Particle) {
    p.position = p.position + p.velocity * TIMESTEP + 0.5 * p.force * TIMESTEP*TIMESTEP;
}

/// Störmer–Verlet integration
/// * `p` - Particle to integrate
/// * `limit` - Optional speed limit
pub fn verlet(p: &mut Particle, limit: Option<f64>) {
    let mut new_position: Point3<f64> =
        EuclideanSpace::from_vec(2.0*p.position - p.old_position + p.force*TIMESTEP*TIMESTEP);
    p.velocity = (new_position - p.old_position) / (2.0*TIMESTEP);

    // Limit velocity
    if limit.is_some() {
        let speed= limit.unwrap();

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
