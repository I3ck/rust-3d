use traits::is2D::Is2D;
use traits::hasPosition2D::HasPosition2D;

pub trait IsNormalized2D : Is2D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition2D;
    fn norm_x() -> Self;
    fn norm_y() -> Self;
}
