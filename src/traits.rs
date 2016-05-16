use point::{Point};
use pointCloud::{PointCloud};

//@todo point and pc also as trait

pub trait MoveAble { //@todo rename to isMoveable
    fn move_by(&mut self, x: f64, y: f64, z: f64);
}

//@todo currently it is not possible to create immutable trees because of this
//@todo add method, which builds from data directly
pub trait IsTree {
    fn new() -> Self;
    fn size(&self) -> usize;
    //toPointcloud (could be collect method)
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
