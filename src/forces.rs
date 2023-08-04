use cgmath::prelude::*;
use rayon::prelude::*;

use crate::{BOX_SIZE, CUTOFF, Particle};
use crate::pbc::minimum_image;
use crate::utility::ThreadLocalVec;
use crate::potentials::PairPotential;
use crate::Vector;

/// Computes pair interactions: set forces in Particle.
/// Returns total potential energy.
/// * `particles` - Slice of particles
pub fn compute_forces<P: PairPotential>(particles: &mut[Particle]) -> f64 {
    // Setup thread-local force vectors
    let tls: ThreadLocalVec<Vector> = ThreadLocalVec::new(Vector::zero(), particles.len());

    // Loop over all unique pairs of particles
    let potential: f64 = particles
        .par_iter()
        .enumerate()
        .map(|(i, p1)| {
            // Thread-local force vector
            let mut force = tls.borrow_mut();

            // Temporary potential
            let mut potential: f64 = 0.0;

            for (j, p2) in particles[..i].iter().enumerate() {
                let mut dr = p1.position - p2.position;

                // Apply minimum image convention for PBCs
                dr -= BOX_SIZE * minimum_image(dr, BOX_SIZE);

                let r2: f64 = dr.magnitude2();

                if r2 < CUTOFF * CUTOFF {
                    // F(r) * dr/|dr|
                    let dr_magnitude = dr.magnitude();
                    let f = dr.normalize_to(P::force(dr_magnitude));
                    force[i] -= f;
                    force[j] += f;

                    potential += P::energy(dr_magnitude);
                };
            }
            potential
        })
        .sum();

    // Sum thread-local vectors element-wise
    let forces = tls.into_sum();

    // Put forces in particle structs
    particles
        .par_iter_mut()
        .zip(forces)
        .for_each(|(p, force)| {
            p.force = force;
        });

    potential
}

#[cfg(test)]
mod tests {
    use cgmath::prelude::*;
    use cgmath::assert_abs_diff_eq;

    use crate::Particle;
    use crate::forces::compute_forces;
    use crate::potentials::LJ;
    use crate::potentials::PairPotential;
    use crate::{Vector, Point};

    #[test]
    fn force() {
        let mut particles = vec![
            Particle {
                old_position: Point::origin(),
                position: Point::origin(),
                velocity: Vector::zero(),
                force: Vector::zero(),
            },
            Particle {
                old_position: Point::from_value(1.0),
                position: Point::from_value(1.0),
                velocity: Vector::zero(),
                force: Vector::zero(),
            },
        ];

        compute_forces::<LJ>(&mut particles);
        let dr = particles[0].position - particles[1].position;

        // Newton's third law
        assert_eq!(particles[0].force, -particles[1].force);

        // Check force value
        assert_abs_diff_eq!(particles[0].force, -LJ::force(dr.magnitude())*dr.normalize());
    }
}
