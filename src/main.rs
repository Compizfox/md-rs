mod types;
mod integration;
mod forces;
mod utility;
mod thermostat;
mod potential;
mod potentials;

use types::Particle;
use rayon::prelude::*;
use rand::prelude::*;
use cgmath::{Point3, Vector3, InnerSpace};

use crate::forces::{compute_forces};
use crate::integration::verlet;

const N_PARTICLES: usize = 1000;
const N_STEPS: u32 = 1000_000;
const BOX_SIZE: f64 = 20.0;  // σ
const TEMP: f64 = 0.5;       // ε/k_B
const TIMESTEP: f64 = 0.001; // τ
const CUTOFF: f64 = 2.5;     // σ
const LIMIT_TIMESTEPS: u32 = 5000;
const LIMIT_SPEED: f64 = 1.0;

type PP = potentials::LJ; // Pair potential

fn main() {
    // Initialize particle positions and velocities
    let mut particles: Vec<Particle> = (0..N_PARTICLES)
        .into_par_iter()
        .map_init(|| rand::thread_rng(), |rng, i| {
            // Give particles random (uniformly distributed) positions and (Gaussian distributed) velocities
            let u = rand_distr::Uniform::new(0.0, BOX_SIZE);
            let position = Point3 { x: u.sample(rng), y: u.sample(rng), z: u.sample(rng) };
            let n = rand_distr::Normal::new(0.0, TEMP).unwrap();
            let velocity = Vector3 { x: n.sample(rng), y: n.sample(rng), z: n.sample(rng) };

            println!("{}: {:?} {:?}", i, position, velocity);

            Particle {
                old_position: position,
                position: position + velocity * TIMESTEP,
                velocity: velocity,
                force: Vector3::new(0.0, 0.0, 0.0),
            }
        })
        .collect();

    // Main MD loop
    for i in 0..N_STEPS {
        let potential = compute_forces::<PP>(&mut particles);

        // Loop over particles, integrating equations of motion and computing kinetic energy
        let kinetic: f64 = particles
            .par_iter_mut()
            .map(|p| {
                verlet(p, if i < LIMIT_TIMESTEPS {Some(LIMIT_SPEED)} else {None});
                0.5*p.velocity.magnitude2()
            })
            .sum();
        if i % 100 == 0 {
            println!("Timestep {}, E={}, E_kin={}, E_pot={}", i, potential+kinetic, kinetic, potential);
        }
    }
}
