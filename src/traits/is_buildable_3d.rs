use std::hash::{Hash};

use traits::is_3d::Is3D;
use matrix4::Matrix4;

pub trait IsBuildable3D :
    Is3D +
    Eq +
    PartialEq +
    Ord +
    PartialOrd +
    Hash {

    fn new() -> Box<Self>;

    fn build(x: f64, y: f64, z: f64) -> Box<Self>;
    
    fn from<P>(&mut self, other: P) where P: IsBuildable3D;

    //@todo return new or alter self???
    fn multiply_m(&self, m: &Matrix4) -> Box<Self> {
        let mut result_x = 0.0;
        let mut result_y = 0.0;
        let mut result_z = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                let addition = match j {
                    0 => m.data[i][j] * self.x(),
                    1 => m.data[i][j] * self.y(),
                    _ => m.data[i][j] * self.z()
                };
                match i { //@todo can be simplified
                    0 => {let newx = result_x + addition; result_x = newx;},
                    1 => {let newy = result_y + addition; result_y = newy;},
                    _ => {let newz = result_z + addition; result_z = newz;},
                }
            }
        }
        Self::build(result_x, result_y, result_z)
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
