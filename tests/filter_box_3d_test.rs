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
use rust_3d::traits::is_filter_pc_3d::*;
use rust_3d::point_3d::*;
use rust_3d::positive::*;
use rust_3d::filters::filter_box_3d::*;
use rust_3d::filters::filter_pc_3d::*;
use rust_3d::view::*;
use rust_3d::io::pointcloud_3d_io::*;

#[test]
fn filter_box_3d_test() {
    let center = *Point3D::build(10.0, 10.0, 10.0);
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(2.1).unwrap();
    let size_z = Positive::new(2.1).unwrap();
    let filter = FilterPC3D::build(FilterBox3D::build(center, size_x, size_y, size_z));

    let mut view = View::Full;
    let pc = load_xyz::<Point3D>("tests/data/test_cube.xyz", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    match view {
        View::Full => { assert!(false); }
        View::Restricted(indices) => { assert!(indices.len() == 27) }
    }
}
