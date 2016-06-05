use std::hash::{Hash};

use traits::is3D::Is3D;
use matrix4::Matrix4;

pub trait HasPosition3D : Is3D + Eq + PartialEq + Ord + PartialOrd + Hash {
    fn new() -> Box<Self>;
    fn build(x: f64, y: f64, z: f64) -> Box<Self>;
    fn from<P>(&mut self, other: P) where P: HasPosition3D;

    //@todo return new or alter self???
    fn multiplyM(&self, m: &Matrix4) -> Box<Self> {
        let mut resultX = 0.0;
        let mut resultY = 0.0;
        let mut resultZ = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                let addition = match j {
                    0 => m.data[i][j] * self.x(),
                    1 => m.data[i][j] * self.y(),
                    _ => m.data[i][j] * self.z()
                };
                match i { //@todo can be simplified
                    0 => {let newx = resultX + addition; resultX = newx;},
                    1 => {let newy = resultY + addition; resultY = newy;},
                    _ => {let newz = resultZ + addition; resultZ = newz;},
                }
            }
        }
        Self::build(resultX, resultY, resultZ)
    }

    fn normalized(&self) -> Option<Box<Self>> {
        let l = self.abs();
        if l <= 0.0 {
            None
        }
        else {
            Some(Self::build(self.x() / l, self.y() / l, self.z() / l))
        }
    }
}
