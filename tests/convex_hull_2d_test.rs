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

#![deny(warnings)]

extern crate rust_3d;

use rust_3d::prelude::*;
use rust_3d::io::xy::*;
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
