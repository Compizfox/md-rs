use std::ops::Add;
use arrayvec::ArrayVec;

/// Adds two arrays element-wise
/// * `a`, `b` - Two arrays (equally-sized)
pub fn add_arrays_into<T: Add<T, Output=T>, const N: usize>(a: [T; N], b: [T; N]) -> [T; N] {
    let c: ArrayVec<T, N> = a.
        into_iter()
        .zip(b)
        .map(|(a, b)| a + b)
        .collect();

    c.into_inner().unwrap_or_else(|_| panic!("This is impossible"))
}

/// Adds two vectors element-wise
/// * `a`, `b` - Two vectors
pub fn add_vecs_into<T: Add<T, Output=T>>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.
        into_iter()
        .zip(b)
        .map(|(a, b)| a + b)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utility::add_vecs_into;

    #[test]
    fn add_arrays_into() {
        let a = [1, 2, 3];
        let b = [2, 3, 4];

        assert_eq!(crate::utility::add_arrays_into(a, b), [3, 5, 7]);
    }

    #[test]
    fn add_vectors_into() {
        let a = vec![1, 2, 3];
        let b = vec![2, 3, 4];

        assert_eq!(add_vecs_into(a, b), [3, 5, 7]);
    }
}
