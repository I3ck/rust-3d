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

#![deny(warnings)]

//! 3D/2D library written in Rust.
//! Offering useful containers, structures and algorithms for 2D and 3D space.
//! Meant as basis for numeric algorithms, viewers, game engines, ...

pub mod prelude;
pub mod strong_types;
pub mod traits;
pub mod impls;
pub mod io;
pub mod filters;
pub mod algorithms;
pub mod functions;
pub mod factory_2d;
pub mod interpolation_2d;
pub mod distances_2d;
pub mod distances_3d;
pub mod distances_nd;
pub mod test_helper;

mod           point_2d;
pub use self::point_2d::Point2D;

mod           point_3d;
pub use self::point_3d::Point3D;

mod           line_2d;
pub use self::line_2d::Line2D;

mod           line_3d;
pub use self::line_3d::Line3D;

mod           line_segment_2d;
pub use self::line_segment_2d::LineSegment2D;

mod           line_segment_3d;
pub use self::line_segment_3d::LineSegment3D;

mod           ray_2d;
pub use self::ray_2d::Ray2D;

mod           ray_3d;
pub use self::ray_3d::Ray3D;

mod           plane_3d;
pub use self::plane_3d::Plane3D;

mod           point_cloud_2d;
pub use self::point_cloud_2d::PointCloud2D;

mod           point_cloud_3d;
pub use self::point_cloud_3d::PointCloud3D;

mod           norm_2d;
pub use self::norm_2d::Norm2D;

mod           norm_3d;
pub use self::norm_3d::Norm3D;

mod           bounding_box_2d;
pub use self::bounding_box_2d::BoundingBox2D;

mod           bounding_box_3d;
pub use self::bounding_box_3d::BoundingBox3D;

mod           matrix4;
pub use self::matrix4::Matrix4;

mod           matrix4_pipe;
pub use self::matrix4_pipe::Matrix4Pipe;

mod           compressed_point_3d;
pub use self::compressed_point_3d::CompressedPoint3D;

mod           compressed_point_cloud_3d;
pub use self::compressed_point_cloud_3d::CompressedPointCloud3D;

mod           projection_to_plane;
//pub use self::projection_to_plane::ProjectionToPlane;

mod           kd_tree;
pub use self::kd_tree::KdTree;

mod           mesh_3d;
pub use self::mesh_3d::Mesh3D;

mod           searchable_mesh;
pub use self::searchable_mesh::SearchableMesh;

mod           oc_node;
pub use self::oc_node::OcNode;

mod           oc_tree;
pub use self::oc_tree::OcTree;

mod           view;
pub use self::view::View;

mod           positive;
pub use self::positive::Positive;

mod           non_negative;
pub use self::non_negative::NonNegative;

mod           result;
pub use self::result::Result;

mod           face3;
pub use self::face3::Face3;

mod           half_edge;
pub use self::half_edge::HalfEdge;


mod utils;
