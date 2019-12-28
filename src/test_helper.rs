/*
Copyright 2017 Martin Buck

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

//! helper functions for testing (these functions unwrap and panic, only use for tests)

use std::{fs::File, io::prelude::*};

use crate::{io::*, *};

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
pub fn test_filter_2d<F, P>(f: F, path_expected: &str, unique_identifier: &str)
where
    F: IsFilter<P>,
    P: IsBuildable2D + Clone,
{
    let path_tmp = ["tests/tmp/tmp", unique_identifier, ".xyz"].join("");
    let filter = FilterRandomAccessible::new(f);

    let mut view = View::Full;
    let mut pc = PointCloud2D::<P>::new();
    let mut f = File::open("tests/data/test_square.xy").unwrap();
    load_xy(&mut f, &mut pc, " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view).unwrap();

    let mut f_tmp = File::create(&path_tmp).unwrap();
    save_xy(&mut f_tmp, &pc, " ", "\n").unwrap();
    assert_files_equal(path_expected, &path_tmp);
}

/// Tests a 3D filter by comparing the result of its usage on the test cube vs. its expected result
pub fn test_filter_3d<F, P>(f: F, path_expected: &str, unique_identifier: &str)
where
    F: IsFilter<P>,
    P: IsBuildable3D + Clone,
{
    let path_tmp = ["tests/tmp/tmp", unique_identifier, ".xyz"].join("");
    let filter = FilterRandomAccessible::new(f);

    let mut view = View::Full;
    let mut pc = PointCloud3D::<P>::new();
    let mut f = File::open("tests/data/test_cube.xyz").unwrap();
    load_xyz(&mut f, &mut pc, " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view).unwrap();

    let mut f_tmp = File::create(&path_tmp).unwrap();
    save_xyz(&mut f_tmp, &pc, " ", "\n").unwrap();
    assert_files_equal(path_expected, &path_tmp);
}

/// Can be used to write the expected result of a 2D filter to later compare against in a test
pub fn write_expected_filter_2d<F, P>(f: F, path_expected: &str)
where
    F: IsFilter<P>,
    P: IsBuildable2D + Clone,
{
    let filter = FilterRandomAccessible::new(f);

    let mut view = View::Full;
    let mut pc = PointCloud2D::<P>::new();
    let mut f = File::open("tests/data/test_square.xy").unwrap();
    load_xy(&mut f, &mut pc, " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    let mut f_expected = File::create(path_expected).unwrap();
    pc.apply_view(&view).unwrap();
    save_xy(&mut f_expected, &pc, " ", "\n").unwrap();
}

/// Can be used to write the expected result of a 3D filter to later compare against in a test
pub fn write_expected_filter_3d<F, P>(f: F, path_expected: &str)
where
    F: IsFilter<P>,
    P: IsBuildable3D + Clone,
{
    let filter = FilterRandomAccessible::new(f);

    let mut view = View::Full;
    let mut pc = PointCloud3D::<P>::new();
    let mut f = File::open("tests/data/test_cube.xyz").unwrap();
    load_xyz(&mut f, &mut pc, " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    let mut f_expected = File::create(path_expected).unwrap();
    pc.apply_view(&view).unwrap();
    save_xyz(&mut f_expected, &pc, " ", "\n").unwrap();
}
