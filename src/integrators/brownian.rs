use std::marker::PhantomData;
use cgmath::prelude::*;
use cgmath::{Vector3};
use rand::prelude::*;

use crate::integrators::Integrator;
use crate::pbc::image;
use crate::{BOX_SIZE, TIMESTEP};
use crate::types::Particle;

pub trait BrownianIntegrator {
    fn integrate(p: &mut Particle, y: f64, T: f64);
}

/// Brownian dynamics by Euler-Maruyama:
/// https://watermark.silverchair.com/abs010.pdf
pub struct EulerMaruyama;
impl BrownianIntegrator for EulerMaruyama {
    fn integrate(p: &mut Particle, y: f64, T: f64) {
        let n = rand_distr::StandardNormal;
        let mut rng = thread_rng();

        let a = Vector3::new(
            n.sample(&mut rng),
            n.sample(&mut rng),
            n.sample(&mut rng)
        );
        p.old_position = p.position;
        p.position += p.force * TIMESTEP / y + (2.0 * T * TIMESTEP / y).sqrt() * a;
        p.position -= BOX_SIZE*image(p.position, BOX_SIZE);
    }
}

/// Brownian dynamics by BAOAB:
/// https://watermark.silverchair.com/abs010.pdf
pub struct BAOAB;
impl BrownianIntegrator for BAOAB {
    fn integrate(p: &mut Particle, y: f64, T: f64) {
        let n = rand_distr::StandardNormal;
        let mut rng = thread_rng();

        let a1 = Vector3::new(
            n.sample(&mut rng),
            n.sample(&mut rng),
            n.sample(&mut rng)
        );
        let a2 = Vector3::new(
            n.sample(&mut rng),
            n.sample(&mut rng),
            n.sample(&mut rng)
        );
        p.old_position = p.position;
        p.position += p.force * TIMESTEP / y + (T * TIMESTEP / (2.0 * y)).sqrt() * (a1 + a2);
        p.position -= BOX_SIZE*image(p.position, BOX_SIZE);
    }
}

/// Generic Brownian Dynamics integrator
/// (takes an algorithm as generic parameter)
pub struct Brownian<I: BrownianIntegrator> {
    y: f64,
    T: f64,
    _i: PhantomData<I>,
}

impl<I: BrownianIntegrator> Brownian<I> {
    /// Constructs a new `Brownian` integrator.
    /// * `gamma` - damping
    /// * `temperature` - temperature
    pub fn new(gamma: f64, temperature: f64) -> Self {
        Self{
            y: gamma,
            T: temperature,
            _i: PhantomData,
        }
    }
}

impl<I: BrownianIntegrator> Integrator for Brownian<I> {
    fn integrate_a(&self, p: &mut Particle) {
        I::integrate(p, self.y, self.T);
    }

    fn integrate_b(&self, p: &mut Particle, limit: Option<f64>) {}
}
