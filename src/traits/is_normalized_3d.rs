use traits::is_3d::Is3D;

pub trait IsNormalized3D : Is3D {
    fn new<P>(p: P) -> Option<Box<Self>> where
        P: Is3D;

    fn norm_x() -> Self;

    fn norm_y() -> Self;

    fn norm_z() -> Self;
}
