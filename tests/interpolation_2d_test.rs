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

#![deny(warnings)]

use rust_3d::{io::*, test_helper::*, *};

use std::fs::File;

static GENERATE_EXCEPTED_RESULT_FILES: bool = false;

#[test]
fn interpolate_bezier_test() {
    let path_expected = "tests/data/expected_interpolate_bezier_save1.csv";
    let path_tmp = "tests/tmp/interpolate_bezier_save1.tmp";
    let mut pc = PointCloud2D::<Point2D>::new();

    pc.push(Point2D::new(0.0, 0.0));
    pc.push(Point2D::new(1.0, 0.0));
    pc.push(Point2D::new(1.0, 1.0));
    pc.push(Point2D::new(0.5, 1.0));

    let result = interpolate_bezier(&pc, 50).unwrap();

    if GENERATE_EXCEPTED_RESULT_FILES {
        save_xy(
            &mut File::create(&path_expected).unwrap(),
            &result,
            ";",
            "\n",
        )
        .unwrap();
    }

    save_xy(&mut File::create(&path_tmp).unwrap(), &result, ";", "\n").unwrap();

    assert!(result.len() == 50);

    assert_files_equal(path_expected, path_tmp);
}

#[test]
fn interpolate_cosine_test() {
    let path_expected = "tests/data/expected_interpolate_cosine_save1.csv";
    let path_tmp = "tests/tmp/interpolate_cosine_save1.tmp";
    let mut pc = PointCloud2D::<Point2D>::new();

    pc.push(Point2D::new(0.0, 0.2));
    pc.push(Point2D::new(1.0, 0.7));
    pc.push(Point2D::new(1.3, 1.0));
    pc.push(Point2D::new(0.5, 6.0));

    let result = interpolate_cosine(&pc, 50).unwrap();

    if GENERATE_EXCEPTED_RESULT_FILES {
        save_xy(
            &mut File::create(&path_expected).unwrap(),
            &result,
            ";",
            "\n",
        )
        .unwrap();
    }

    save_xy(&mut File::create(&path_tmp).unwrap(), &result, ";", "\n").unwrap();

    assert!(result.len() == 50);

    assert_files_equal(path_expected, path_tmp);
}

#[test]
fn interpolate_linear_test() {
    let path_expected = "tests/data/expected_interpolate_linear_save1.csv";
    let path_tmp = "tests/tmp/interpolate_linear_save1.tmp";
    let mut pc = PointCloud2D::<Point2D>::new();

    pc.push(Point2D::new(0.0, 0.0));
    pc.push(Point2D::new(1.0, 0.0));
    pc.push(Point2D::new(1.0, 1.0));
    pc.push(Point2D::new(0.5, 1.0));

    let result = interpolation_linear(&pc, 50).unwrap();

    if GENERATE_EXCEPTED_RESULT_FILES {
        save_xy(
            &mut File::create(&path_expected).unwrap(),
            &result,
            ";",
            "\n",
        )
        .unwrap();
    }

    save_xy(&mut File::create(&path_tmp).unwrap(), &result, ";", "\n").unwrap();

    assert!(result.len() == 50);

    assert_files_equal(path_expected, path_tmp);
}
