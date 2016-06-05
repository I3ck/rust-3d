use traits::has_position_2d::HasPosition2D;
use traits::is_3d::Is3D;

pub trait TransFormableTo2D : Is3D {
    fn transform_to_2d<P>(&self) -> P where
        P: HasPosition2D;
}
