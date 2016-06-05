use traits::hasPosition2D::HasPosition2D;
use traits::hasPosition3D::HasPosition3D;

pub trait TransFormableTo2D : HasPosition3D {
    fn transform_to_2D<P>(&self) -> P where P: HasPosition2D;
}
