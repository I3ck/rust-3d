/*
Copyright 2016 Martin Buck

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

//! rust-3d
//! =======
//! 3D/2D library written in Rust.
//! Offering useful containers, structures and algorithms for 2D and 3D space.
//! Meant as basis for numeric algorithms, viewers, game engines, ...
//!
//!
//! Migration 0.29.0 -> 0.30.0
//! --------------------------
//! Note that the module structure changed. There's now only submodules for `io` and `impls`.  
//! Also `prelude` was removed.  
//! If you were using the prelude via `rust_3d::prelude::*;` you should now be able to just switch to
//! `rust_3d::*;`.  
//! If you were using explicit paths such as `rust_3d::filters::combinators::FilterAll` you should now use `rust_3d::FilterAll`.  
//! Note that `io` and `impls` are still part of the module path.  
//! This should make future usage easier, but might be painful for existing users.  
//!
//!
//! Tour
//! ----
//! Here's a little overview of some of `rust-3d`'s features.
//! The snippets / names might not be up-to-date, so please check `tests/` for compiling examples.
//!
//!
//! ### Proper error handling
//! No `.unwrap()` where it's not 100% safe.
//!
//! ### Strong / Smart Types
//! There's strong types for everything that might get mixed up easily.  
//! This way e.g. ids of faces can't be mistaken for ids of vertices.
//! ```rust,ignore
//! fn edges_of_face(&self, faceid: FId) -> Option<(EId, EId, EId)>;
//! ```
//! There's also smart types which restrict the values they can hold.  
//! This way distances can never be `< 0.0`, sizes can be enfored to be `> 0.0` etc.
//! ```rust,ignore
//! Positive  
//! NonNegative
//! ```
//!
//! ### Generic Code Base
//! I try and keep all algorithms and types as generic as possible.
//! - Even rather basic types like `Is2D` are split into several versions: `IsEditable2D`, `IsBuildable2D`
//! - `IsMesh` is defined for any vertex type and any number of vertices / face
//! - There's traits for collections (no need to use `Vec`)  
//!
//! This makes it possible to require as little implementation work as possible if you want to use your own types.  
//!
//!
//! ### Combinators / Transformers
//! - Any `IsFilter<T>` can be combined via `FilterAND`, `FilterOR`, `FilterAny`, `FilterNegate`...  
//! - Any `IsFilter<T>` can be transformed to work for any collection of `T`s (`IsFilterRandomAccessible`).
//! - `IsDirectionField2D` might be transformed to an `IsFilter<Is2D>`, which can then be transformed to an `IsFilterRandomAccessible<Is2D>`.
//!
//!
//! ### IO
//! Any `IO` method is defined on traits, so if you implement these, you'll get read/write of different file formats for free.
//!
//!
//! Documentation
//! -------------
//! You can find the documentation [here](https://docs.rs/rust-3d/).
//!
//!
//! Examples
//! --------
//! Please take a look at the tests in `tests/`. These will be up-to-date and compiling.  
//! I might add extensive tutorials / examples / demo projects in the future.
//!
//!
//! Links
//! -----
//! [crates.io](https://crates.io/crates/rust-3d)  
//! [github.com](https://github.com/I3ck/rust-3d)  
//! [docs.rs](https://docs.rs/rust-3d/)
//!
//!
//! Contribute
//! ----------
//! Feel free to open an issue in case you're missing something or found a bug.
//! Please avoid directly contributing since I might be working on breaking changes or the feature you want to implement.
//! Open an issue or email me beforehand.
//!
//!
//! License
//! ------
//! MIT (see LICENSE)

pub mod io;

mod distances_2d;
pub use self::distances_2d::*;

mod distances_3d;
pub use self::distances_3d::*;

mod distances_nd;
pub use self::distances_nd::*;

mod factory_2d;
pub use self::factory_2d::*;

mod functions;
pub use self::functions::*;

mod impls;
pub use self::impls::*;

mod interpolate_2d;
pub use interpolate_2d::*;

pub mod test_helper;

mod utils;
pub use self::utils::*;

mod aa_bb_tree_2d;
pub use self::aa_bb_tree_2d::AABBTree2D;

mod strong_types;
pub use self::strong_types::*;

mod aa_bb_tree_3d;
pub use self::aa_bb_tree_3d::AABBTree3D;

mod point_2d;
pub use self::point_2d::Point2D;

mod point_3d;
pub use self::point_3d::Point3D;

mod line_2d;
pub use self::line_2d::Line2D;

mod line_3d;
pub use self::line_3d::Line3D;

mod line_segment_2d;
pub use self::line_segment_2d::LineSegment2D;

mod line_segment_3d;
pub use self::line_segment_3d::LineSegment3D;

mod ray_2d;
pub use self::ray_2d::Ray2D;

mod ray_3d;
pub use self::ray_3d::Ray3D;

mod plane_3d;
pub use self::plane_3d::Plane3D;

mod point_cloud_2d;
pub use self::point_cloud_2d::PointCloud2D;

mod point_cloud_3d;
pub use self::point_cloud_3d::PointCloud3D;

mod point_cloud_3d_f32;
pub use self::point_cloud_3d_f32::PointCloud3Df32;

mod polygon_2d;
pub use self::polygon_2d::Polygon2D;

mod polygon_3d;
pub use self::polygon_3d::Polygon3D;

mod norm_2d;
pub use self::norm_2d::Norm2D;

mod norm_3d;
pub use self::norm_3d::Norm3D;

mod bounding_box_2d;
pub use self::bounding_box_2d::BoundingBox2D;

mod bounding_box_3d;
pub use self::bounding_box_3d::BoundingBox3D;

mod matrix3;
pub use self::matrix3::Matrix3;

mod matrix4;
pub use self::matrix4::Matrix4;

mod matrix3_pipe;
pub use self::matrix3_pipe::Matrix3Pipe;

mod matrix4_pipe;
pub use self::matrix4_pipe::Matrix4Pipe;

mod compressed_point_3d;
pub use self::compressed_point_3d::CompressedPoint3D;

mod compressed_point_cloud_3d;
pub use self::compressed_point_cloud_3d::CompressedPointCloud3D;

mod kd_tree;
pub use self::kd_tree::KdTree;

mod mesh_3d;
pub use self::mesh_3d::Mesh3D;

mod searchable_mesh;
pub use self::searchable_mesh::SearchableMesh;

mod oc_tree;
pub use self::oc_tree::OcTree;

mod view;
pub use self::view::View;

mod positive;
pub use self::positive::Positive;

mod non_negative;
pub use self::non_negative::NonNegative;

mod result;
pub use self::result::*;

mod rgb;
pub use self::rgb::Rgb;

mod face3;
pub use self::face3::Face3;

mod half_edge;
pub use self::half_edge::HalfEdge;

mod enums;
pub use self::enums::*;

mod cluster;
pub use self::cluster::*;

mod sat_collider;
pub use self::sat_collider::*;

mod box_unaligned_3d;
pub use self::box_unaligned_3d::*;

mod tri_face_3d;
pub use self::tri_face_3d::*;

mod collider_3d;
pub use self::collider_3d::*;

mod convex_hull_2d;
pub use self::convex_hull_2d::convex_hull_2d;

mod douglas_peucker_2d;
pub use self::douglas_peucker_2d::douglas_peucker_2d;

pub mod subdivide;

mod unify_faces;
pub use self::unify_faces::unify_faces;

mod heal_mesh;
pub use self::heal_mesh::heal_mesh;

mod cluster_vertices;
pub use self::cluster_vertices::cluster_vertices;

mod circle;
pub use self::circle::Circle;

mod box_2d;
pub use self::box_2d::Box2D;

mod sphere;
pub use self::sphere::Sphere;

mod box_3d;
pub use self::box_3d::Box3D;

mod has_bounding_box_2d;
pub use self::has_bounding_box_2d::{
    HasBoundingBox2D, HasBoundingBox2DConverted, HasBoundingBox2DMaybe,
};

mod has_bounding_box_3d;
pub use self::has_bounding_box_3d::{
    HasBoundingBox3D, HasBoundingBox3DConverted, HasBoundingBox3DMaybe,
};

mod has_center_of_gravity_2d;
pub use self::has_center_of_gravity_2d::HasCenterOfGravity2D;

mod has_center_of_gravity_3d;
pub use self::has_center_of_gravity_3d::HasCenterOfGravity3D;

mod has_length;
pub use self::has_length::HasLength;

mod is_editable_2d;
pub use self::is_editable_2d::IsEditable2D;

mod is_editable_3d;
pub use self::is_editable_3d::IsEditable3D;

mod is_editable_nd;
pub use self::is_editable_nd::IsEditableND;

mod is_editable_polygon;
pub use self::is_editable_polygon::IsEditablePolygon;

mod is_buildable_2d;
pub use self::is_buildable_2d::IsBuildable2D;

mod is_buildable_3d;
pub use self::is_buildable_3d::IsBuildable3D;

mod is_buildable_nd;
pub use self::is_buildable_nd::IsBuildableND;

mod is_2d;
pub use self::is_2d::Is2D;

mod is_3d;
pub use self::is_3d::Is3D;

mod is_random_accessible;
pub use self::is_random_accessible::IsRandomAccessible;

mod is_random_insertible;
pub use self::is_random_insertible::IsRandomInsertible;

mod is_pushable;
pub use self::is_pushable::IsPushable;

mod is_face_editable_mesh;
pub use self::is_face_editable_mesh::IsFaceEditableMesh;

mod is_vertex_editable_mesh;
pub use self::is_vertex_editable_mesh::IsVertexEditableMesh;

mod is_k_nearest_searchable;
pub use self::is_k_nearest_searchable::IsKNearestSearchable;

mod is_matrix3_transformable;
pub use self::is_matrix3_transformable::IsMatrix3Transformable;

mod is_matrix4_transformable;
pub use self::is_matrix4_transformable::IsMatrix4Transformable;

mod is_sphere_searchable;
pub use self::is_sphere_searchable::IsSphereSearchable;

mod is_box_3d_searchable;
pub use self::is_box_3d_searchable::IsBox3DSearchable;

mod is_mesh;
pub use self::is_mesh::IsMesh;

mod is_mesh_3d;
pub use self::is_mesh_3d::IsMesh3D;

mod is_topology_unit;
pub use self::is_topology_unit::IsTopologyUnit;

mod is_searchable_mesh;
pub use self::is_searchable_mesh::IsSearchableMesh;

mod is_movable_2d;
pub use self::is_movable_2d::IsMovable2D;

mod is_movable_3d;
pub use self::is_movable_3d::IsMovable3D;

mod is_normalized_2d;
pub use self::is_normalized_2d::IsNormalized2D;

mod is_normalized_3d;
pub use self::is_normalized_3d::IsNormalized3D;

mod is_oc_tree;
pub use self::is_oc_tree::IsOcTree;

mod is_plane_3d;
pub use self::is_plane_3d::IsPlane3D;

mod is_polygon;
pub use self::is_polygon::IsPolygon;

mod is_tree_3d;
pub use self::is_tree_3d::IsTree3D;

mod is_voxel_image;
pub use self::is_voxel_image::IsVoxelImage;

mod is_transformable_to_2d;
pub use self::is_transformable_to_2d::IsTransFormableTo2D;

mod is_transformable_to_3d;
pub use self::is_transformable_to_3d::IsTransFormableTo3D;

mod is_filter;
pub use self::is_filter::IsFilter;

mod is_filter_random_accessible;
pub use self::is_filter_random_accessible::IsFilterRandomAccessible;

mod is_scalable;
pub use self::is_scalable::IsScalable;

mod is_view_buildable;
pub use self::is_view_buildable::IsViewBuildable;

mod is_nd;
pub use self::is_nd::IsND;

mod is_sortable_nd;
pub use self::is_sortable_nd::IsSortableND;

mod is_sortable_2d;
pub use self::is_sortable_2d::IsSortable2D;

mod is_sortable_3d;
pub use self::is_sortable_3d::IsSortable3D;

mod is_mergeable;
pub use self::is_mergeable::IsMergeable;

mod has_distance_to;
pub use self::has_distance_to::HasDistanceTo;

mod is_direction_field_2d;
pub use self::is_direction_field_2d::IsDirectionField2D;

mod is_direction_field_3d;
pub use self::is_direction_field_3d::IsDirectionField3D;

mod is_sat_object;
pub use self::is_sat_object::IsSATObject;

mod has_colliders_3d;
pub use self::has_colliders_3d::HasColliders3D;

mod is_collider_container_3d;
pub use self::is_collider_container_3d::IsColliderContainer3D;

mod filter_all_random_accessible;
pub use self::filter_all_random_accessible::FilterAllRandomAccessible;

mod filter_any_random_accessible;
pub use self::filter_any_random_accessible::FilterAnyRandomAccessible;

mod filter_all;
pub use self::filter_all::FilterAll;

mod filter_any;
pub use self::filter_any::FilterAny;

mod filter_negate;
pub use self::filter_negate::FilterNegate;

mod filter_and;
pub use self::filter_and::FilterAND;

mod filter_or;
pub use self::filter_or::FilterOR;

mod filter_xor;
pub use self::filter_xor::FilterXOR;

mod filter_outer_inner;
pub use self::filter_outer_inner::FilterOuterInner;

mod filter_allow;
pub use self::filter_allow::FilterAllow;

mod filter_deny;
pub use self::filter_deny::FilterDeny;

mod filter_random_accessible;
pub use self::filter_random_accessible::FilterRandomAccessible;

mod filter_direction_field_2d;
pub use self::filter_direction_field_2d::FilterDirectionField2D;

mod filter_direction_field_3d;
pub use self::filter_direction_field_3d::FilterDirectionField3D;

mod filter_box_2d;
pub use self::filter_box_2d::FilterBox2D;

mod filter_box_3d;
pub use self::filter_box_3d::FilterBox3D;

mod filter_circle;
pub use self::filter_circle::FilterCircle;

mod filter_sphere;
pub use self::filter_sphere::FilterSphere;

mod filter_outlier_3d;
pub use self::filter_outlier_3d::FilterOutlier3D;

mod is_index_container;
pub use self::is_index_container::{IsIndexContainer, IsIndexContainerIterator};

mod dynamic_precision_index_vec;
pub use self::dynamic_precision_index_vec::DynamicPrecisionIndexVec;

mod u32_index_vec;
pub use self::u32_index_vec::U32IndexVec;

mod is_data_container;
pub use self::is_data_container::IsDataContainer;

mod skip_empty_string;
pub use self::skip_empty_string::*;

mod skip_empty;
pub use self::skip_empty::*;
