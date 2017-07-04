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

extern crate rust_3d;

use rust_3d::traits::is_3d::*;
use rust_3d::traits::is_buildable_3d::*;
use rust_3d::traits::is_filter::*;
use rust_3d::traits::is_filter_random_accessible::*;
use rust_3d::traits::is_random_accessible::*;
use rust_3d::traits::is_view_buildable::*;
use rust_3d::point_3d::*;
use rust_3d::point_cloud_3d::*;
use rust_3d::positive::*;
use rust_3d::filters::filter_box_3d::*;
use rust_3d::filters::filter_sphere::*;
use rust_3d::filters::combinators::filter_negate::*;
use rust_3d::filters::combinators::filter_or::*;
use rust_3d::filters::combinators::filter_and::*;
use rust_3d::filters::combinators::filter_xor::*;
use rust_3d::filters::combinators::filter_all::*;
use rust_3d::filters::combinators::filter_any::*;
use rust_3d::filters::combinators::filter_outer_inner::*;
use rust_3d::filters::transformers::filter_random_accessible::*;
use rust_3d::view::*;
use rust_3d::io::xyz::*;
use rust_3d::test_helper::*;

#[test]
fn filter_box_3d_test() {
    let center = *Point3D::new(10.0, 10.0, 10.0);
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(2.1).unwrap();
    let size_z = Positive::new(2.1).unwrap();
    test_filter_3d::<_, Point3D>(FilterBox3D::new(center, size_x, size_y, size_z), "tests/data/expected_filter_box_3d.xyz", "box3d");
}

#[test]
fn filter_sphere_test() {
    let center = *Point3D::new(10.0, 10.0, 10.0);
    let radius = Positive::new(4.0).unwrap();
    let filter = FilterSphere::new(center, radius);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_sphere.xyz", "sphere");
}
