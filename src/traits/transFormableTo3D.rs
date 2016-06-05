use traits::hasPosition2D::HasPosition2D;
use traits::hasPosition3D::HasPosition3D;

pub trait TransFormableTo3D : HasPosition2D {
    fn transform_to_3D<P>(&self, z: f64) -> P where P: HasPosition3D;
}
