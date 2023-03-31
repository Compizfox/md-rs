use cgmath::{Point3, Vector3};

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

/// Computes minimum image flags
/// * `v` - Distance vector
/// * `box_size` - (Cubic) box size
pub fn minimum_image(v: Vector3<f64>, box_size: f64) -> Vector3<f64> {
    Vector3::new(
        (v.x / box_size).round(),
        (v.y / box_size).round(),
        (v.z / box_size).round(),
    )
}
