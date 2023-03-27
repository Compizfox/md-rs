mod andersen;

pub use andersen::*;
use crate::types::Particle;

pub trait Thermostat {
    fn run(&self, p: &mut Particle);
}