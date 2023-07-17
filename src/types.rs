use cgmath::{Point3, Vector3};
use crate::xyz::WritableParticle;

#[derive(Debug, Clone)]
pub struct Particle {
    pub old_position: Point3<f64>,
    pub position:     Point3<f64>,
    pub velocity:     Vector3<f64>,
    pub force:        Vector3<f64>,
}

#[derive(Debug, Clone)]
pub struct FixedParticle {
    pub position: Point3<f64>,
    pub timestep: u32,
}

impl WritableParticle for Particle {
    fn write(&self, i: usize) -> String {
        format!(
            "{}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\t{:.3}\n", i,
            self.position.x, self.position.y, self.position.z,
            self.velocity.x, self.velocity.y, self.velocity.z
        )
    }
}

impl WritableParticle for FixedParticle {
    fn write(&self, i: usize) -> String {
        format!(
            "{}\t{:.3}\t{:.3}\t{:.3}\t{}\n", i,
            self.position.x, self.position.y, self.position.z, self.timestep
        )
    }
}
