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

//! Module containing BoundingBox2D, an axis aligned bounding box within 2D space

use result::*;
use point_2d::*;
use traits::is_2d::*;
use traits::has_bounding_box_2d::*;

/// BoundingBox2D, an axis aligned bounding box within 2D space
pub struct BoundingBox2D {
    pub min: Point2D,
    pub max: Point2D
}

impl BoundingBox2D {
    /// Creates a new BoundingBox2D with the given min and max positions
    pub fn new(min: Point2D, max: Point2D) -> BoundingBox2D { //@todo return result and check for correctness within min max, or fix it //@todo also offer other constructor with concrete values (or build from Is2D)
        BoundingBox2D{min: min, max: max}
    }
    /// Creates a new BoundBox2D which contains all the given positions
    pub fn from_iterator<'a, It2D,P>(source: It2D) -> Result<BoundingBox2D> where
        It2D: IntoIterator<Item=&'a Box<P>>,
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
            Ok((Self::new(Point2D{x: minx, y: miny}, Point2D{x: maxx, y: maxy})))
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
}

impl HasBoundingBox2D for BoundingBox2D {
    fn bounding_box(&self) -> Result<(Point2D, Point2D)> {
        Ok((self.min.clone(), self.max.clone()))
    }
}
