use traits::has_position_2d::HasPosition2D;
use traits::has_position_3d::HasPosition3D;

pub trait TransFormableTo3D : HasPosition2D {
    fn transform_to_3d<P>(&self, z: f64) -> P where P: HasPosition3D;
}
