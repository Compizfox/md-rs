use cgmath::prelude::*;

use crate::{Vector, Point};

/// Computes image flags.
/// * `p` - Particle position
/// * `box_size` - (Cubic) box size
pub fn image(p: Point, box_size: f64) -> Vector {
    p.map(|x| (x / box_size).floor()).to_vec()
}

/// Computes minimum image flags
/// * `v` - Distance vector
/// * `box_size` - (Cubic) box size
pub fn minimum_image(v: Vector, box_size: f64) -> Vector {
    v.map(|x| (x / box_size).round())
}
