/*
Copyright 2016 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! PointCloud2D, a collection of positions within 2D space

use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

use prelude::*;
use distances_2d::*;
use functions::{sort_vec_2d_x, sort_vec_2d_y};

#[derive (Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// PointCloud2D, a collection of positions within 2D space
pub struct PointCloud2D<P> where
    P: Is2D {

    pub data: Vec<P>
}

impl<P> PointCloud2D<P> where
    P: Is2D {
    /// Creates a new, empty point cloud
    pub fn new() -> PointCloud2D<P> {
        PointCloud2D{data: Vec::new()}
    }
    /// Creates a new, empty point cloud with capacity
    pub fn with_capacity(n: usize) -> PointCloud2D<P> {
        PointCloud2D{data: Vec::with_capacity(n)}
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
    /// Reserves number of vertices
    pub fn reserve_vertices(&mut self, n: usize) {
        self.data.reserve(n)
    }
}

impl<P> PointCloud2D<P> where
    P: IsBuildable2D + Clone {
    /// Creates a new point cloud from an input string
    pub fn parse(text: String) -> Result<PointCloud2D<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud2D::new();
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

impl<P> Index<usize> for PointCloud2D<P> where
    P: Is2D {

    type Output = P;
    fn index(&self, i: usize) -> &P {
        &self.data[i]
    }
}

impl<P> IndexMut<usize> for PointCloud2D<P> where
    P: Is2D {

    fn index_mut(&mut self, i: usize) -> &mut P {
        &mut self.data[i]
    }
}

impl<P> IsRandomAccessible<P> for PointCloud2D<P> where
    P: Is2D {

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<P> IsRandomInsertible<P> for PointCloud2D<P> where
    P: Is2D {

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

impl<P> IsMovable2D for PointCloud2D<P> where
    P: Is2D + IsMovable2D {

    fn move_by(&mut self, x: f64, y: f64) {
        for p in &mut self.data {
            p.move_by(x, y);
        }
    }
}

impl<P> HasBoundingBox2D for PointCloud2D<P>
    where P: Is2D {

    fn bounding_box(&self) -> Result<BoundingBox2D> {
        BoundingBox2D::from_iterator(&self.data)
    }
}

impl<P> HasCenterOfGravity2D for PointCloud2D<P>
    where P: Is2D {

    fn center_of_gravity(&self) -> Result<Point2D> {
        let size = self.len();

        if size < 1 {
            return Err(ErrorKind::TooFewPoints);
        }

        let sizef = size as f64;

        let mut sumx: f64 = 0.0;
        let mut sumy: f64 = 0.0;

        for p in &self.data {
            sumx += p.x();
            sumy += p.y();
        }

        Ok(Point2D{x: sumx / sizef, y: sumy / sizef})
    }
}

impl<P> HasLength for PointCloud2D<P> where
    P: Is2D {

    fn length(&self) -> f64 {
        let mut length : f64 = 0.0;
        if self.data.len() < 2 { return length; }

        for i in 1..self.data.len() {
            length += dist_2d(&self.data[i], &self.data[i-1]);
        }
        length
    }
}

impl<P> IsViewBuildable for PointCloud2D<P> where
    P: Is2D + Clone {

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

impl<P> IsSortableND for PointCloud2D<P> where
    P: Is2D {

    fn n_dimensions() -> usize {
        2
    }

    fn sort_dim(&mut self, dimension: usize) -> Result<()> {
        match dimension {
            0 => { self.sort_x(); Ok(()) }
            1 => { self.sort_y(); Ok(()) }
            _ => Err(ErrorKind::IncorrectDimension)
        }
    }
}

impl<P> IsSortable2D for PointCloud2D<P> where
    P: Is2D {

    fn sort_x(&mut self) {
        sort_vec_2d_x(&mut self.data)
    }

    fn sort_y(&mut self) {
        sort_vec_2d_y(&mut self.data)
    }
}

impl<P> IsMergeable for PointCloud2D<P> where
    P: Is2D + Clone {

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

impl<P> IsScalable for PointCloud2D<P> where
    P : IsEditable2D {

    fn scale(&mut self, factor: Positive) {
        if let Ok(bb) = self.bounding_box() {
            let c = bb.center_bb();
            for p in &mut self.data {
                p.increase_distance_to_by(&c, factor);
            }
        }
    }
}

impl<P> IsMatrix3Transformable for PointCloud2D<P> where
    P: Is2D + IsMatrix3Transformable + Clone {

    fn transformed(&self, m: &Matrix3) -> Self {
        let mut new = self.clone();
        new.transform(m);
        new
    }

    fn transform(&mut self, m: &Matrix3) {
        for p in &mut self.data {
            p.transform(m);
        }
    }
}

impl<P> Default for PointCloud2D<P> where //https://github.com/rust-lang/rust/issues/26925
    P: Is2D {

    fn default() -> Self {
        let data = Vec::new();
        PointCloud2D{data}
    }
}

impl<P> fmt::Display for PointCloud2D<P> where
    P: Is2D + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.data {
            p.fmt(f)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}
