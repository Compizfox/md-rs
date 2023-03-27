mod verlet;
mod velocity_verlet;

pub use verlet::*;
pub use velocity_verlet::*;

use crate::types::Particle;

pub trait Integrator {
    /// Integration before force calculation
    /// * `p` - Particle to integrate
    /// * `limit` - Optional speed limit
    fn integrate_a(p: &mut Particle);

    /// Integration after force calculation
    /// * `p` - Particle to integrate
    /// * `limit` - Optional speed limit
    fn integrate_b(p: &mut Particle, limit: Option<f64>);
}
