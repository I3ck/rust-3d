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

//! Containing traits used by rust-3d
mod           has_bounding_box_2d;
pub use self::has_bounding_box_2d::HasBoundingBox2D;

mod           has_bounding_box_3d;
pub use self::has_bounding_box_3d::HasBoundingBox3D;

mod           has_center_of_gravity_2d;
pub use self::has_center_of_gravity_2d::HasCenterOfGravity2D;

mod           has_center_of_gravity_3d;
pub use self::has_center_of_gravity_3d::HasCenterOfGravity3D;

mod           has_length;
pub use self::has_length::HasLength;

mod           is_editable_2d;
pub use self::is_editable_2d::IsEditable2D;

mod           is_editable_3d;
pub use self::is_editable_3d::IsEditable3D;

mod           is_editable_nd;
pub use self::is_editable_nd::IsEditableND;

mod           is_editable_polygon;
pub use self::is_editable_polygon::IsEditablePolygon;

mod           is_buildable_2d;
pub use self::is_buildable_2d::IsBuildable2D;

mod           is_buildable_3d;
pub use self::is_buildable_3d::IsBuildable3D;

mod           is_buildable_nd;
pub use self::is_buildable_nd::IsBuildableND;

mod           is_2d;
pub use self::is_2d::Is2D;

mod           is_3d;
pub use self::is_3d::Is3D;

mod           is_random_accessible;
pub use self::is_random_accessible::IsRandomAccessible;

mod           is_random_insertible;
pub use self::is_random_insertible::IsRandomInsertible;

mod           is_editable_mesh;
pub use self::is_editable_mesh::IsEditableMesh;

mod           is_k_nearest_searchable;
pub use self::is_k_nearest_searchable::IsKNearestSearchable;

mod           is_matrix3_transformable;
pub use self::is_matrix3_transformable::IsMatrix3Transformable;

mod           is_matrix4_transformable;
pub use self::is_matrix4_transformable::IsMatrix4Transformable;

mod           is_sphere_searchable;
pub use self::is_sphere_searchable::IsSphereSearchable;

mod           is_box_3d_searchable;
pub use self::is_box_3d_searchable::IsBox3DSearchable;

mod           is_mesh;
pub use self::is_mesh::IsMesh;

mod           is_mesh_3d;
pub use self::is_mesh_3d::IsMesh3D;

mod           is_topology_unit;
pub use self::is_topology_unit::IsTopologyUnit;

mod           is_searchable_mesh;
pub use self::is_searchable_mesh::IsSearchableMesh;

mod           is_movable_2d;
pub use self::is_movable_2d::IsMovable2D;

mod           is_movable_3d;
pub use self::is_movable_3d::IsMovable3D;

mod           is_normalized_2d;
pub use self::is_normalized_2d::IsNormalized2D;

mod           is_normalized_3d;
pub use self::is_normalized_3d::IsNormalized3D;

mod           is_oc_tree;
pub use self::is_oc_tree::IsOcTree;

mod           is_plane_3d;
pub use self::is_plane_3d::IsPlane3D;

mod           is_polygon;
pub use self::is_polygon::IsPolygon;

mod           is_projection_to_plane;
pub use self::is_projection_to_plane::IsProjectionToPlane;

mod           is_tree_3d;
pub use self::is_tree_3d::IsTree3D;

mod           is_voxel_image;
pub use self::is_voxel_image::IsVoxelImage;

mod           is_transformable_to_2d;
pub use self::is_transformable_to_2d::IsTransFormableTo2D;

mod           is_transformable_to_3d;
pub use self::is_transformable_to_3d::IsTransFormableTo3D;

mod           is_filter;
pub use self::is_filter::IsFilter;

mod           is_filter_random_accessible;
pub use self::is_filter_random_accessible::IsFilterRandomAccessible;

mod           is_scalable;
pub use self::is_scalable::IsScalable;

mod           is_view_buildable;
pub use self::is_view_buildable::IsViewBuildable;

mod           is_nd;
pub use self::is_nd::IsND;

mod           is_sortable_nd;
pub use self::is_sortable_nd::IsSortableND;

mod           is_sortable_2d;
pub use self::is_sortable_2d::IsSortable2D;

mod           is_sortable_3d;
pub use self::is_sortable_3d::IsSortable3D;

mod           is_mergeable;
pub use self::is_mergeable::IsMergeable;

mod           has_distance_to;
pub use self::has_distance_to::HasDistanceTo;

mod           is_direction_field_2d;
pub use self::is_direction_field_2d::IsDirectionField2D;

mod           is_direction_field_3d;
pub use self::is_direction_field_3d::IsDirectionField3D;
