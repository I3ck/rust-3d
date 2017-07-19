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

//! Containing filters

pub mod combinators;
pub mod transformers;

mod           filter_box_2d;
pub use self::filter_box_2d::FilterBox2D;

mod           filter_box_3d;
pub use self::filter_box_3d::FilterBox3D;

mod           filter_circle;
pub use self::filter_circle::FilterCircle;

mod           filter_sphere;
pub use self::filter_sphere::FilterSphere;

mod           filter_outlier_3d;
pub use self::filter_outlier_3d::FilterOutlier3D;
