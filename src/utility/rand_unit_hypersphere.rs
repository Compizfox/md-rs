use std::f64::consts::PI;
use rand::Rng;
use rand::rngs::ThreadRng;

/// Samples coordinates uniformly on a unit hypersphere
/// * `N` - Dimensionality
/// * `rng` - Reference to the thread-local random number generator
pub fn rand_unit_hypersphere<const N: usize>(rng: &mut ThreadRng) -> Vec<f64> {
    // Generate N-1 random angular coordinates
    let a: Vec<f64> = (0..N-1)
        .map(|_| rng.gen::<f64>()*2.0*PI)
        .collect();

    hyperspherical_to_cartesian(&a[..], 1.0)
}


/// Converts hyperspherical coordinates to cartesian coordinates
/// * `phi` - Slice of angular coordinates
/// * `r` - Radial coordinate
// I would like to return an [f64, N+1], but generic_const_exprs are not stable yet
pub fn hyperspherical_to_cartesian(phi: &[f64], r: f64) -> Vec<f64> {
    let n = phi.len();
    (0..n+1)
        .map(|i| {
            let mut x_i = r;

            if i < n {
                // Sines of phi[i] up to i
                x_i *= (0..i)
                    .map(|i| phi[i].sin())
                    .product::<f64>();

                // Cosine of phi[i]
                x_i *= phi[i].cos()
            } else {
                // Sines of phi[i] up i
                x_i *= (0..i)
                    .map(|i| phi[i].sin())
                    .product::<f64>();
            }
            x_i
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn hyperspherical_to_cartesian() {
        let r = 3.343;
        let p = [1.334, 3.544];

        assert_eq!(crate::utility::hyperspherical_to_cartesian(&p, r),
                   vec![r * p[0].cos(),
                        r * p[0].sin() * p[1].cos(),
                        r * p[0].sin() * p[1].sin()
                   ]
        );
    }
}
