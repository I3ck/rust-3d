/*
Copyright 2017 Martin Buck
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

//! Exporting often used types / traits for convenience

pub use result::*;
pub use strong_types::*;
pub use positive::*;

pub use bounding_box_2d::*;
pub use bounding_box_3d::*;

pub use face3::*;

pub use line_2d::*;
pub use line_3d::*;

pub use line_segment_2d::*;
pub use line_segment_3d::*;

pub use ray_2d::*;
pub use ray_3d::*;

pub use norm_2d::*;
pub use norm_3d::*;

pub use point_2d::*;
pub use point_3d::*;

pub use matrix4::*;

pub use oc_node::*;
pub use oc_tree::*;

pub use mesh_3d::*;

pub use interpolation_2d::*;

pub use filters::transformers::filter_random_accessible::*;

pub use traits::is_nd::*;
pub use traits::is_2d::*;
pub use traits::is_3d::*;

pub use traits::is_movable_2d::*;
pub use traits::is_movable_3d::*;

pub use traits::is_buildable_nd::*;
pub use traits::is_buildable_2d::*;
pub use traits::is_buildable_3d::*;

pub use traits::is_editable_nd::*;
pub use traits::is_editable_2d::*;
pub use traits::is_editable_3d::*;

pub use traits::is_normalized_2d::*;
pub use traits::is_normalized_3d::*;

pub use traits::is_transformable_to_2d::*;
pub use traits::is_transformable_to_3d::*;

pub use traits::is_sortable_nd::*;
pub use traits::is_sortable_2d::*;
pub use traits::is_sortable_3d::*;

pub use traits::is_plane_3d::*;

pub use traits::has_length::*;

pub use traits::is_tree_3d::*;
pub use traits::is_kd_tree_3d::*;
pub use traits::is_oc_tree::*;

pub use traits::is_random_accessible::*;
pub use traits::is_random_insertible::*;

pub use traits::has_bounding_box_2d::*;
pub use traits::has_bounding_box_3d::*;

pub use traits::has_center_of_gravity_2d::*;
pub use traits::has_center_of_gravity_3d::*;

pub use traits::is_mergeable::*;

pub use traits::is_view_buildable::*;

pub use point_cloud_2d::*;
pub use point_cloud_3d::*;

pub use traits::is_mesh::*;
pub use traits::is_mesh_3d::*;
pub use traits::is_topology_unit::*;
pub use traits::is_editable_mesh::*;
pub use traits::is_searchable_mesh::*;

pub use traits::is_filter::*;
pub use traits::is_filter_random_accessible::*;

pub use view::*;

pub use half_edge::*;

pub use filters::filter_box_2d::*;
pub use filters::filter_box_3d::*;
pub use filters::filter_circle::*;
pub use filters::filter_sphere::*;

pub use filters::combinators::filter_all::*;
pub use filters::combinators::filter_all_random_accessible::*;
pub use filters::combinators::filter_allow::*;
pub use filters::combinators::filter_and::*;
pub use filters::combinators::filter_any::*;
pub use filters::combinators::filter_any_random_accessible::*;
pub use filters::combinators::filter_deny::*;
pub use filters::combinators::filter_negate::*;
pub use filters::combinators::filter_or::*;
pub use filters::combinators::filter_outer_inner::*;
pub use filters::combinators::filter_xor::*;

pub use algorithms::convex_hull_2d::*;





