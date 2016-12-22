/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate core;
use self::core::str::FromStr;

use traits::is_2d::Is2D;
use traits::is_buildable_2d::IsBuildable2D;

pub trait IsEditable2D : IsBuildable2D {
    fn set_x(&mut self, val: f64); //@todo these kinda make it moveable, maybe put into IsMoveable3D? Or remove moveable trait

    fn set_y(&mut self, val: f64);

    fn set_pos(&mut self, x: f64, y: f64) {
        self.set_x(x);
        self.set_y(y);
    }

    fn add<P>(&mut self, other: &P) where
        P: Is2D {

        let x = self.x() + other.x();
        let y = self.y() + other.y();
        self.set_x(x);
        self.set_y(y);
    }

    fn substract<P>(&mut self, other: &P) where
        P: Is2D {

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

    fn rotate<P>(&mut self, rad: f64, center: &P) where
        P: Is2D {

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
