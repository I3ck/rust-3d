use traits::{IsNormalized2D, HasPosition2D};

pub struct Norm2D {
    pub x: f64,
    pub y: f64
}

impl IsNormalized2D for Norm2D {
    fn new<P>(p: P) -> Option<Box<Self>> where P: HasPosition2D {
        match p.abs() {
            0.0 => None,
            l => Some(Box::new(Norm2D {
                x: p.x() / l,
                y: p.y() / l
            }))
        }
    }
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}
