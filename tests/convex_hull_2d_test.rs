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

extern crate rust_3d;

use rust_3d::prelude::*;
use rust_3d::io::*;
use rust_3d::test_helper::*;

#[test]
fn convex_hull_2d_test() {
    //@todo rather weak test, find better input data for this
    let mut pc = PointCloud2D::<Point2D>::new();
    load_xy(&mut pc, "tests/data/test_square.xy", " ", "\n").unwrap();

    let hull = convex_hull_2d(&pc);
    save_xy(&hull, "tests/tmp/tmp_convex_hull_2d.xy", " ", "\n").unwrap();
    assert_files_equal("tests/data/expected_convex_hull_2d.xy", "tests/tmp/tmp_convex_hull_2d.xy");
}
