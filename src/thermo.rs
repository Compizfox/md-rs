/// Computes average temperature of the system by the equipartition theorem.
/// * `e_kin` - Total kinetic energy
/// * `n_particles` - Number of particles
pub fn temperature(e_kin: f64, n_particles: usize) -> f64 {
    e_kin * 2.0 / 3.0 / n_particles as f64
}

