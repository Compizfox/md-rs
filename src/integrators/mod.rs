mod verlet;
mod velocity_verlet;
mod langevin;
mod brownian;

pub use verlet::*;
pub use velocity_verlet::*;
pub use langevin::*;
pub use brownian::*;

use crate::types::Particle;

pub trait Integrator {
    /// Integration before force calculation
    /// * `p` - Particle to integrate
    fn integrate_a(&self, p: &mut Particle);

    /// Integration after force calculation
    /// * `p` - Particle to integrate
    /// * `limit` - Optional speed limit
    fn integrate_b(&self, p: &mut Particle, limit: Option<f64>);
}
