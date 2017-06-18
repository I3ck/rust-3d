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
use rust_3d::point_3d::*;
use rust_3d::positive::*;
use rust_3d::view::*;
use rust_3d::filters::filter_box_3d::*;
use rust_3d::point_cloud_3d::*;
use rust_3d::traits::is_filter_random_accessible::*;
use rust_3d::filters::filter_random_accessible::*;
use rust_3d::traits::is_random_insertible::*;

#[test]
fn filter_pc_3d_test() {
    let center = *Point3D::build(10.0, -5.0, 1.0);
    let size_x = Positive::new(3.0).unwrap();
    let size_y = Positive::new(5.0).unwrap();
    let size_z = Positive::new(10.0).unwrap();
    let filter = FilterBox3D::build(center, size_x, size_y, size_z);

    let mut pc = PointCloud3D::<Point3D>::new();
    let mut view = View::Full;
    pc.push(*Point3D::build(10.0, -5.0, 1.0));  // 0 YES
    pc.push(*Point3D::build(9.0, -5.0, 1.0));   // 1 YES
    pc.push(*Point3D::build(8.0, -5.0, 1.0));   // 2 NO
    pc.push(*Point3D::build(11.0, -5.0, 1.0));  // 3 YES
    pc.push(*Point3D::build(12.0, -5.0, 1.0));  // 4 NO
    pc.push(*Point3D::build(10.0, -3.0, 1.0));  // 5 YES
    pc.push(*Point3D::build(10.0, -2.0, 1.0));  // 6 NO
    pc.push(*Point3D::build(10.0, -7.0, 1.0));  // 7 YES
    pc.push(*Point3D::build(10.0, -8.0, 1.0));  // 8 NO
    pc.push(*Point3D::build(10.0, -5.0, -3.0)); // 9 YES
    pc.push(*Point3D::build(10.0, -5.0, -5.0)); // 10 NO
    pc.push(*Point3D::build(10.0, -5.0, 5.0));  // 11 YES
    pc.push(*Point3D::build(10.0, -5.0, 7.0));  // 12 NO
    // => 0 1 3 5 7 9 11  in => len = 7

    let filter_pc = FilterRandomAccessible::build(filter);
    filter_pc.filter(&pc, &mut view);

    match view {
        View::Full => assert!(false),
        View::Restricted(indices) => {
            assert!(indices.len() == 7);

            assert!( indices.contains(&(0 as usize)));
            assert!( indices.contains(&(1 as usize)));
            assert!(!indices.contains(&(2 as usize)));
            assert!( indices.contains(&(3 as usize)));
            assert!(!indices.contains(&(4 as usize)));
            assert!( indices.contains(&(5 as usize)));
            assert!(!indices.contains(&(6 as usize)));
            assert!( indices.contains(&(7 as usize)));
            assert!(!indices.contains(&(8 as usize)));
            assert!( indices.contains(&(9 as usize)));
            assert!(!indices.contains(&(10 as usize)));
            assert!( indices.contains(&(11 as usize)));
            assert!(!indices.contains(&(12 as usize)));
        }
    }
}
