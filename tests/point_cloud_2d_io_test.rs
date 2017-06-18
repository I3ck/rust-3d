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

use rust_3d::traits::is_buildable_2d::*;
use rust_3d::traits::is_moveable_2d::*;
use rust_3d::traits::is_random_insertible::*;
use rust_3d::traits::is_random_accessible::*;
use rust_3d::point_2d::*;
use rust_3d::point_cloud_2d::*;
use rust_3d::io::xy::*;
use rust_3d::test_helper::*;

use std::io::prelude::*;
use std::fs::File;

static GENERATE_EXCEPTED_RESULT_FILES: bool = false;

#[test]
fn point_cloud_2d_io_test() {
    {
        let path_expected = "tests/data/expected_pc_2d_save1.csv";
        let path_tmp = "tests/tmp/pc_2d_save1.tmp";
        let mut pc = PointCloud2D::<Point2D>::new();

        for i in 0..10 {
            let p = *Point2D::build(0.1 * i as f64, 0.2 * i as f64);
            pc.push(p);
        }

        if GENERATE_EXCEPTED_RESULT_FILES {
            save_xy(&pc, &path_expected, ";", "\n");
        }

        save_xy(&pc, &path_tmp, ";", "\n");

        assert_files_equal(path_expected, path_tmp);
    }

    {
        //@todo also compare values
        let mut pc = PointCloud2D::<Point2D>::new();
        load_xy(&mut pc, "tests/data/test_square.xy", " ", "\n").unwrap();
        assert!(pc.len() == 20 * 20);
    }

}
