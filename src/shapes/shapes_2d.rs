/*
Copyright 2017 Martin Buck
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

//! Containing basic 2D shape data types used by other types / algorithms

use std::cmp::{Eq, Ordering};

use prelude::*;
use distances_2d::*;

//@todo own file since quite a lot of code now
#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Circle, a circle in 2D space
pub struct Circle {
    pub center: Point2D,
    pub radius: Positive
}
impl Eq for Circle {}

impl Ord for Circle {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => self.radius.partial_cmp(&other.radius).unwrap_or(Ordering::Equal)
        }
    }
}

impl IsND for Circle {
    fn n_dimensions() -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is2D for Circle {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for Circle {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(Circle{ center: Point2D{x: coords[0], y: coords[1]}, radius: Positive::one()}))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(other.get_position(0)?);
        self.center.set_y(other.get_position(1)?);
        Ok(())
    }
}

impl IsBuildable2D for Circle {
    fn new(x: f64, y: f64) -> Box<Self> {
        Box::new(Circle{ center: Point2D{x: x, y: y}, radius: Positive::one()})
    }

    fn from<P>(&mut self, other: P)
        where P: Is2D {

        self.center.from(other)
    }
}

impl IsEditableND for Circle {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.center.set_x(val),
            1 => self.center.set_y(val),
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable2D for Circle {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for Circle {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        let p_min = Point2D{x: self.center.x() - self.radius.get(), y: self.center.y() - self.radius.get()};
        let p_max = Point2D{x: self.center.x() + self.radius.get(), y: self.center.y() + self.radius.get()};
        BoundingBox2D::new(&p_min, &p_max)
    }
}



































//@todo own file since quite a lot of code now
#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Box2D, a box in 2D space
pub struct Box2D {
    pub center: Point2D,
    pub size_x: Positive,
    pub size_y: Positive
}
impl Eq for Box2D {}

impl Ord for Box2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point2D::default();
        match sqr_dist_2d(&origin, &self.center).partial_cmp(&sqr_dist_2d(&origin, &other.center)) {
            Some(x) => x,
            None => match self.size_x.partial_cmp(&other.size_x) {
                Some(x) => x,
                None => self.size_y.partial_cmp(&other.size_y).unwrap_or(Ordering::Equal)
            }
        }
    }
}

impl IsND for Box2D {
    fn n_dimensions() -> usize {
        2
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.center.get_position(dimension)
    }
}

impl Is2D for Box2D {
    fn x(&self) -> f64 {
        self.center.x()
    }

    fn y(&self) -> f64 {
        self.center.y()
    }
}

impl IsBuildableND for Box2D {
    fn new_nd(coords: &Vec<f64>) -> Result<Box<Self>> {
        if coords.len() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }
        Ok(Box::new(Box2D{center: Point2D{x: coords[0], y: coords[1]}, size_x: Positive::one(), size_y: Positive::one()}))
    }

    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND {

        if P::n_dimensions() != 2 {
            return Err(ErrorKind::DimensionsDontMatch);
        }

        self.center.set_x(other.get_position(0)?);
        self.center.set_y(other.get_position(1)?);
        Ok(())
    }
}

impl IsBuildable2D for Box2D {
    fn new(x: f64, y: f64) -> Box<Self> {
        Box::new(Box2D{center: Point2D{x: x, y: y}, size_x: Positive::one(), size_y: Positive::one()})
    }

    fn from<P>(&mut self, other: P) where
        P: Is2D {

        self.center.from(other)
    }
}

impl IsEditableND for Box2D {
    fn set_position(&mut self, dimension: usize, val: f64) -> Result<()> {
        match dimension {
            0 => self.center.set_x(val),
            1 => self.center.set_y(val),
            _ => return Err(ErrorKind::DimensionsDontMatch),
        }
        Ok(())
    }
}

impl IsEditable2D for Box2D {
    fn set_x(&mut self, val: f64) {
        self.center.set_x(val);
    }

    fn set_y(&mut self, val: f64) {
        self.center.set_y(val);
    }
}

impl HasBoundingBox2D for Box2D {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        let p_min = Point2D{x: self.center.x() - self.size_x.get() / 2.0, y: self.center.y() - self.size_y.get() / 2.0};
        let p_max = Point2D{x: self.center.x() + self.size_x.get() / 2.0, y: self.center.y() + self.size_y.get() / 2.0};
        BoundingBox2D::new(&p_min, &p_max)
    }
}

impl<T> IsFilter<T> for Box2D where
    T: Is2D {

    fn is_allowed(&self, p: &T) -> bool {
        p.x() >= self.center.x() - self.size_x.get() / 2.0
            && p.x() <= self.center.x() + self.size_x.get() / 2.0
            && p.y() >= self.center.y() - self.size_y.get() / 2.0
            && p.y() <= self.center.y() + self.size_y.get() / 2.0
    }
}

impl From<BoundingBox2D> for Box2D {
    fn from(x: BoundingBox2D) -> Self {
        Box2D{center: x.center_bb(), size_x: x.size_x(), size_y: x.size_y()}
    }
}
