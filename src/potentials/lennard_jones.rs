use crate::potentials::PairPotential;

pub struct LJ;

impl PairPotential for LJ {
    fn energy(dr: f64) -> f64 {
        4.0 * (dr.powi(-12) - dr.powi(-6))
    }
    fn force(dr: f64) -> f64 {
        48.0 * (-dr.powi(-13) + 0.5 * dr.powi(-7))
    }
}