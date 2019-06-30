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

pub use result::*;
pub use strong_types::*;
pub use interpolation_2d::*;
pub use traits::*;
pub use algorithms::*;

pub use AABBTree2D;

pub use Positive;
pub use NonNegative;

pub use BoundingBox2D;
pub use BoundingBox3D;

pub use Face3;

pub use Line2D;
pub use Line3D;

pub use LineSegment2D;
pub use LineSegment3D;

pub use Ray2D;
pub use Ray3D;

pub use Norm2D;
pub use Norm3D;

pub use Point2D;
pub use Point3D;

pub use Matrix3;
pub use Matrix4;

pub use OcTree;

pub use Mesh3D;

pub use PointCloud2D;
pub use PointCloud3D;

pub use Polygon2D;
pub use Polygon3D;

pub use View;

pub use HalfEdge;

pub use Circle;
pub use Box2D;

pub use Sphere;
pub use Box3D;

pub use Precision;

pub use filters::FilterBox2D;
pub use filters::FilterBox3D;
pub use filters::FilterCircle;
pub use filters::FilterSphere;
pub use filters::FilterOutlier3D;

pub use filters::combinators::*;
pub use filters::transformers::*;
