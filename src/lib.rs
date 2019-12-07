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
//! Notes
//! -----
//! `rust-3d` is still in really early stages, there might come breaking changes with each update.
//! The test coverage is far from perfect, so you might find some bugs (please report them).
//! Compiling with `stable`.
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
//! fn edges_of_face(&self, faceid: FId) -> Result<(EId, EId, EId)>;
//! ```
//! There's also smart types which restrict the values they can hold.
//! This way distances can never be `< 0.0`, sizes can be enfored to be `> 0.0` etc.
//! ```rust,ignore
//! Positive
//! NonNegative
//! ```
//!
//! ### Generic Code Base
//! I try and keep all algorithms and types as generic as possible (see `/src/traits`).
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
//! The documentation is quite good already, come and [take a look](https://docs.rs/rust-3d/).
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

pub mod distances_2d;
pub mod distances_3d;
pub mod distances_nd;
pub mod factory_2d;
pub mod filters;
pub mod functions;
pub mod impls;
pub mod interpolate_2d;
pub mod io;
pub mod prelude;
pub mod strong_types;
pub mod test_helper;
pub mod traits;
pub mod utils;

mod aa_bb_tree_2d;
pub use self::aa_bb_tree_2d::AABBTree2D;

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
pub use self::result::Result;

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
