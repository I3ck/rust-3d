use traits::is_2d::Is2D;

pub trait IsNormalized2D : Is2D {
    fn new<P>(p: P) -> Option<Box<Self>> where
        P: Is2D;

    fn norm_x() -> Self;

    fn norm_y() -> Self;
}
