mod lennard_jones;
pub use lennard_jones::*;

pub trait PairPotential {
    /// Energy of the pair interaction
    /// * `dr` - Distance between particles
    fn energy(dr: f64) -> f64;

    /// Magnitude of the pair force in the direction of dr
    /// * `dr` - Distance between particles
    fn force(dr: f64) -> f64;
}