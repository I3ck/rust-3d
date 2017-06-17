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

//! PointCloud3D, a collection of positions within 3D space

use std::fmt;
use std::cmp::Ordering;
use std::ops::Index;
use std::ops::IndexMut;

use result::*;
use traits::is_3d::*;
use traits::is_random_accessible::*;
use traits::is_random_insertible::*;
use traits::is_moveable_3d::*;
use traits::is_buildable_3d::*;
use traits::has_bounding_box_3d::*;
use traits::has_center_of_gravity_3d::*;
use traits::has_length::*;
use traits::is_view_buildable::*;
use point_3d::{Point3D};
use bounding_box_3d::*;
use functions::dist_3d;
use view::*;

/// PointCloud3D, a collection of positions within 3D space
pub struct PointCloud3D<P> where
    P: Is3D {

    pub data: Vec<Box<P>>
}

impl<P> PointCloud3D<P> where
    P: Is3D {
    /// Creates a new, empty point cloud
    pub fn new() -> PointCloud3D<P> {
        PointCloud3D{data: Vec::new()}
    }

    /// Serializes the point cloud
    pub fn to_str(&self) -> String {
        let mut result = String::new();
        for p in &self.data {
            result = result + &p.to_str() + "\n";
        }
        result
    }

    //@todo remove (is in random access trait)
    /// Pushes a new position to the end of the point cloud
    pub fn push(&mut self, p: P) {
        self.data.push(Box::new(p));
    }

    //@todo remove (is in random access trait)
    /// Returns the length / number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Sorts all positions within the point cloud by x
    pub fn sort_x(&mut self) {
        self.data.sort_by(|a, b| a.x().partial_cmp(&b.x()).unwrap_or(Ordering::Equal));
    }

    /// Sorts all positions within the point cloud by y
    pub fn sort_y(&mut self) {
        self.data.sort_by(|a, b| a.y().partial_cmp(&b.y()).unwrap_or(Ordering::Equal));
    }

    /// Sorts all positions within the point cloud by z
    pub fn sort_z(&mut self) {
        self.data.sort_by(|a, b| a.z().partial_cmp(&b.z()).unwrap_or(Ordering::Equal));
    }

    /// Applies a function to each position
    pub fn for_each_point<F>(&mut self, mut f: F) where
        F: FnMut(&mut P) {

        for p in &mut self.data {
            f(&mut **p);
        }
    }
}

impl<P> PointCloud3D<P> where
    P: Is3D + Clone {

    /// Creates a copy of the point cloud
    pub fn clone(&self) -> PointCloud3D<P> {
        let mut data: Vec<Box<P>>;
        data = Vec::new();

        for p in &self.data {
            data.push(p.clone());
        }

        PointCloud3D { data: data }
    }

    /// Appends all values of other behind this
    pub fn consume(&mut self, other: Self) {
        for p in other.data {
            self.data.push(Box::new((*p).clone()));
        }
    }
}

impl<P> PointCloud3D<P> where
    P: IsBuildable3D + Clone {

    /// Creates a new point cloud from an input string
    pub fn parse(text: String) -> Result<PointCloud3D<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud3D::new();
        for line in lines {
            P::parse(String::from(line)).map(|p| pc.push(*p))?;
        }
        if pc.len() == 0 { return Err(ErrorKind::ParseError); }
        Ok(pc)
    }
}

impl<P> Index<usize> for PointCloud3D<P> where
    P: Is3D {

    type Output = P;
    fn index(&self, i: usize) -> &P {
        &self.data[i]
    }
}

impl<P> IndexMut<usize> for PointCloud3D<P> where
    P: Is3D {

    fn index_mut(&mut self, i: usize) -> &mut P {
        &mut self.data[i]
    }
}

impl<P> IsRandomAccessible<P> for PointCloud3D<P> where
    P: Is3D {

    fn len(&self) -> usize {
        self.len()
    }
}


impl<P> IsRandomInsertible<P> for PointCloud3D<P> where
    P: Is3D {

    fn push(&mut self, point: P) {
        self.data.push(Box::new(point))
    }

    fn insert(&mut self, index: usize, point: P) -> Result<()> {
        if index > self.len() {
            Err(ErrorKind::IncorrectVertexID)
        } else {
            self.data.insert(index, Box::new(point));
            Ok(())
        }
    }
}

impl<P> IsMoveable3D for PointCloud3D<P> where
    P: Is3D + IsMoveable3D {

    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

impl<P> HasBoundingBox3D for PointCloud3D<P> where
    P: Is3D {

    fn bounding_box(&self) -> Result<BoundingBox3D> {
        BoundingBox3D::from_iterator(&self.data)
    }
}

impl<P> HasCenterOfGravity3D for PointCloud3D<P>
    where P: Is3D {

    fn center_of_gravity(&self) -> Result<Point3D> {
        let size = self.len();

        if size < 1 {
            return Err(ErrorKind::TooFewPoints);
        }

        let sizef = size as f64;

        let mut sumx: f64 = 0.0;
        let mut sumy: f64 = 0.0;
        let mut sumz: f64 = 0.0;

        for p in &self.data {
            sumx += p.x();
            sumy += p.y();
            sumz += p.z();
        }

        Ok(*Point3D::build(
            (sumx / sizef),
            (sumy / sizef),
            (sumz / sizef)
        ))
    }
}

impl<P> HasLength for PointCloud3D<P> where
    P: Is3D {

    fn length(&self) -> f64 {
        let mut length : f64 = 0.0;
        if self.data.len() < 2 { return length; }

        for i in 1..self.data.len() {
            length += dist_3d(&*self.data[i], &*self.data[i-1]);
        }
        length
    }
}

impl<P> IsViewBuildable for PointCloud3D<P> where
    P: Is3D + Clone {

    fn apply_view(&mut self, view: &View) -> Result<()> {
        self.data.apply_view(view)?;
        Ok(())
    }

    fn from_view(&self, view: &View) -> Result<Box<Self>> {
        let mut cloned = self.clone();
        cloned.apply_view(view)?;
        Ok(Box::new(cloned))
    }
}

impl<P> fmt::Display for PointCloud3D<P> where
    P: Is3D + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.data {
            p.fmt(f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}
