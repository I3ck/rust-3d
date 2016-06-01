use point3D::Point3D;

use traits::{IsPlane3D, HasPosition3D};

pub struct Plane3D<P> where P: HasPosition3D {
    pub origin: P,
    pub u: P,
    pub v: P
}


impl<P> IsPlane3D<P> for Plane3D<P> where P: HasPosition3D {
    fn new() -> Box<Self> {
        Box::new(Plane3D {
            origin: *P::build(0.0, 0.0, 0.0),
            u: *P::build(1.0, 0.0, 0.0),
            v: *P::build(0.0, 1.0, 0.0)
        })
    }

    fn build(origin: P, u: P, v: P) -> Box<Self> {
        Box::new(Plane3D {
            origin: origin,
            u: u,
            v: v
        })
    }

    fn origin(&self) -> P {
        self.origin.clone()
    }

    fn u(&self) -> P {
        self.u.clone()
    }

    fn v(&self) -> P {
        self.v.clone()
    }
}
