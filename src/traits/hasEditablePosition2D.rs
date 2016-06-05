extern crate core;
use self::core::str::FromStr;

use traits::hasPosition2D::HasPosition2D;

pub trait HasEditablePosition2D : HasPosition2D {
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable3D? Or remove moveable trait
    fn set_y(&mut self, val: f64);

    fn set_pos(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }

    fn add<P>(&mut self, other: &P) where P: HasPosition2D {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn substract<P>(&mut self, other: &P) where P: HasPosition2D {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn scale(&mut self, val: f64) {
        let x = val * self.x();
        let y = val * self.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn rotate<P>(&mut self, rad: f64, center: &P) where P: HasPosition2D {
        let newx = center.x() + rad.cos() * (self.x() - center.x()) - rad.sin() * (self.y() - center.y());
        let newy = center.y() + rad.sin() * (self.x() - center.x()) + rad.cos() * (self.y() - center.y());

        self.set_x(newx);
        self.set_y(newy);
    }

    fn parse(text: String) -> Option<Box<Self>> {
        let split = text.split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            2 => {
                let mut p = Self::new();
                match f64::from_str(words[0]) {
                    Err(_) => return None,
                    Ok(x) => p.set_x(x)
                };
                match f64::from_str(words[1]) {
                    Err(_) => return None,
                    Ok(y) => p.set_y(y)
                };
                Some(p)
            },
            _ => None
        }
    }
}
