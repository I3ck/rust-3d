use traits::has_position_2d::HasPosition2D;
use traits::has_position_3d::HasPosition3D;

pub trait TransFormableTo2D : HasPosition3D {
    fn transform_to_2D<P>(&self) -> P where P: HasPosition2D;
}
