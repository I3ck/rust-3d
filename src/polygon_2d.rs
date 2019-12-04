/*
Copyright 2018 Martin Buck

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

//! Polygon2D, a polygon within 2D space

use std::fmt;

use crate::{distances_2d::*, prelude::*};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// Polygon2D, a polygon within 2D space
pub struct Polygon2D<P>
where
    P: Is2D,
{
    pc: PointCloud2D<P>,
}

impl<P> IsPolygon<P> for Polygon2D<P>
where
    P: Is2D + Clone,
{
    fn num_segments(&self) -> usize {
        self.pc.len()
    }

    fn segment_vertex_ids(&self, segmentid: SId) -> Result<(VId, VId)> {
        if segmentid.val >= self.pc.len() {
            Err(ErrorKind::IncorrectSegmentID)
        } else if segmentid.val == self.pc.len() - 1 {
            Ok((VId { val: segmentid.val }, VId { val: 0 }))
        } else {
            Ok((
                VId { val: segmentid.val },
                VId {
                    val: segmentid.val + 1,
                },
            ))
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

impl<P> IsEditablePolygon<P> for Polygon2D<P>
where
    P: Is2D + Clone,
{
    fn add_vertex(&mut self, vertex: P) -> VId {
        self.pc.data.push(vertex);
        VId {
            val: self.pc.len() - 1,
        }
    }

    fn change_vertex(&mut self, vertexid: VId, vertex: P) -> Result<()> {
        if vertexid.val >= self.pc.len() {
            return Err(ErrorKind::IncorrectVertexID);
        }

        self.pc[vertexid.val] = vertex;
        Ok(())
    }
}

impl<P> IsMovable2D for Polygon2D<P>
where
    P: Is2D + IsMovable2D,
{
    fn move_by(&mut self, x: f64, y: f64) {
        self.pc.move_by(x, y)
    }
}

impl<P> HasBoundingBox2DMaybe for Polygon2D<P>
where
    P: Is2D,
{
    fn bounding_box_maybe(&self) -> Result<BoundingBox2D> {
        self.pc.bounding_box_maybe()
    }
}

impl<P> HasCenterOfGravity2D for Polygon2D<P>
where
    P: Is2D,
{
    fn center_of_gravity(&self) -> Result<Point2D> {
        self.pc.center_of_gravity()
    }
}

impl<P> HasLength for Polygon2D<P>
where
    P: Is2D,
{
    fn length(&self) -> f64 {
        let mut length = self.pc.length();

        if self.pc.data.len() > 0 {
            length += dist_2d(&self.pc.data[self.pc.data.len() - 1], &self.pc.data[0]);
        }

        length
    }
}

impl<P> IsScalable for Polygon2D<P>
where
    P: IsEditable2D,
{
    fn scale(&mut self, factor: Positive) {
        self.pc.scale(factor)
    }
}

impl<P> IsMatrix3Transformable for Polygon2D<P>
where
    P: Is2D + IsMatrix3Transformable + Clone,
{
    fn transformed(&self, m: &Matrix3) -> Self {
        let mut new = self.clone();
        new.transform(m);
        new
    }

    fn transform(&mut self, m: &Matrix3) {
        self.pc.transform(m);
    }
}

impl<P> fmt::Display for Polygon2D<P>
where
    P: Is2D + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pc.fmt(f)
    }
}

impl<P> Default for Polygon2D<P>
where
    //https://github.com/rust-lang/rust/issues/26925
    P: Is2D,
{
    fn default() -> Self {
        let pc = PointCloud2D::default();
        Polygon2D { pc }
    }
}

impl<P> From<PointCloud2D<P>> for Polygon2D<P>
where
    P: Is2D,
{
    fn from(pc: PointCloud2D<P>) -> Self {
        Polygon2D { pc }
    }
}
