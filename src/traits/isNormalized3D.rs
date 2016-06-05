use traits::is3D::Is3D;
use traits::hasPosition3D::HasPosition3D;

pub trait IsNormalized3D : Is3D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition3D;
    fn norm_x() -> Self;
    fn norm_y() -> Self;
    fn norm_z() -> Self;
}
