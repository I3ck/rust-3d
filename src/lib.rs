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

pub mod strong_types;
pub mod traits;
pub mod impls;
pub mod io;
pub mod filters;
pub mod algorithms;
pub mod functions;
pub mod point_2d;
pub mod point_3d;
pub mod line_2d;
pub mod line_3d;
pub mod line_segment_2d;
pub mod line_segment_3d;
pub mod ray_2d;
pub mod ray_3d;
pub mod plane_3d;
pub mod point_cloud_2d;
pub mod point_cloud_3d;
pub mod norm_2d;
pub mod norm_3d;
pub mod bounding_box_2d;
pub mod bounding_box_3d;
pub mod matrix4;
pub mod matrix4_pipe;
pub mod compressed_point_3d;
pub mod compressed_point_cloud_3d;
pub mod projection_to_plane;
pub mod kd_tree;
pub mod mesh_3d;
pub mod searchable_mesh;
pub mod oc_node;
pub mod oc_tree;
pub mod factory_2d;
pub mod interpolation_2d;
pub mod view;
pub mod positive;
pub mod test_helper;
pub mod result;
pub mod distances_2d;
pub mod distances_3d;
pub mod distances_nd;
pub mod face3;
pub mod half_edge;
pub mod prelude;

mod utils;
