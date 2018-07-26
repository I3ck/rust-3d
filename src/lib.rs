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
//! LGPL (see LICENSE)

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

mod           aa_bb_tree_2d;
pub use self::aa_bb_tree_2d::AABBTree2D;

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

mod           polygon_2d;
pub use self::polygon_2d::Polygon2D;

mod           polygon_3d;
pub use self::polygon_3d::Polygon3D;

mod           norm_2d;
pub use self::norm_2d::Norm2D;

mod           norm_3d;
pub use self::norm_3d::Norm3D;

mod           bounding_box_2d;
pub use self::bounding_box_2d::BoundingBox2D;

mod           bounding_box_3d;
pub use self::bounding_box_3d::BoundingBox3D;

mod           matrix3;
pub use self::matrix3::Matrix3;

mod           matrix4;
pub use self::matrix4::Matrix4;

mod           matrix3_pipe;
pub use self::matrix3_pipe::Matrix3Pipe;

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

mod           shapes;
pub use self::shapes::*;

mod enums;
pub use self::enums::*;

mod utils;
