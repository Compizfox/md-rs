mod types;
mod integrators;
mod forces;
mod utility;
mod thermostats;
mod potentials;
mod xyz;
mod thermo;
mod pbc;

use std::time::Instant;
use std::sync::mpsc::channel;
use rayon::prelude::*;
use rand::prelude::*;
use cgmath::prelude::*;

use crate::forces::compute_forces;
use crate::integrators::Integrator;
use crate::pbc::minimum_image;
use crate::thermo::temperature;
use crate::thermostats::Thermostat;
use crate::types::{Particle, FixedParticle};
use crate::xyz::XYZWriter;

const N_PARTICLES: usize = 10;
const N_STEPS: u32 = 10_000_000;
const BOX_SIZE: f64 = 1000.0;  // σ
const TEMP: f64 = 100.0;       // ε/k_B
const TIMESTEP: f64 = 0.1; // τ
const CUTOFF: f64 = 2.5;     // σ
const LIMIT_TIMESTEPS: u32 = 100;
const LIMIT_SPEED: f64 = 1.0;
const DUMP_INTERVAL: u32 = 10_000;
const TRAJECTORY_PATH: &str = "traj.xyz.gz";
const DAMPING: f64 = 1000.0;

const N: u8 = 2; // dimensionality
use cgmath::{Point3, Point2, Vector3, Vector2};
type Vector = Vector2<f64>;
type Point = Point2<f64>;

type PP = potentials::LJ; // Pair potential

// Inner and outer shell radii
fn r1(r_max: f64) -> f64 {
    r_max + 2.0
}
fn r2(r_max: f64) -> f64 {
    r1(r_max) + 2.0
}

fn main() {
    // Initialize particle positions and velocities
    let u = rand_distr::Uniform::new(0.0, BOX_SIZE);
    let n = rand_distr::Normal::new(0.0, TEMP.sqrt()).unwrap();

    let mut particles: Vec<Particle> = (0..N_PARTICLES)
        .into_par_iter()
        .map_init(thread_rng, |rng, _| new_random_particle(u, n, rng))
        .collect();

    // Create seed particle
    let mut fixed_particles: Vec<_> = vec![FixedParticle{
        position: Point::origin().map(|_| BOX_SIZE * 0.5),
        timestep: 0
    }];
    let mut farthest_fixed_particle = 0.0;

    //let thermostat = Andersen::new(TEMP);
    //let integrator = integrators::VelocityVerlet;
    //let integrator = integrators::Langevin::new(DAMPING, TEMP);
    let integrator = integrators::Brownian::<integrators::BAOAB>::new(DAMPING, TEMP);
    let mut xyz_writer = XYZWriter::new(TRAJECTORY_PATH);
    let mut xyz_writer_fixed = XYZWriter::new("fixed.xyz.gz");

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).unwrap()).unwrap();

    let now = Instant::now();

    // Main MD loop
    for i in 0..N_STEPS {
        particles
            .par_iter_mut()
            .for_each(|p| {
                integrator.integrate_a(p);
            });

        //let potential = compute_forces::<PP>(&mut particles);
        let potential = 0.0;

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
                for f_p in (&fixed_particles).iter().rev() {
                    let mut dr = p.position - f_p.position;
                    dr -= BOX_SIZE * minimum_image(dr, BOX_SIZE);

                    if dr.magnitude2() < 1.0 {
                        return Some(i);
                    }
                }
                None
            })
            .collect::<Vec<usize>>();

        // Reset newly fixed particles
        // Tricky to parallelise because of required mutable access to particles[]
        fixed_particles.extend(new_fixed_particles
            .into_iter()
            .map(|i_f_p| {
                // Clone particle's position, and randomise the original particle
                let p = particles[i_f_p].position;

                // Calculate distance from seed
                let mut dr = p - fixed_particles[0].position;
                dr -= BOX_SIZE * minimum_image(dr, BOX_SIZE);

                if dr.magnitude2() > farthest_fixed_particle {
                    farthest_fixed_particle = dr.magnitude2();
                }

                particles[i_f_p] = new_shell_particle(n, &mut thread_rng(), farthest_fixed_particle, fixed_particles[0].position);

                FixedParticle{position: p, timestep: i}
            })
            .collect::<Vec<FixedParticle>>()
        );

        // Add more walking particles
        let desired_particles = (2.0 * farthest_fixed_particle.sqrt()) as usize;
        let new_particles = desired_particles.saturating_sub(particles.len());
        particles.extend((0..new_particles)
            .into_par_iter()
            .map_init(thread_rng, |rng, _| {
                new_shell_particle(n, rng, farthest_fixed_particle, fixed_particles[0].position)
            })
            .collect::<Vec<Particle>>()
        );

        // Move particles inside outer shell
        particles
            .par_iter_mut()
            .for_each_init(thread_rng, |rng, p| {
                // Calculate distance from seed
                let dr = p.position - fixed_particles[0].position;

                if dr.magnitude() > (r2(farthest_fixed_particle.sqrt())) {
                    *p = new_shell_particle(n, rng, farthest_fixed_particle, fixed_particles[0].position);
                }
            });

        if i % DUMP_INTERVAL == 0 {
            let walltime = now.elapsed().as_secs() as f64;
            let fixed_over_time = fixed_particles.len() as f64 / walltime;
            let ts_over_time = i as f64 / walltime;
            println!("Timestep {:.3}, E={:.3}, E_kin={:.3}, E_pot={:.3}, T={:.3}, n_particles={}, \
             n_fixed={}, n_fixed_overtime={:.3}, ts_overtime={:.3}, r2={:.3}", i,
                     potential + kinetic, kinetic, potential,
                     temperature(kinetic,particles.len()), particles.len(),
                     fixed_particles.len(), fixed_over_time, ts_over_time, farthest_fixed_particle);
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

fn new_random_particle<T: Distribution<f64>, U: Distribution<f64>>(dp: T, dv: U, rng: &mut ThreadRng) -> Particle {
    // Give particles random (uniformly distributed) positions and (Gaussian distributed) velocities
    let position = Point::origin().map(|_| dp.sample(rng));
    let velocity = Vector::zero().map(|_| dv.sample(rng));

    Particle {
        old_position: position,
        position: position + velocity * TIMESTEP,
        velocity: velocity,
        force: Vector::zero(),
    }
}

fn new_shell_particle(n: rand_distr::Normal<f64>, rng: &mut ThreadRng, r_min_sq: f64, center: Point) -> Particle {
    // Generate particle on sphere r_min_sq.sqrt() + DR
    let a: [f64; N as usize] = utility::rand_unit_hypersphere::<{N as usize}>(rng).try_into().unwrap();
    let velocity = Vector::zero().map(|_| n.sample(rng));

    Particle {
        old_position: center + Vector::from(a) * r1(r_min_sq.sqrt()),
        position: center + Vector::from(a) * r1(r_min_sq.sqrt()) + velocity * TIMESTEP,
        velocity: velocity,
        force: Vector::zero(),
    }
}
