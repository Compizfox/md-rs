use cgmath::{Vector3, InnerSpace, Zero};
use rayon::prelude::*;

use crate::{N_PARTICLES, BOX_SIZE, CUTOFF, Particle};
use crate::pbc::minimum_image;
use crate::utility::ThreadLocalVec;
use crate::potentials::PairPotential;

/// Computes pair interactions: set forces in Particle.
/// Returns total potential energy.
/// * `particles` - Slice of particles
pub fn compute_forces<P: PairPotential>(particles: &mut[Particle]) -> f64 {
    // Setup thread-local force arrays
    let tls: ThreadLocalVec<Vector3<f64>, N_PARTICLES> = ThreadLocalVec::new(Vector3::zero());

    // Loop over all unique pairs of particles
    let potential: f64 = particles
        .par_iter()
        .enumerate()
        .map(|(i, p1)| {
            // Thread-local force array
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

    // Sum thread-local arrays element-wise
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
    use cgmath::{Point3, Vector3, InnerSpace, assert_abs_diff_eq, EuclideanSpace, Zero, Array};

    use crate::Particle;
    use crate::forces::compute_forces;
    use crate::potentials::LJ;
    use crate::potentials::PairPotential;

    #[test]
    fn force() {
        let mut particles = vec![
            Particle {
                old_position: Point3::origin(),
                position: Point3::origin(),
                velocity: Vector3::zero(),
                force: Vector3::zero(),
            },
            Particle {
                old_position: Point3::from_value(1.0),
                position: Point3::from_value(1.0),
                velocity: Vector3::zero(),
                force: Vector3::zero(),
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
