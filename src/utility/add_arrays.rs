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

#[cfg(test)]
mod tests {
    #[test]
    fn add_arrays() {
        let a = [1, 2, 3];
        let b = [2, 3, 4];

        assert_eq!(crate::utility::add_arrays_into(a, b), [3, 5, 7])
    }
}
