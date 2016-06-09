use traits::is_2d::Is2D;
use traits::is_buildable_3d::IsBuildable3D;

pub trait TransFormableTo3D : Is2D {
    fn transform_to_3d<P>(&self, z: f64) -> P where
        P: IsBuildable3D;
}
