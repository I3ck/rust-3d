extern crate core;

use point3D::{Point3D};
use pointCloud3D::{Point3DCloud3D};

use self::core::str::FromStr;
use std::hash::{Hash};

//@todo point and pc also as trait

pub trait IsMoveable3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64);
}

pub trait HasPosition3D : Eq + PartialEq + Ord + PartialOrd + Hash {
    fn new() -> Box<Self>;
    fn build(x: f64, y: f64, z: f64) -> Box<Self>; //@todo can be implemented here
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable3D? Or remove moveable trait
    fn set_y(&mut self, val: f64);
    fn set_z(&mut self, val: f64);
    fn clone(&self) -> Self;

    fn pos(&self) -> (f64, f64, f64) {
        ( self.x(), self.y(), self.z() )
    }

    fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }

    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();
        let sz: String = self.z().to_string();

        sx + " " + &sy + " " + &sz
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
pub trait IsTree3D<P> where P: HasPosition3D {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn to_pointcloud(&self) -> Point3DCloud3D<P>;
    fn build(&mut self, pc : Point3DCloud3D<P>) -> bool;
}

pub trait IsOcTree<P> : IsTree3D<P> where P: HasPosition3D {
    fn collect(&self, maxdepth: i8) -> Point3DCloud3D<P>;
}

pub trait IsKdTree3D<P> : IsTree3D<P> where P: HasPosition3D {
    fn nearest(&self, search: &P) -> Option<P>;
    fn knearest(&self, search: &P, n: usize) -> Point3DCloud3D<P>;
    fn in_sphere(&self, search: &P, radius: f64) -> Point3DCloud3D<P>;
    fn in_box(&self, search: &P, xSize: f64, ySize: f64, zSize: f64) -> Point3DCloud3D<P>;
}
