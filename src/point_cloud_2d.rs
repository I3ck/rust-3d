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

//! PointCloud2D, a collection of positions within 2D space

use std::fmt;
use std::ops::Index;
use std::ops::IndexMut;

use result::*;
use traits::is_2d::*;
use traits::is_random_accessible::*;
use traits::is_random_insertible::*;
use traits::is_movable_2d::*;
use traits::is_buildable_2d::*;
use traits::has_bounding_box_2d::*;
use traits::has_center_of_gravity_2d::*;
use traits::has_length::*;
use traits::is_view_buildable::*;
use traits::is_sortable_nd::*;
use traits::is_sortable_2d::*;
use traits::is_mergeable::*;
use point_2d::*;
use bounding_box_2d::*;
use distances_2d::*;
use functions::{sort_vec_2d_x, sort_vec_2d_y};
use view::*;

#[derive (Default)]
/// PointCloud2D, a collection of positions within 2D space
pub struct PointCloud2D<P> where
    P: Is2D {

    pub data: Vec<Box<P>>
}

impl<P> PointCloud2D<P> where
    P: Is2D {
    /// Creates a new, empty point cloud
    pub fn new() -> PointCloud2D<P> {
        PointCloud2D{data: Vec::new()}
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
            f(&mut **p);
        }
    }
}

impl<P> PointCloud2D<P> where
    P: IsBuildable2D + Clone {

    /// Creates a new point cloud from an input string
    pub fn parse(text: String) -> Result<PointCloud2D<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud2D::new();
        for line in lines {
            P::parse(String::from(line)).map(|p| pc.push(*p))?;
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
            self.data.push(Box::new(ra[i].clone()));
        }
    }
}

impl<P> Clone for PointCloud2D<P> where
    P: Is2D + Clone {

    fn clone(&self) -> Self {
        let mut data: Vec<Box<P>>;
        data = Vec::new();

        for p in &self.data {
            data.push(p.clone());
        }

        PointCloud2D { data: data }
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

        Ok(*Point2D::new(
            (sumx / sizef),
            (sumy / sizef)
        ))
    }
}

impl<P> HasLength for PointCloud2D<P> where
    P: Is2D {

    fn length(&self) -> f64 {
        let mut length : f64 = 0.0;
        if self.data.len() < 2 { return length; }

        for i in 1..self.data.len() {
            length += dist_2d(&*self.data[i], &*self.data[i-1]);
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

    fn from_view(&self, view: &View) -> Result<Box<Self>> {
        let mut cloned = self.clone();
        cloned.apply_view(view)?;
        Ok(Box::new(cloned))
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
            self.data.push(Box::new((*p).clone()));
        }
    }

    fn combine(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.consume(other.clone());
        result
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
