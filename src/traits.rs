use point::{Point};
use pointCloud::{PointCloud};

//@todo point and pc also as trait

pub trait IsMoveable {
    fn move_by(&mut self, x: f64, y: f64, z: f64);
}

//@todo maybe rename to HasCenter to not confuse
pub trait HasPosition {
    fn new() -> Self;
    fn build(x: f64, y: f64, z: f64) -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable? Or remove moveable trait
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
}

//@todo implement for pointcloud (already has a method for bbox)
pub trait HasBoundingBox : HasPosition {
    fn bounding_box(&self) -> Option<(Point, Point)>;
    //@todo below methods can be implemented in here
    fn min_pos(&self) -> Option<(Point)>;
    fn max_pos(&self) -> Option<(Point)>;
    fn is_inside<B>(&self, other: &B) -> bool where B: HasBoundingBox;
    fn contains<P>(&self, other: &P) -> bool where P: HasPosition;
    fn contains_fully<B>(&self, other: &B) -> bool where B: HasBoundingBox;
    fn collides_with<B>(&self, other: &B) -> bool where B: HasBoundingBox;
}

//@todo currently it is not possible to create immutable trees because of this
//@todo add method, which builds from data directly
//@todo abstract to only use a HasPosition trait instead of Points
pub trait IsTree<P> where P: HasPosition {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn to_pointcloud(&self) -> PointCloud<P>;
    fn build(&mut self, pc : PointCloud<P>) -> bool;
}

pub trait IsOcTree<P> : IsTree<P> where P: HasPosition {
    fn collect(&self, maxdepth: i8) -> PointCloud<P>;
}

pub trait IsKdTree<P> : IsTree<P> where P: HasPosition {
    fn nearest(&self, search: &P) -> Option<P>;
    fn knearest(&self, search: &P, n: usize) -> PointCloud<P>;
    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud<P>;
    fn in_box(&self, search: &P, xSize: f64, ySize: f64, zSize: f64) -> PointCloud<P>;
}
