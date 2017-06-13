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

/// Containing traits used by rust-3d

pub mod has_bounding_box_2d;
pub mod has_bounding_box_3d;
pub mod has_center_of_gravity_2d;
pub mod has_center_of_gravity_3d;
pub mod has_length;
pub mod is_editable_2d;
pub mod is_editable_3d;
pub mod is_editable_nd;
pub mod is_buildable_2d;
pub mod is_buildable_3d;
pub mod is_buildable_nd;
pub mod is_2d;
pub mod is_3d;
pub mod is_random_accessible_2d;
pub mod is_random_accessible_3d;
pub mod is_editable_mesh_3d;
pub mod is_kd_tree_3d;
pub mod is_mesh_3d;
pub mod is_moveable_2d;
pub mod is_moveable_3d;
pub mod is_normalized_2d;
pub mod is_normalized_3d;
pub mod is_oc_tree;
pub mod is_plane_3d;
pub mod is_projection_to_plane;
pub mod is_tree_3d;
pub mod is_voxel_image;
pub mod transformable_to_2d;
pub mod transformable_to_3d;
pub mod is_filter;
pub mod is_filter_pc_2d;
pub mod is_filter_pc_3d;
pub mod is_nd;
