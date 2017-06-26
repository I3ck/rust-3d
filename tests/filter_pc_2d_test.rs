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
use rust_3d::traits::is_filter::*;
use rust_3d::point_2d::*;
use rust_3d::positive::*;
use rust_3d::view::*;
use rust_3d::filters::filter_box_2d::*;
use rust_3d::point_cloud_2d::*;
use rust_3d::traits::is_filter_random_accessible::*;
use rust_3d::filters::transformers::filter_random_accessible::*;
use rust_3d::traits::is_random_insertible::*;

#[test]
fn filter_pc_2d_test() {
    let center = *Point2D::build(10.0, -5.0);
    let size_x = Positive::new(3.0).unwrap();
    let size_y = Positive::new(5.0).unwrap();
    let filter = FilterBox2D::build(center, size_x, size_y);
    let mut pc = PointCloud2D::<Point2D>::new();
    let mut view = View::Full;
    pc.push(*Point2D::build(10.0, -5.0)); // 0  YES
    pc.push(*Point2D::build(9.0, -5.0));  // 1  YES
    pc.push(*Point2D::build(8.0, -5.0));  // 2  NO
    pc.push(*Point2D::build(11.0, -5.0)); // 3  YES
    pc.push(*Point2D::build(12.0, -5.0)); // 4  NO
    pc.push(*Point2D::build(10.0, -4.0)); // 5  YES
    pc.push(*Point2D::build(10.0, -3.0)); // 6  YES
    pc.push(*Point2D::build(10.0, -2.0)); // 7  NO
    pc.push(*Point2D::build(10.0, -6.0)); // 8  YES
    pc.push(*Point2D::build(10.0, -7.0)); // 9  YES
    pc.push(*Point2D::build(10.0, -8.0)); // 10 NO
    // => 0 1 3 5 6 8 9 in => len = 7

    let filter_pc = FilterRandomAccessible::new(filter);
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
            assert!( indices.contains(&(6 as usize)));
            assert!(!indices.contains(&(7 as usize)));
            assert!( indices.contains(&(8 as usize)));
            assert!( indices.contains(&(9 as usize)));
            assert!(!indices.contains(&(10 as usize)));
        }
    }
}
