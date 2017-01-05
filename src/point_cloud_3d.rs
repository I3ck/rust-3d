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

use std::fmt;

use std::cmp::Ordering;

use traits::is_moveable_3d::IsMoveable3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_3d::IsEditable3D;
use traits::has_bounding_box_3d::HasBoundingBox3D;
use point_3d::{Point3D};



pub struct PointCloud3D<P> {
    pub data: Vec<Box<P>>
}

impl<P> PointCloud3D<P> where
    P: IsEditable3D + IsBuildable3D + Clone {

    pub fn new() -> PointCloud3D<P> {
        PointCloud3D{data: Vec::new()}
    }

    pub fn parse(text: String) -> Option<PointCloud3D<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud3D::new();
        for line in lines {
            match P::parse(String::from(line)) { //@todo must be templated too
                Some(p) => pc.push(*p),
                None => {}
            }
        }
        if pc.len() == 0 { return None; }
        Some(pc)
    }

    pub fn to_str(&self) -> String {
        let mut result = String::new();

        for p in &self.data {
            result = result + &p.to_str() + "\n";
        }

        result
    }

    pub fn clone(&self) -> PointCloud3D<P> {
        let mut data: Vec<Box<P>>;
        data = Vec::new();

        for p in &self.data {
            data.push(p.clone());
        }

        PointCloud3D { data: data }
    }

    pub fn push(&mut self, p: P) {
        self.data.push(Box::new(p));
    }

    pub fn consume(&mut self, other: Self) {
        for p in other.data {
            self.data.push(Box::new((*p).clone()));
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn center(&self) -> Option<P> { //@todo missing test //@todo rename to center of gravity
        let size = self.len();

        if size < 1 {
            return None;
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

        return Some(*P::build(
            (sumx / sizef),
            (sumy / sizef),
            (sumz / sizef)
        ))
    }

    pub fn sort_x(&mut self) {
        self.data.sort_by(|a, b| a.x().partial_cmp(&b.x()).unwrap_or(Ordering::Equal));
    }

    pub fn sort_y(&mut self) {
        self.data.sort_by(|a, b| a.y().partial_cmp(&b.y()).unwrap_or(Ordering::Equal));
    }

    pub fn sort_z(&mut self) {
        self.data.sort_by(|a, b| a.z().partial_cmp(&b.z()).unwrap_or(Ordering::Equal));
    }
}

impl<P> IsMoveable3D for PointCloud3D<P> where
    P: IsEditable3D + IsMoveable3D {

    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

impl<P> HasBoundingBox3D for PointCloud3D<P> where
    P: IsEditable3D + IsBuildable3D + Clone {

    fn bounding_box(&self) -> Option<(Point3D, Point3D)> {
        if self.len() < 2 {
            return None;
        }

        let mut minx = self.data[0].x();
        let mut miny = self.data[0].y();
        let mut minz = self.data[0].z();
        let mut maxx = self.data[0].x();
        let mut maxy = self.data[0].y();
        let mut maxz = self.data[0].z();

        for p in &self.data {
            if p.x() < minx { minx = p.x(); }
            if p.y() < miny { miny = p.y(); }
            if p.z() < minz { minz = p.z(); }
            if p.x() > maxx { maxx = p.x(); }
            if p.y() > maxy { maxy = p.y(); }
            if p.z() > maxz { maxz = p.z(); }
        }

        return Some((Point3D{x: minx, y: miny, z: minz}, Point3D{x: maxx, y: maxy, z: maxz}));
    }
}

impl<P> fmt::Display for PointCloud3D<P> where
    P: IsEditable3D + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.data {
            match p.fmt(f) {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
            match f.write_str("\n") {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
        }
        return Ok(());
    }
}
