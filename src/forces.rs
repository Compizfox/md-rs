use std::vec;
use cgmath::{Vector3, InnerSpace, Zero};
use rayon::prelude::*;
use crate::{N_PARTICLES, BOX_SIZE, CUTOFF, Particle};
use crate::utility::add_arrays;
use crate::potentials::PairPotential;

/// Computes pair interactions: set forces in Particle.
/// Returns total potential energy.
/// * `particles` - Slice of particles
pub fn compute_forces<P: PairPotential>(particles: &mut[Particle]) -> f64 {
    // Loop over all unique pairs of particles
    let (potential, forces): (f64, Vec<Vector3<f64>>) = particles
        .par_iter()
        .enumerate()
        .map(| (i, p1) | {
            // Temporary force array
            let mut force = vec![Vector3::zero(); N_PARTICLES];
            // Temporary potential array
            let mut potential: f64 = 0.0;

            for (j, p2) in particles[..i].iter().enumerate() {
                let mut dr = p1.position - p2.position;

                // Apply minimum image convention for PBCs
                let image = Vector3::new(
                    (dr.x / BOX_SIZE).round(),
                    (dr.y / BOX_SIZE).round(),
                    (dr.z / BOX_SIZE).round(),
                );
                dr -= BOX_SIZE * image;

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
            (potential, force)
        })
        // Add potential, and vectors element-wise
        .reduce(|| (0.0, vec![Vector3::zero(); N_PARTICLES]),
                |(pot_a, f_a), (pot_b, f_b)| (pot_a + pot_b, add_arrays(&f_a, &f_b)));

    // Put forces in particle structs
    particles
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, p)| {
            p.force = forces[i];
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
