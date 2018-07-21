/*
Copyright 2018 Martin Buck
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

//! Polygon2D, a polygon within 2D space

use std::fmt;

use prelude::*;
use distances_2d::*;

#[derive (Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Polygon2D, a polygon within 2D space
pub struct Polygon2D<P> where
    P: Is2D {

    pc: PointCloud2D<P>
}

impl<P> IsPolygon<P> for Polygon2D<P> where
    P: Is2D + Clone {

    fn num_segments(&self) -> usize {
        self.pc.len()
    }

    fn segment_vertex_ids(&self, segmentid: SId) -> Result<(VId, VId)> {
        if segmentid.val >= self.pc.len() {
            Err(ErrorKind::IncorrectSegmentID)
        } else if segmentid.val == self.pc.len() -1 {
            Ok((VId{val: segmentid.val}, VId{val: 0}))
        } else {
            Ok((VId{val: segmentid.val}, VId{val: segmentid.val + 1}))
        }
    }

    fn segment_vertices(&self, segmentid: SId) -> Result<(P, P)> {
        let (vid1, vid2) = self.segment_vertex_ids(segmentid)?;
        Ok((self.pc[vid1.val].clone(), self.pc[vid2.val].clone()))
    }

    fn vertex(&self, vertexid: VId) -> Result<P> {
        if vertexid.val >= self.pc.len() {
            Err(ErrorKind::IncorrectVertexID)
        } else {
            Ok(self.pc[vertexid.val].clone())
        }
    }
}

impl<P> IsEditablePolygon<P> for Polygon2D<P> where
    P: Is2D + Clone {

    fn add_vertex(&mut self, vertex: P) -> VId {
        self.pc.data.push(vertex);
        VId{val: self.pc.len() - 1}
    }

    fn change_vertex(&mut self, vertexid: VId, vertex: P) -> Result<()> {
        if vertexid.val >= self.pc.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }

        self.pc[vertexid.val] = vertex;
        Ok(())
    }
}

impl<P> IsMovable2D for Polygon2D<P> where
    P: Is2D + IsMovable2D {

    fn move_by(&mut self, x: f64, y: f64) {
        self.pc.move_by(x, y)
    }
}

impl<P> HasBoundingBox2D for Polygon2D<P>
    where P: Is2D {

    fn bounding_box(&self) -> Result<BoundingBox2D> {
        self.pc.bounding_box()
    }
}

impl<P> HasCenterOfGravity2D for Polygon2D<P>
    where P: Is2D {

    fn center_of_gravity(&self) -> Result<Point2D> {
        self.pc.center_of_gravity()
    }
}

impl<P> HasLength for Polygon2D<P> where
    P: Is2D {

    fn length(&self) -> f64 {
        let mut length = self.pc.length();

        if self.pc.data.len() > 0 {
            length += dist_2d(&self.pc.data[self.pc.data.len()-1], &self.pc.data[0]);
        }

        length
    }
}

impl<P> IsScalable for Polygon2D<P> where
    P : IsEditable2D {

    fn scale(&mut self, factor: Positive) {
        self.pc.scale(factor)
    }
}

impl<P> fmt::Display for Polygon2D<P> where
    P: Is2D + fmt::Display {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pc.fmt(f) //@todo consider output similar to Line2D
    }
}

impl<P> From<PointCloud2D<P>> for Polygon2D<P> where
    P: Is2D {

    fn from(pc: PointCloud2D<P>) -> Self {
        Polygon2D{pc}
    }
}
