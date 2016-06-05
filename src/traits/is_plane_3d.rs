use traits::is_3d::Is3D;
use traits::has_position_3d::HasPosition3D;
use traits::is_normalized_3d::IsNormalized3D;


pub trait IsPlane3D<P,N> where
    P: Is3D,
    N: IsNormalized3D {

    fn new() -> Box<Self>;

    fn build(origin: P, u: N, v: N) -> Box<Self>;

    fn origin(&self) -> P;

    fn u(&self) -> N;

    fn v(&self) -> N;
}
