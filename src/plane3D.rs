use point3D::Point3D;

use traits::{IsPlane3D, HasPosition3D};

pub struct Plane3D<P> where P: HasPosition3D {
    pub origin: P,
    pub normal: P,
    pub length_dir: P
}


impl<P> IsPlane3D<P> for Plane3D<P> where P: HasPosition3D {
    fn new() -> Box<Self> {
        Box::new(Plane3D {
            origin: *P::build(0.0, 0.0, 0.0),
            normal: *P::build(0.0, 0.0, 1.0),
            length_dir: *P::build(1.0, 0.0, 0.0)
        })
    }

    fn build(origin: P, normal: P, length_dir: P) -> Box<Self> {
        Box::new(Plane3D {
            origin: origin,
            normal: normal,
            length_dir: length_dir
        })
    }

    fn origin(&self) -> P {
        self.origin.clone()
    }

    fn set_origin(&mut self, origin: P) {
        self.origin = origin
    }

    fn normal(&self) -> P {
        self.normal.clone()
    }

    fn set_normal(&mut self, normal: P) {
        self.normal = normal
    }

    fn length_dir(&self) -> P {
        self.length_dir.clone()
    }

    fn set_length_dir(&mut self, length_dir: P) {
        self.length_dir = length_dir
    }
}
