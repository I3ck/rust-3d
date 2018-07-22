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

//! BoundingBox2D, an axis aligned bounding box within 2D space

use prelude::*;

#[derive (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// BoundingBox2D, an axis aligned bounding box within 2D space
pub struct BoundingBox2D {
    min: Point2D,
    max: Point2D
}

impl BoundingBox2D {
    /// Creates a new BoundingBox2D with the given min and max positions
    pub fn new<P1, P2>(min: &P1, max: &P2) -> Result<BoundingBox2D> where
        P1: Is2D,
        P2: Is2D {

        if min.x() == max.x() || min.y() == max.y() {
            Err(ErrorKind::MinMaxEqual)
        } else if min.x() > max.x() || min.y() > max.y() {
            Err(ErrorKind::MinMaxSwapped)
        } else {
            Ok(BoundingBox2D{min: Point2D{x: min.x(), y: min.y()}, max: Point2D{x: max.x(), y: max.y()}})
        }
    }
    /// Creates a new BoundBox2D which contains all the given positions
    pub fn from_iterator<'a, It2D,P>(source: It2D) -> Result<BoundingBox2D> where
        It2D: IntoIterator<Item=&'a P>,
        P: 'a + Is2D + Sized {

        let mut count = 0;

        let mut minx : f64 = 0.0;
        let mut miny : f64 = 0.0;
        let mut maxx : f64 = 0.0;
        let mut maxy : f64 = 0.0;

        for p in source {
            if count == 0 {
                minx = p.x();
                miny = p.y();
                maxx = p.x();
                maxy = p.y();
                count += 1;
                continue;
            }
            if p.x() < minx { minx = p.x(); }
            if p.y() < miny { miny = p.y(); }
            if p.x() > maxx { maxx = p.x(); }
            if p.y() > maxy { maxy = p.y(); }
            count += 1;
        }
        if count >= 2 {
            Self::new(&Point2D{x: minx, y: miny}, &Point2D{x: maxx, y: maxy})
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
    /// Returns the minimum position of the bounding box
    pub fn min_p(&self) -> Point2D {
        self.min.clone()
    }
    /// Returns the maximum position of the bounding box
    pub fn max_p(&self) -> Point2D {
        self.max.clone()
    }
    /// Returns the size the bounding box within the x-dimension
    pub fn size_x(&self) -> Positive {
        Positive::new((self.max.x() - self.min.x()).abs()).unwrap() //safe since constrain enforced on construction
    }
    /// Returns the size the bounding box within the y-dimension
    pub fn size_y(&self) -> Positive {
        Positive::new((self.max.y() - self.min.y()).abs()).unwrap() //safe since constrain enforced on construction
    }
    /// Returns the center of the bounding box
    pub fn center_bb(&self) -> Point2D {
        Point2D{x: self.min.x() + (self.max.x() - self.min.x()) / 2.0,
                y: self.min.y() + (self.max.y() - self.min.y()) / 2.0}
    }
    /// Tests whether this bounding box is within the other
    pub fn is_inside(&self, other: &BoundingBox2D) -> bool {
           self.min.x() > other.min.x()
        && self.min.y() > other.min.y()
        && self.max.x() < other.max.x()
        && self.max.y() < other.max.y()
    }
    /// Tests whether this bounding box contains a position
    pub fn contains<P>(&self, other: &P) -> bool where
        Self: Sized, P: Is2D {

           other.x() > self.min.x()
        && other.x() < self.max.x()
        && other.y() > self.min.y()
        && other.y() < self.max.y()
    }
    /// Tests whether this bounding box contains the other
    pub fn has_inside(&self, other: &BoundingBox2D) -> bool {
           self.min.x() < other.min.x()
        && self.min.y() < other.min.y()
        && self.max.x() > other.max.x()
        && self.max.y() > other.max.y()
    }
    /// Tests whether this bounding box and the other overlap in any way
    pub fn collides_with(&self, other: &BoundingBox2D) -> bool {
           2.0 * self.center_bb().x - other.center_bb().x < ((self.size_x() + other.size_x()).get())
        && 2.0 * self.center_bb().y - other.center_bb().y < ((self.size_y() + other.size_y()).get())
    }
}

impl Default for BoundingBox2D {
    fn default() -> Self {
        BoundingBox2D {min: Point2D {x: -0.5, y: -0.5}, max: Point2D {x: 0.5, y: 0.5}}
    }
}

impl HasBoundingBox2D for BoundingBox2D {
    fn bounding_box(&self) -> Result<BoundingBox2D> {
        BoundingBox2D::new(&self.min, &self.max)
    }
}

impl HasDistanceTo<BoundingBox2D> for BoundingBox2D {
    fn sqr_distance(&self, other: &BoundingBox2D) -> NonNegative {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if other.max_p().x() < self.min_p().x() {
            dx = other.max_p().x() - self.min_p().x();
        }
        else if other.min_p().x() > self.max_p().x() {
            dx = other.min_p().x() - self.max_p().x();
        }

        if other.max_p().y() < self.min_p().y() {
            dy = other.max_p().y() - self.min_p().y();
        }
        else if other.min_p().y() > self.max_p().y() {
            dy = other.min_p().y() - self.max_p().y();
        }

        NonNegative::new(dx*dx + dy*dy).unwrap()
    }
}

impl IsScalable for BoundingBox2D {
    fn scale(&mut self, factor: Positive) {
        let c = self.center_bb();
        let min_x = c.x - (0.5 * factor.get() * self.size_x().get());
        let max_x = c.x + (0.5 * factor.get() * self.size_x().get());
        let min_y = c.y - (0.5 * factor.get() * self.size_y().get());
        let max_y = c.y + (0.5 * factor.get() * self.size_y().get());

        self.min.set_pos(min_x, min_y);
        self.max.set_pos(max_x, max_y);
    }
}

impl IsMergeable for BoundingBox2D {
    fn consume(&mut self, other: Self) {
        let (mut min_x, mut min_y) = (self.min.x(), self.min.y());
        let (mut max_x, mut max_y) = (self.max.x(), self.max.y());

        if other.min.x() < min_x { min_x = other.min.x() }
        if other.min.y() < min_y { min_y = other.min.y() }

        if other.max.x() < max_x { max_x = other.max.x() }
        if other.max.y() < max_y { max_y = other.max.y() }

        self.min.set_pos(min_x, min_y);
        self.max.set_pos(max_x, max_y);
    }

    fn combine(&self, other: &Self) -> Self {
        let (mut min_x, mut min_y) = (self.min.x(), self.min.y());
        let (mut max_x, mut max_y) = (self.max.x(), self.max.y());

        if other.min.x() < min_x { min_x = other.min.x() }
        if other.min.y() < min_y { min_y = other.min.y() }

        if other.max.x() < max_x { max_x = other.max.x() }
        if other.max.y() < max_y { max_y = other.max.y() }

        let min = Point2D::new(min_x, min_y);
        let max = Point2D::new(max_x, max_y);

        BoundingBox2D{min, max}
    }
}
