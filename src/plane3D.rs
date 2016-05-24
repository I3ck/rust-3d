use point3D::Point3D;

use traits::{IsPlane3D, HasPosition3D};

pub struct Plane3D<P> where P: HasPosition3D {
    pub origin: P,
    pub normal: P,
    pub length_dir: P,
    pub width: f64,
    pub height: f64
}


impl<P> IsPlane3D<P> for Plane3D<P> where P: HasPosition3D {
    fn new() -> Box<Self> {
        Box::new(Plane3D {
            origin: *P::new(),
            normal: *P::new(),
            length_dir: *P::new(),
            width: 0.0,
            height: 0.0
        })
    }

    fn build(origin: P, normal: P, length_dir: P, width: f64, height: f64) -> Box<Self> {
        Box::new(Plane3D {
            origin: origin,
            normal: normal,
            length_dir: length_dir,
            width: width,
            height: height
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

    fn width(&self) -> f64 {
        self.width
    }

    fn set_width(&mut self, width: f64) {
        self.width = width
    }
    fn height(&self) -> f64 {
        self.height
    }

    fn set_height(&mut self, height: f64) {
        self.height = height
    }
}
