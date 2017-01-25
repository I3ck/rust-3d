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

//! Module containing BoundingBox3D, an axis aligned bounding box within 3D space

use result::*;
use point_3d::*;
use traits::is_3d::*;
use traits::has_bounding_box_3d::*;

/// BoundingBox3D, an axis aligned bounding box within 3D space
pub struct BoundingBox3D {
    pub min: Point3D,
    pub max: Point3D
}

impl BoundingBox3D {
    /// Creates a new BoundingBox3D with the given min and max positions
    pub fn new(min: Point3D, max: Point3D) -> BoundingBox3D { //@todo return result and check for correctness within min max, or fix it //@todo also offer other constructor with concrete values (or build from Is2D)
        BoundingBox3D{min: min, max: max}
    }
    /// Creates a new BoundBox3D which contains all the given positions
    pub fn from_iterator<'a, It3D,P>(source: It3D) -> Result<BoundingBox3D> where
        It3D: IntoIterator<Item=&'a Box<P>>,
        P: 'a + Is3D + Sized {

        let mut count = 0;

        let mut minx : f64 = 0.0;
        let mut miny : f64 = 0.0;
        let mut minz : f64 = 0.0;
        let mut maxx : f64 = 0.0;
        let mut maxy : f64 = 0.0;
        let mut maxz : f64 = 0.0;

        for p in source {
            if count == 0 {
                minx = p.x();
                miny = p.y();
                minz = p.z();
                maxx = p.x();
                maxy = p.y();
                maxz = p.z();
                count += 1;
                continue;
            }
            if p.x() < minx { minx = p.x(); }
            if p.y() < miny { miny = p.y(); }
            if p.z() < minz { minz = p.z(); }
            if p.x() > maxx { maxx = p.x(); }
            if p.y() > maxy { maxy = p.y(); }
            if p.z() > maxz { maxz = p.z(); }
            count += 1;
        }
        if count >= 2 {
            Ok((Self::new(Point3D{x: minx, y: miny, z: minz}, Point3D{x: maxx, y: maxy, z: maxz})))
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
}

impl HasBoundingBox3D for BoundingBox3D {
    fn bounding_box(&self) -> Result<(Point3D, Point3D)> {
        Ok((self.min.clone(), self.max.clone()))
    }
}
