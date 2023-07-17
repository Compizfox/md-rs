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
use rayon::prelude::*;
use rand::prelude::*;
use cgmath::{Point3, Vector3, InnerSpace, Zero, EuclideanSpace};

use crate::forces::compute_forces;
use crate::integrators::Integrator;
use crate::pbc::minimum_image;
use crate::thermo::temperature;
use crate::thermostats::{Andersen, Thermostat};
use crate::types::{Particle, FixedParticle};
use crate::xyz::XYZWriter;

const N_PARTICLES: usize = 1000;
const N_STEPS: u32 = 1000_000;
const BOX_SIZE: f64 = 75.0;  // σ
const TEMP: f64 = 10.0;       // ε/k_B
const TIMESTEP: f64 = 0.025; // τ
const CUTOFF: f64 = 2.5;     // σ
const LIMIT_TIMESTEPS: u32 = 100;
const LIMIT_SPEED: f64 = 1.0;
const DUMP_INTERVAL: u32 = 1000;
const TRAJECTORY_PATH: &str = "traj.xyz.gz";
const DAMPING: f64 = 10.0;

type PP = potentials::LJ; // Pair potential

fn main() {
    // Initialize particle positions and velocities
    let u = rand_distr::Uniform::new(0.0, BOX_SIZE);
    let n = rand_distr::Normal::new(0.0, TEMP.sqrt()).unwrap();

    let mut particles: Vec<Particle> = (0..N_PARTICLES)
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, _| new_particle(u, n, rng))
        .collect();

    // Create seed particle
    let p = Point3::new(BOX_SIZE * 0.5, BOX_SIZE * 0.5, BOX_SIZE * 0.5);
    let mut fixed_particles: Vec<_> = vec![FixedParticle{position: p, timestep: 0}];

    //let thermostat = Andersen::new(TEMP);
    //let integrator = integrators::VelocityVerlet;
    let integrator = integrators::Langevin::new(DAMPING, TEMP);
    let mut xyz_writer = XYZWriter::new(TRAJECTORY_PATH);
    let mut xyz_writer_fixed = XYZWriter::new("fixed.xyz.gz");

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
                //thermostat.run(p);
                0.5*p.velocity.magnitude2()
            })
            .sum();

        // Find particles to glue:
        let new_fixed_particles = particles
            .par_iter()
            .enumerate()
            .filter_map(|(i, p)| {
                for f_p in &fixed_particles {
                    let mut dr = p.position - f_p.position;
                    dr -= BOX_SIZE * minimum_image(dr, BOX_SIZE);

                    if dr.magnitude2() < 1.0 {
                        return Some(i);
                    }
                }
                None
            })
            .collect::<Vec<usize>>();

        // Add new walking particles
        fixed_particles.extend(new_fixed_particles
            .into_iter()
            .map(|i_f_p| {
                // Clone particle into fixed particle, and randomise the original particle
                let p = particles[i_f_p].position;
                particles[i_f_p] = new_particle(u, n, &mut thread_rng());
                FixedParticle{position: p, timestep: i}
            })
            .collect::<Vec<FixedParticle>>()
        );

        if i % DUMP_INTERVAL == 0 {
            println!("Timestep {:.3}, E={:.3}, E_kin={:.3}, E_pot={:.3}, T={:.3}, n_particles={}, n_fixed={}", i,
                     potential + kinetic, kinetic, potential,
                     temperature(kinetic,particles.len()), particles.len(),
                     fixed_particles.len());
            xyz_writer.write_frame(&particles);
            xyz_writer_fixed.write_frame(&fixed_particles);
        }

        // Check for SIGINT
        if rx.try_recv().is_ok() {
            println!("Terminating...");
            drop(xyz_writer);
            break;
        }
    }
}

fn new_particle(u: rand_distr::Uniform<f64>, n: rand_distr::Normal<f64>, rng: &mut ThreadRng) -> Particle {
    // Give particles random (uniformly distributed) positions and (Gaussian distributed) velocities
    let position = Point3::new(u.sample(rng), u.sample(rng), u.sample(rng));
    let velocity = Vector3::new(n.sample(rng), n.sample(rng), n.sample(rng));

    Particle {
        old_position: position,
        position: position + velocity * TIMESTEP,
        velocity: velocity,
        force: Vector3::zero(),
    }
}
