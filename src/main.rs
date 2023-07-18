mod types;
mod integrators;
mod forces;
mod utility;
mod thermostats;
mod potentials;
mod xyz;
mod thermo;
mod pbc;

use std::sync::mpsc::channel;
use types::Particle;
use rayon::prelude::*;
use rand::prelude::*;
use cgmath::{Point3, Vector3, InnerSpace, Zero};

use crate::forces::compute_forces;
use crate::integrators::Integrator;
use crate::thermo::temperature;
use crate::thermostats::{Andersen, Thermostat};
use crate::xyz::XYZWriter;

const N_PARTICLES: usize = 1000;
const N_STEPS: u32 = 10_000;
const BOX_SIZE: f64 = 20.0;  // σ
const TEMP: f64 = 0.5;       // ε/k_B
const TIMESTEP: f64 = 0.01; // τ
const CUTOFF: f64 = 2.5;     // σ
const LIMIT_TIMESTEPS: u32 = 200;
const LIMIT_SPEED: f64 = 1.0;
const DUMP_INTERVAL: u32 = 10;
const TRAJECTORY_PATH: &str = "traj.xyz.gz";
const DAMPING: f64 = 10.0;

type PP = potentials::LJ; // Pair potential

fn main() {
    // Initialize particle positions and velocities
    let u = rand_distr::Uniform::new(0.0, BOX_SIZE);
    let n = rand_distr::Normal::new(0.0, TEMP.sqrt()).unwrap();

    let mut particles: Vec<Particle> = (0..N_PARTICLES)
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, _| {
            // Give particles random (uniformly distributed) positions and (Gaussian distributed) velocities
            let position = Point3::new(u.sample(rng), u.sample(rng), u.sample(rng));
            let velocity = Vector3::new(n.sample(rng), n.sample(rng), n.sample(rng));

            Particle {
                old_position: position,
                position: position + velocity * TIMESTEP,
                velocity: velocity,
                force: Vector3::zero(),
            }
        })
        .collect();

    let thermostat = Andersen::new(TEMP);
    let integrator = integrators::VelocityVerlet;
    //let integrator = integrators::Langevin::new(DAMPING, TEMP);
    let mut xyz_writer = XYZWriter::new(TRAJECTORY_PATH);

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();

    // Main MD loop
    for i in 0..N_STEPS {
        particles
            .par_iter_mut()
            .for_each(|p| {
                integrator.integrate_a(p);
            });

        let potential = compute_forces::<PP>(&mut particles);

        // Loop over particles, integrating equations of motion and computing kinetic energy
        let kinetic: f64 = particles
            .par_iter_mut()
            .map(|p| {
                integrator.integrate_b(p, if i < LIMIT_TIMESTEPS {Some(LIMIT_SPEED)} else {None});
                thermostat.run(p);
                0.5*p.velocity.magnitude2()
            })
            .sum();

        if i % DUMP_INTERVAL == 0 {
            println!("Timestep {}, E={}, E_kin={}, E_pot={}, T={}", i, potential + kinetic, kinetic,
                     potential, temperature(kinetic, particles.len()));
            xyz_writer.write_frame(&particles);
        }

        // Check for SIGINT
        if rx.try_recv().is_ok() {
            println!("Terminating...");
            drop(xyz_writer);
            break;
        }
    }
}
