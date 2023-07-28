use cgmath::{Point2, Point3, Vector2, Vector3};
use crate::xyz::WritableParticle;

use crate::{Vector, Point};

#[derive(Debug, Clone)]
pub struct Particle {
    pub old_position: Point,
    pub position:     Point,
    pub velocity:     Vector,
    pub force:        Vector,
}

#[derive(Debug, Clone)]
pub struct FixedParticle {
    pub position: Point,
    pub timestep: u32,
}

impl WritableParticle for Particle {
    fn write(&self, i: usize) -> String {
        format!("{}\t", i)
        + &self.position.write() + "\t" + &self.velocity.write() + "\n"
    }
}

impl WritableParticle for FixedParticle {
    fn write(&self, i: usize) -> String {
        format!("{}\t", i)
        + &self.position.write() + "\t" + &format!("{}", self.timestep) + "\n"
    }
}

trait WritableTuple {
    fn write(&self) -> String;
}

impl WritableTuple for Point2<f64> {
    fn write(&self) -> String {
        format!("{:.3}\t{:.3}", self.x, self.y)
    }
}

impl WritableTuple for Point3<f64> {
    fn write(&self) -> String {
        format!("{:.3}\t{:.3}\t{:.3}", self.x, self.y, self.z)
    }
}

impl WritableTuple for Vector2<f64> {
    fn write(&self) -> String {
        format!("{:.3}\t{:.3}", self.x, self.y)
    }
}

impl WritableTuple for Vector3<f64> {
    fn write(&self) -> String {
        format!("{:.3}\t{:.3}\t{:.3}", self.x, self.y, self.z)
    }
}
