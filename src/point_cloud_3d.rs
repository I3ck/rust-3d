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
use std::ops::Index;
use std::ops::IndexMut;

use prelude::*;
use distances_3d::*;
use functions::{sort_vec_3d_x, sort_vec_3d_y, sort_vec_3d_z};

#[derive (Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// PointCloud3D, a collection of positions within 3D space
pub struct PointCloud3D<P> where
    P: Is3D {

    pub data: Vec<P>
}

impl<P> PointCloud3D<P> where
    P: Is3D {
    /// Creates a new, empty point cloud
    pub fn new() -> PointCloud3D<P> {
        PointCloud3D{data: Vec::new()}
    }
    /// Creates a new, empty point cloud with capacity
    pub fn with_capacity(n: usize) -> PointCloud3D<P> {
        PointCloud3D{data: Vec::with_capacity(n)}
    }
    /// Serializes the point cloud
    pub fn to_str(&self) -> String {
        let mut result = String::new();
        for p in &self.data {
            result = result + &p.to_str() + "\n";
        }
        result
    }
    /// Applies a function to each position
    pub fn for_each_point<F>(&mut self, mut f: F) where
        F: FnMut(&mut P) {

        for p in &mut self.data {
            f(&mut *p);
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
            P::parse(String::from(line)).map(|p| pc.push(p))?;
        }
        if pc.len() == 0 { return Err(ErrorKind::ParseError); }
        Ok(pc)
    }
    //@todo make trait?
    /// Appends all elements of an IsRandomAccessible
    pub fn append_ra<RA>(&mut self, ra: &RA) where
        RA: IsRandomAccessible<P> {

        let n = ra.len();
        self.data.reserve(n);

        for i in 0..n {
            self.data.push(ra[i].clone());
        }
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
        self.data.len()
    }
}


impl<P> IsRandomInsertible<P> for PointCloud3D<P> where
    P: Is3D {

    fn push(&mut self, point: P) {
        self.data.push(point)
    }

    fn insert(&mut self, index: usize, point: P) -> Result<()> {
        if index > self.len() {
            Err(ErrorKind::IncorrectVertexID)
        } else {
            self.data.insert(index, point);
            Ok(())
        }
    }
}

impl<P> IsMovable3D for PointCloud3D<P> where
    P: Is3D + IsMovable3D {

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

        Ok(Point3D{x: sumx / sizef, y: sumy / sizef, z: sumz / sizef})
    }
}

impl<P> HasLength for PointCloud3D<P> where
    P: Is3D {

    fn length(&self) -> f64 {
        let mut length : f64 = 0.0;
        if self.data.len() < 2 { return length; }

        for i in 1..self.data.len() {
            length += dist_3d(&self.data[i], &self.data[i-1]);
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

    fn from_view(&self, view: &View) -> Result<Self> {
        let mut cloned = self.clone();
        cloned.apply_view(view)?;
        Ok(cloned)
    }
}

impl<P> IsSortableND for PointCloud3D<P> where
    P: Is3D {

    fn n_dimensions() -> usize {
        3
    }

    fn sort_dim(&mut self, dimension: usize) -> Result<()> {
        match dimension {
            0 => { self.sort_x(); Ok(()) }
            1 => { self.sort_y(); Ok(()) }
            2 => { self.sort_z(); Ok(()) }
            _ => Err(ErrorKind::IncorrectDimension)
        }
    }
}

impl<P> IsSortable3D for PointCloud3D<P> where
    P: Is3D {

    fn sort_x(&mut self) {
        sort_vec_3d_x(&mut self.data);
    }

    fn sort_y(&mut self) {
        sort_vec_3d_y(&mut self.data);
    }

    fn sort_z(&mut self) {
        sort_vec_3d_z(&mut self.data);
    }
}

impl<P> IsMergeable for PointCloud3D<P> where
    P: Is3D + Clone {

    fn consume(&mut self, other: Self) {
        for p in other.data {
            self.data.push(p.clone());
        }
    }

    fn combine(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.consume(other.clone());
        result
    }
}

impl<P> IsScalable for PointCloud3D<P> where
    P : IsEditable3D {
    
    fn scale(&mut self, factor: Positive) {
        if let Ok(bb) = self.bounding_box() {
            let c = bb.center_bb();
            for p in &mut self.data {
                p.increase_distance_to_by(&c, factor);
            }
        }
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
