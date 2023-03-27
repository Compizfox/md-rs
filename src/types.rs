use cgmath::{Point3, Vector3};

#[derive(Debug, Clone)]
pub struct Particle {
    pub old_position: Point3<f64>,
    pub position:     Point3<f64>,
    pub velocity:     Vector3<f64>,
    pub force:        Vector3<f64>,
}