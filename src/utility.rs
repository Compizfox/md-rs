use std::ops::Add;
use rayon::prelude::*;
use cgmath::{Point3, Vector3};

/// Adds two (equal-length) vectors element-wise
pub fn par_add_vectors<T: Send + Add<T, Output = T>>(vec_a: Vec<T>, vec_b: Vec<T>) -> Vec<T> {
    vec_a
        .into_par_iter()
        .zip_eq(vec_b)
        .map(|(a, b)| a + b)
        .collect()
}

/// Adds two slices element-wise
pub fn add_arrays<T: Copy + Add<T, Output=T>>(a: &[T], b: &[T]) -> Vec<T> {
    a.
        iter()
        .zip(b)
        .map(|(a, b)| *a + *b)
        .collect()
}

/// Computes image flags.
/// * `p` - Particle position
/// * `box_size` - (Cubic) box size
pub fn image(p: Point3<f64>, box_size: f64) -> Vector3<f64> {
    Vector3::new(
        (p.x / box_size).floor(),
        (p.y / box_size).floor(),
        (p.z / box_size).floor(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_arrays() {
        let a = [1, 2, 3];
        let b = [2, 3, 4];

        assert_eq!(crate::utility::add_arrays(&a, &b), [3, 5, 7])
    }
}
