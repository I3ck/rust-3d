extern crate core;

use point2D::{Point2D};
use point3D::{Point3D};
use matrix4::Matrix4;
use pointCloud2D::{PointCloud2D};
use pointCloud3D::{PointCloud3D};

use self::core::str::FromStr;
use std::hash::{Hash};

//@todo point and pc also as trait
pub trait IsMoveable2D { //@todo remove trait and impl in HasPosition2D
    fn move_by(&mut self, x: f64, y: f64);
}

pub trait IsMoveable3D { //@todo remove trait and impl in HasPosition2D
    fn move_by(&mut self, x: f64, y: f64, z: f64);
}

///@todo parse could be moved to non-editable traits, but cant be implemented there?

///@todo split into several files
//@todo finish trait and add implementation
//@todo better method names
//@todo maybe implement projection methods within the pc
//@todo transformable traits required later on?
pub trait IsProjectionToPlane<P2,P3,N> where P2: HasEditablePosition2D + TransFormableTo3D, P3: HasEditablePosition3D + TransFormableTo2D, N: IsNormalized3D {
    fn from_2d<PL>(plane: PL, pc: PointCloud2D<P2>) -> Box<Self> where PL: IsPlane3D<P3,N>; //places 2d pc on plane, assuming plane 0/0 == pc2d 0/0
    fn from_3d<PL>(plane: PL, pc: PointCloud3D<P3>) -> Box<Self> where PL: IsPlane3D<P3,N>; //projects 3d pc onto plane from global coords
    fn projected_pointcloud_3d_global(&self) -> PointCloud3D<P3>;
    fn plane<PL>(&self) -> PL where PL: IsPlane3D<P3,N>;
    fn projected_pointcloud_2d_local(&self) -> PointCloud2D<P2>;
    //@todo add overload which lets one set the layer count?
    fn extrude(&self, distance: f64) -> (PointCloud3D<P3>, PointCloud3D<P3>); //@todo fst = on plane, snd within dist (maybe add data type for this)
}

pub trait IsPlane3D<P,N> where P: HasPosition3D, N: IsNormalized3D {
    fn new() -> Box<Self>;
    fn build(origin: P, u: N, v: N) -> Box<Self>;
    fn origin(&self) -> P;
    fn u(&self) -> N;
    fn v(&self) -> N;
}

pub trait Is2D {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn clone(&self) -> Self;

    fn pos(&self) -> (f64, f64) {
        ( self.x(), self.y() )
    }

    fn dot<P>(&self, other: &P) -> f64 where P: HasPosition2D {
        self.x() * other.x() + self.y() * other.y()
    }

    fn cross<P>(&self, other: &P) -> f64 where P: HasPosition2D {
        self.x() * other.y() - self.y() * other.x()
    }

    fn abs(&self) -> f64 {
        (self.x()).powi(2) + (self.y()).powi(2)
    }

    fn rad_to<P>(&self, other: &P) -> f64 where P: HasPosition2D {
        (other.y() - self.y()).atan2(other.x() - self.x())
    }

    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();

        sx + " " + &sy
    }
}

pub trait HasPosition2D : Is2D +  Eq + PartialEq + Ord + PartialOrd + Hash {
    fn new() -> Box<Self>;
    fn build(x: f64, y: f64) -> Box<Self>;
    fn from<P>(&mut self, other: P) where P: HasPosition2D;

    fn normalized(&self) -> Option<Box<Self>> {
        let l = self.abs();
        if l <= 0.0 {
            None
        }
        else {
            Some(Self::build(self.x() / l, self.y() / l))
        }
    }
}
pub trait HasEditablePosition2D : HasPosition2D {
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable3D? Or remove moveable trait
    fn set_y(&mut self, val: f64);

    fn set_pos(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }

    fn add<P>(&mut self, other: &P) where P: HasPosition2D {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn substract<P>(&mut self, other: &P) where P: HasPosition2D {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn scale(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn rotate<P>(&mut self, rad: f64, center: &P) where P: HasPosition2D {
        let newx = center.x() + rad.cos() * (self.x() - center.x()) - rad.sin() * (self.y() - center.y());
        let newy = center.y() + rad.sin() * (self.x() - center.x()) + rad.cos() * (self.y() - center.y());

        self.set_x(newx);
        self.set_y(newy);
    }

    fn parse(text: String) -> Option<Box<Self>> {
        let split = text.split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            2 => {
                let mut p = Self::new();
                match f64::from_str(words[0]) {
                    Err(_) => return None,
                    Ok(x) => p.set_x(x)
                };
                match f64::from_str(words[1]) {
                    Err(_) => return None,
                    Ok(y) => p.set_y(y)
                };
                Some(p)
            },
            _ => None
        }
    }
}
/*
pub trait Mesh3D<P> where P: HasPosition3D {
    fn nFaces
}
*/

pub trait TransFormableTo3D : HasPosition2D {
    fn transform_to_3D<P>(&self, z: f64) -> P where P: HasPosition3D;
}

pub trait TransFormableTo2D : HasPosition3D {
    fn transform_to_2D<P>(&self) -> P where P: HasPosition2D;
}

pub trait IsNormalized3D : Is3D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition3D;
    fn norm_x() -> Self;
    fn norm_y() -> Self;
    fn norm_z() -> Self;
}

pub trait IsNormalized2D : Is2D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition2D;
    fn norm_x() -> Self;
    fn norm_y() -> Self;
}
//@todo split HasPositionxD into reading and writing parts
//@todo this way NormalizedxD can be of this type and be used
//@todo in way more cases

pub trait Is3D {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn clone(&self) -> Self;

    fn pos(&self) -> (f64, f64, f64) {
        ( self.x(), self.y(), self.z() )
    }

    fn dot<P>(&self, other: &P) -> f64 where P: Is3D {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<P,HP>(&self, other: &P) -> Box<HP> where P: Is3D, HP: HasPosition3D {
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.z() * other.x() - self.x() * other.z();
        let z = self.x() * other.y() - self.y() * other.x();
        HP::build(x, y, z)
    }

    fn abs(&self) -> f64 {
        (self.x()).powi(2) + (self.y()).powi(2) + (self.z()).powi(2)
    }

    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();
        let sz: String = self.z().to_string();

        sx + " " + &sy + " " + &sz
    }
}

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

pub trait HasEditablePosition3D : HasPosition3D {
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable3D? Or remove moveable trait
    fn set_y(&mut self, val: f64);
    fn set_z(&mut self, val: f64);

    fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn add<P>(&mut self, other: &P) where P: HasPosition3D {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn substract<P>(&mut self, other: &P) where P: HasPosition3D {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn scale(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        let z = val * self.z();
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn parse(text: String) -> Option<Box<Self>> {
        let split = text.split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let mut p = Self::new();
                match f64::from_str(words[0]) {
                    Err(_) => return None,
                    Ok(x) => p.set_x(x)
                };
                match f64::from_str(words[1]) {
                    Err(_) => return None,
                    Ok(y) => p.set_y(y)
                };
                match f64::from_str(words[2]) {
                    Err(_) => return None,
                    Ok(z) => p.set_z(z)
                };
                Some(p)
            },
            _ => None
        }
    }
}

//@todo implement for pointcloud (already has a method for bbox)
//@todo could be defined for any number of dimensions
pub trait HasBoundingBox3D : HasPosition3D {
    fn bounding_box(&self) -> Option<(Point3D, Point3D)>;
    //@todo below methods can be implemented in here
    fn min_pos(&self) -> Option<(Point3D)>;
    fn max_pos(&self) -> Option<(Point3D)>;
    fn is_inside<B>(&self, other: &B) -> bool where B: HasBoundingBox3D;
    fn contains<P>(&self, other: &P) -> bool where P: HasPosition3D;
    fn contains_fully<B>(&self, other: &B) -> bool where B: HasBoundingBox3D;
    fn collides_with<B>(&self, other: &B) -> bool where B: HasBoundingBox3D;
}

//@todo currently it is not possible to create immutable trees because of this
//@todo add method, which builds from data directly
//@todo abstract to only use a HasPosition3D trait instead of Point3Ds
pub trait IsTree3D<P> where P: HasEditablePosition3D {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn to_pointcloud(&self) -> PointCloud3D<P>;
    fn build(&mut self, pc : PointCloud3D<P>) -> bool;
}

pub trait IsOcTree<P> : IsTree3D<P> where P: HasEditablePosition3D {
    fn collect(&self, maxdepth: i8) -> PointCloud3D<P>;
}

pub trait IsKdTree3D<P> : IsTree3D<P> where P: HasEditablePosition3D {
    fn nearest(&self, search: &P) -> Option<P>;
    fn knearest(&self, search: &P, n: usize) -> PointCloud3D<P>;
    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud3D<P>;
    fn in_box(&self, search: &P, xSize: f64, ySize: f64, zSize: f64) -> PointCloud3D<P>;
}
