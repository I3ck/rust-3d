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

pub use crate::{algorithms::*, interpolation_2d::*, result::*, strong_types::*, traits::*};

pub use crate::AABBTree2D;

pub use crate::Cluster;

pub use crate::SATCollider;

pub use crate::{NonNegative, Positive};

pub use crate::{BoundingBox2D, BoundingBox3D};

pub use crate::Face3;

pub use crate::{Line2D, Line3D};

pub use crate::{LineSegment2D, LineSegment3D};

pub use crate::{Ray2D, Ray3D};

pub use crate::{Norm2D, Norm3D};

pub use crate::{Point2D, Point3D};

pub use crate::{Matrix3, Matrix4};

pub use crate::OcTree;

pub use crate::Mesh3D;

pub use crate::{PointCloud2D, PointCloud3D};

pub use crate::{Polygon2D, Polygon3D};

pub use crate::View;

pub use crate::HalfEdge;

pub use crate::{Box2D, Circle};

pub use crate::{Box3D, Sphere};

pub use crate::Precision;

pub use crate::filters::{FilterBox2D, FilterBox3D, FilterCircle, FilterOutlier3D, FilterSphere};

pub use crate::filters::{combinators::*, transformers::*};

pub use crate::BoxUnaligned3D;

pub use crate::TriFace3D;

pub use crate::Collider3D;
