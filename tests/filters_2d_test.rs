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

use rust_3d::traits::is_2d::*;
use rust_3d::traits::is_buildable_2d::*;
use rust_3d::traits::is_filter::*;
use rust_3d::traits::is_filter_random_accessible::*;
use rust_3d::traits::is_random_accessible::*;
use rust_3d::traits::is_view_buildable::*;
use rust_3d::point_2d::*;
use rust_3d::point_cloud_2d::*;
use rust_3d::positive::*;
use rust_3d::filters::filter_circle::*;
use rust_3d::filters::filter_box_2d::*;
use rust_3d::view::*;
use rust_3d::io::xyz::*;
use rust_3d::test_helper::*;

#[test]
fn filter_circle_test() {
    let center = *Point2D::new(10.0, 10.0);
    let size   = Positive::new(2.1).unwrap();
    test_filter_2d::<_, Point2D>(FilterCircle::new(center, size), "tests/data/expected_filter_circle.xy", "circle");
}

#[test]
fn filter_box_2d_test() {
    let center = *Point2D::new(10.0, 10.0);
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(4.1).unwrap();
    test_filter_2d::<_, Point2D>(FilterBox2D::new(center, size_x, size_y), "tests/data/expected_filter_box_2d.xy", "box_2d");
}
