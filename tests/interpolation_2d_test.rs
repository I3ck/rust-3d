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
use rust_3d::traits::is_moveable_2d::*;
use rust_3d::point_2d::*;
use rust_3d::point_cloud_2d::*;
use rust_3d::interpolation_2d::*;
use rust_3d::io::pointcloud_2d_io::*;

use std::io::prelude::*;
use std::fs::File;

static GENERATE_EXCEPTED_RESULT_FILES: bool = false;

#[test]
fn interpolate_bezier_test() {
    let path_expected = "tests/data/expected_interpolate_bezier_save1.csv";
    let path_tmp = "tests/tmp/interpolate_bezier_save1.tmp";
    let mut pc = PointCloud2D::<Point2D>::new();

    pc.push(*Point2D::build(0.0, 0.0));
    pc.push(*Point2D::build(1.0, 0.0));
    pc.push(*Point2D::build(1.0, 1.0));
    pc.push(*Point2D::build(0.5, 1.0));

    let result = *interpolate_bezier(&pc, 50);

    if GENERATE_EXCEPTED_RESULT_FILES {
        save_xy(&result, &path_expected, ";", "\n");
    }

    save_xy(&result, &path_tmp, ";", "\n");

    assert!(result.len() == 50);

    //@todo below as helper which can be reused for many other io functions
    let mut f_expected = File::open(path_expected).unwrap();
    let mut f_tmp = File::open(path_tmp).unwrap();

    let mut s_expected = String::new();
    f_expected.read_to_string(&mut s_expected).unwrap();

    let mut s_tmp = String::new();
    f_tmp.read_to_string(&mut s_tmp).unwrap();

    assert!(s_tmp == s_expected);
}
