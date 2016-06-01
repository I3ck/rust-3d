use traits::{IsNormalized3D, HasPosition3D};

pub struct Norm3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl IsNormalized3D for Norm3D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition3D {
        match p.abs() {
            0.0 => None,
            l => Some(Box::new(Norm3D {
                x: p.x() / l,
                y: p.y() / l,
                z: p.z() / l,
            }))
        }
    }
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
}
