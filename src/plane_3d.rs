use traits::is_plane_3d::IsPlane3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_normalized_3d::IsNormalized3D;

pub struct Plane3D<P,N> where
    P: IsBuildable3D,
    N: IsNormalized3D {

    pub origin: P,
    pub u: N,
    pub v: N
}


impl<P,N> IsPlane3D<P,N> for Plane3D<P,N> where
    P: IsBuildable3D,
    N: IsNormalized3D {
        
    fn new() -> Box<Self> {
        Box::new(Plane3D {
            origin: *P::build(0.0, 0.0, 0.0),
            u: N::norm_x(),
            v: N::norm_y()
        })
    }

    fn build(origin: P, u: N, v: N) -> Box<Self> {
        Box::new(Plane3D {
            origin: origin,
            u: u,
            v: v
        })
    }

    fn origin(&self) -> P {
        self.origin.clone()
    }

    fn u(&self) -> N {
        self.u.clone()
    }

    fn v(&self) -> N {
        self.v.clone()
    }
}
