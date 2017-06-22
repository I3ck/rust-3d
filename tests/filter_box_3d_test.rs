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

use rust_3d::traits::is_buildable_3d::*;
use rust_3d::traits::is_filter::*;
use rust_3d::traits::is_filter_random_accessible::*;
use rust_3d::traits::is_view_buildable::*;
use rust_3d::point_3d::*;
use rust_3d::point_cloud_3d::*;
use rust_3d::positive::*;
use rust_3d::filters::filter_box_3d::*;
use rust_3d::filters::transformers::filter_random_accessible::*;
use rust_3d::view::*;
use rust_3d::io::xyz::*;
use rust_3d::test_helper::assert_files_equal;

#[test]
fn filter_box_3d_test() {
    let path_expected = "tests/data/expected_filter_box_3d.xyz";
    let path_tmp = "tests/tmp/filter_box_3d.xyz";

    let center = *Point3D::build(10.0, 10.0, 10.0);
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(2.1).unwrap();
    let size_z = Positive::new(2.1).unwrap();
    let filter = FilterRandomAccessible::build(FilterBox3D::build(center, size_x, size_y, size_z));

    let mut view = View::Full;
    let mut pc = PointCloud3D::<Point3D>::new();
    load_xyz(&mut pc, "tests/data/test_cube.xyz", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view);
    save_xyz(&pc, &path_tmp, " ", "\n");
    assert_files_equal(path_expected, path_tmp);
}
