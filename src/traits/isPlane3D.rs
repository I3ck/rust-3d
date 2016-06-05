use traits::hasPosition3D::HasPosition3D;
use traits::isNormalized3D::IsNormalized3D;


pub trait IsPlane3D<P,N> where P: HasPosition3D, N: IsNormalized3D {
    fn new() -> Box<Self>;
    fn build(origin: P, u: N, v: N) -> Box<Self>;
    fn origin(&self) -> P;
    fn u(&self) -> N;
    fn v(&self) -> N;
}
