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

//! helper functions for testing (these functions unwrap and panic, only use for tests)

use std::io::prelude::*;
use std::fs::File;
use traits::is_buildable_2d::*;
use traits::is_buildable_3d::*;
use traits::is_filter::*;
use traits::is_filter_random_accessible::*;
use traits::is_view_buildable::*;
use point_cloud_2d::*;
use point_cloud_3d::*;
use filters::transformers::filter_random_accessible::*;
use view::*;
use io::xy::*;
use io::xyz::*;

//@todo maybe move directly to tests directory
/// Ensures the content of two files is equal
pub fn assert_files_equal(filepath1: &str, filepath2: &str) {
    let mut f1 = File::open(filepath1).unwrap();
    let mut f2 = File::open(filepath2).unwrap();

    let mut s1 = String::new();
    f1.read_to_string(&mut s1).unwrap();

    let mut s2 = String::new();
    f2.read_to_string(&mut s2).unwrap();

    assert!(s1 == s2);
}

/// Tests a 2D filter by comparing the result of its usage on the test quare vs. its expected result
pub fn test_filter_2d<F, P>(f: F, path_expected: &str, unique_identifier: &str) where
    F: IsFilter<P>,
    P: IsBuildable2D + Clone {

    let path_tmp = ["tests/tmp/tmp", unique_identifier, ".xyz"].join("");
    let filter = FilterRandomAccessible::build(f);

    let mut view = View::Full;
    let mut pc = PointCloud2D::<P>::new();
    load_xy(&mut pc, "tests/data/test_square.xy", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view).unwrap();
    save_xy(&pc, &path_tmp, " ", "\n").unwrap();
    assert_files_equal(path_expected, &path_tmp);
}

/// Tests a 3D filter by comparing the result of its usage on the test cube vs. its expected result
pub fn test_filter_3d<F, P>(f: F, path_expected: &str, unique_identifier: &str) where
    F: IsFilter<P>,
    P: IsBuildable3D + Clone {

    let path_tmp = ["tests/tmp/tmp", unique_identifier, ".xyz"].join("");
    let filter = FilterRandomAccessible::build(f);

    let mut view = View::Full;
    let mut pc = PointCloud3D::<P>::new();
    load_xyz(&mut pc, "tests/data/test_cube.xyz", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view).unwrap();
    save_xyz(&pc, &path_tmp, " ", "\n").unwrap();
    assert_files_equal(path_expected, &path_tmp);
}

/// Can be used to write the expected result of a 2D filter to later compare against in a test
pub fn write_expected_filter_2d<F, P>(f: F, path_expected: &str) where
    F: IsFilter<P>,
    P: IsBuildable2D + Clone {

    let filter = FilterRandomAccessible::build(f);

    let mut view = View::Full;
    let mut pc = PointCloud2D::<P>::new();
    load_xy(&mut pc, "tests/data/test_square.xy", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view).unwrap();
    save_xy(&pc, &path_expected, " ", "\n").unwrap();
}

/// Can be used to write the expected result of a 3D filter to later compare against in a test
pub fn write_expected_filter_3d<F, P>(f: F, path_expected: &str) where
    F: IsFilter<P>,
    P: IsBuildable3D + Clone {

    let filter = FilterRandomAccessible::build(f);

    let mut view = View::Full;
    let mut pc = PointCloud3D::<P>::new();
    load_xyz(&mut pc, "tests/data/test_cube.xyz", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view).unwrap();
    save_xyz(&pc, &path_expected, " ", "\n").unwrap();
}
