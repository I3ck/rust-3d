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

//! BoundingBox3D, an axis aligned bounding box within 3D space

use prelude::*;

#[derive (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// BoundingBox3D, an axis aligned bounding box within 3D space
pub struct BoundingBox3D {
    min: Point3D,
    max: Point3D
}

impl BoundingBox3D {
    /// Creates a new BoundingBox3D with the given min and max positions
    pub fn new<P1, P2>(min: &P1, max: &P2) -> Result<BoundingBox3D> where
        P1: Is3D,
        P2: Is3D {

        if min.x() == max.x() || min.y() == max.y() || min.z() == max.z() {
            Err(ErrorKind::MinMaxEqual)
        } else if min.x() > max.x() || min.y() > max.y() || min.z() > max.z() {
            Err(ErrorKind::MinMaxSwapped)
        } else {
            Ok(BoundingBox3D{min: Point3D{x: min.x(), y: min.y(), z: min.z()}, max: Point3D{x: max.x(), y: max.y(), z: max.z()}})
        }
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
            Self::new(&Point3D{x: minx, y: miny, z: minz}, &Point3D{x: maxx, y: maxy, z: maxz})
        } else {
            Err(ErrorKind::TooFewPoints)
        }
    }
    /// Returns the minimum position of the bounding box
    pub fn min_p(&self) -> Point3D {
        self.min.clone()
    }
    /// Returns the maximum position of the bounding box
    pub fn max_p(&self) -> Point3D {
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
    /// Returns the size the bounding box within the z-dimension
    pub fn size_z(&self) -> Positive {
        Positive::new((self.max.z() - self.min.z()).abs()).unwrap() //safe since constrain enforced on construction
    }
    /// Returns the center of the bounding box
    pub fn center_bb(&self) -> Point3D {
        Point3D{x: self.min.x() + (self.max.x() - self.min.x()) / 2.0,
                y: self.min.y() + (self.max.y() - self.min.y()) / 2.0,
                z: self.min.z() + (self.max.z() - self.min.z()) / 2.0}
    }
    /// Tests whether this bounding box is within the other
    pub fn is_inside(&self, other: &BoundingBox3D) -> bool {
           self.min.x() > other.min.x()
        && self.min.y() > other.min.y()
        && self.min.z() > other.min.z()
        && self.max.x() < other.max.x()
        && self.max.y() < other.max.y()
        && self.max.z() < other.max.z()
    }
    /// Tests whether this bounding box contains a position
    pub fn contains<P>(&self, other: &P) -> bool where
        Self: Sized, P: Is3D {

           other.x() > self.min.x()
        && other.x() < self.max.x()
        && other.y() > self.min.y()
        && other.y() < self.max.y()
        && other.z() > self.min.z()
        && other.z() < self.max.z()
    }
    /// Tests whether this bounding box contains the other
    pub fn has_inside(&self, other: &BoundingBox3D) -> bool {
           self.min.x() < other.min.x()
        && self.min.y() < other.min.y()
        && self.min.z() < other.min.z()
        && self.max.x() > other.max.x()
        && self.max.y() > other.max.y()
        && self.max.z() > other.max.z()
    }
    /// Tests whether this bounding box and the other overlap in any way
    pub fn collides_with(&self, other: &BoundingBox3D) -> bool {
           2.0 * self.center_bb().x - other.center_bb().x < ((self.size_x() + other.size_x()).get())
        && 2.0 * self.center_bb().y - other.center_bb().y < ((self.size_y() + other.size_y()).get())
        && 2.0 * self.center_bb().z - other.center_bb().z < ((self.size_z() + other.size_z()).get())
    }
}

impl HasBoundingBox3D for BoundingBox3D {
    fn bounding_box(&self) -> Result<BoundingBox3D> {
        BoundingBox3D::new(&self.min, &self.max)
    }
}

impl HasDistanceTo<BoundingBox3D> for BoundingBox3D {
    fn sqr_distance(&self, other: &BoundingBox3D) -> NonNegative {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;

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

        if other.max_p().z() < self.min_p().z() {
            dz = other.max_p().z() - self.min_p().z();
        }
        else if other.min_p().z() > self.max_p().z() {
            dz = other.min_p().z() - self.max_p().z();
        }

        NonNegative::new(dx*dx + dy*dy + dz*dz).unwrap()
    }
}
