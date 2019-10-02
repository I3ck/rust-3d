/*
Copyright 2017 Martin Buck

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

#![deny(warnings)]

//! Exporting often used types / traits for convenience

pub use crate::result::*;
pub use crate::strong_types::*;
pub use crate::interpolation_2d::*;
pub use crate::traits::*;
pub use crate::algorithms::*;

pub use crate::AABBTree2D;

pub use crate::Positive;
pub use crate::NonNegative;

pub use crate::BoundingBox2D;
pub use crate::BoundingBox3D;

pub use crate::Face3;

pub use crate::Line2D;
pub use crate::Line3D;

pub use crate::LineSegment2D;
pub use crate::LineSegment3D;

pub use crate::Ray2D;
pub use crate::Ray3D;

pub use crate::Norm2D;
pub use crate::Norm3D;

pub use crate::Point2D;
pub use crate::Point3D;

pub use crate::Matrix3;
pub use crate::Matrix4;

pub use crate::OcTree;

pub use crate::Mesh3D;

pub use crate::PointCloud2D;
pub use crate::PointCloud3D;

pub use crate::Polygon2D;
pub use crate::Polygon3D;

pub use crate::View;

pub use crate::HalfEdge;

pub use crate::Circle;
pub use crate::Box2D;

pub use crate::Sphere;
pub use crate::Box3D;

pub use crate::Precision;

pub use crate::filters::FilterBox2D;
pub use crate::filters::FilterBox3D;
pub use crate::filters::FilterCircle;
pub use crate::filters::FilterSphere;
pub use crate::filters::FilterOutlier3D;

pub use crate::filters::combinators::*;
pub use crate::filters::transformers::*;
