use point::{Point};
use pointCloud::{PointCloud};

//@todo point and pc also as trait

pub trait IsMoveable {
    fn move_by(&mut self, x: f64, y: f64, z: f64);
}

//@todo maybe rename to HasCenter to not confuse
pub trait HasPosition {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn set_x(&mut self, val: f64);
    fn set_y(&mut self, val: f64);
    fn set_z(&mut self, val: f64);

    fn pos(&self) -> (f64, f64, f64) {
        ( self.x(), self.y(), self.z() )
    }

    fn set_pos(&mut self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }
}

//@todo currently it is not possible to create immutable trees because of this
//@todo add method, which builds from data directly
//@todo abstract to only use a HasPosition trait instead of Points
pub trait IsTree {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn to_pointcloud(&self) -> PointCloud;
    fn build(&mut self, pc : PointCloud) -> bool;
}

pub trait IsOcTree : IsTree {
    fn collect(&self, maxdepth: i8) -> PointCloud;
}

pub trait IsKdTree : IsTree {
    fn nearest(&self, search: &Point) -> Option<Point>;
    fn knearest(&self, search: &Point, n: usize) -> PointCloud;
    fn in_sphere(&self, search: &Point, radius: f64) -> PointCloud;
    fn in_box(&self, search: &Point, xSize: f64, ySize: f64, zSize: f64) -> PointCloud;
}
