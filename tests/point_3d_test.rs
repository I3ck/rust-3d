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

use rust_3d::traits::is_3d::*;
use rust_3d::traits::is_buildable_3d::*;
use rust_3d::traits::is_editable_3d::*;
use rust_3d::traits::is_moveable_3d::*;
use rust_3d::functions::cross;
use rust_3d::point_3d::*;

#[test]
fn point_3d_test() {
    let eps = 0.0000001;

    let mut p1 = *Point3D::new();

    assert!(p1.x() == 0.0);
    assert!(p1.y() == 0.0);
    assert!(p1.z() == 0.0);
    assert!(p1.abs() == 0.0);

    let mut p2 = *Point3D::build(1.0, 0.0, 0.0);
    assert!(p2.x() == 1.0);
    assert!(p2.y() == 0.0);
    assert!(p2.z() == 0.0);
    assert!(p2.abs() == 1.0);

    let p3 = *Point3D::build(2.0, 2.0, 2.0);

    let cross12 = cross(&p1, &p2);
    assert!(cross12.x() == 0.0);
    assert!(cross12.y() == 0.0);
    assert!(cross12.z() == 0.0);

    assert!(p1.dot(&p2) == 0.0);

    let cross23 = cross(&p2, &p3);
    assert!(cross23.x() == 0.0 * 2.0 - 0.0 * 2.0);
    assert!(cross23.y() == 0.0 * 2.0 - 1.0 * 2.0);
    assert!(cross23.z() == 1.0 * 2.0 - 0.0 * 2.0);


    assert!(p2.pos() == (p2.x(), p2.y(), p2.z()));
    let mut p2_clone = p2.clone();
    assert!(p2_clone.pos() == p2.pos());
    assert!(p2.to_str() == "1 0 0");

    p2_clone.from(p1.clone());
    assert!(p2_clone.pos() == p1.pos());

    let p1_norm = p1.normalized();
    assert!(p1_norm.is_none());

    let p3_norm = p3.normalized();
    assert!(p3_norm.is_some());

    match p3_norm {
        None => {},
        Some(n) => {
            assert!((1.0 - n.abs()).abs() < eps) ;
            assert!(n.x() == p3.x() / p3.abs());
            assert!(n.y() == p3.y() / p3.abs());
            assert!(n.z() == p3.z() / p3.abs());
        }
    }

    p1.set_x(3.0);
    p1.set_y(10.0);
    p1.set_z(11.0);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 10.0);
    assert!(p1.z() == 11.0);

    p1.set_pos(3.0, 11.0, 14.0);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 11.0);
    assert!(p1.z() == 14.0);

    p2.set_pos(1.0, 2.0, 3.0);
    p1.add(&p2);
    assert!(p1.x() == 4.0);
    assert!(p1.y() == 13.0);
    assert!(p1.z() == 17.0);

    p1.substract(&p2);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 11.0);
    assert!(p1.z() == 14.0);

    p1.scale(2.0);
    assert!(p1.x() == 6.0);
    assert!(p1.y() == 22.0);
    assert!(p1.z() == 28.0);

    match Point3D::parse("1.3 7.9 13.7".to_string()) {
        None => assert!(false),
        Some(bp) => {
            assert!(bp.x() == 1.3);
            assert!(bp.y() == 7.9);
            assert!(bp.z() == 13.7);
        }
    }

    p1.set_pos(1.0, 2.0, 3.0);
    p1.move_by(0.1, 0.2, 0.3);
    assert!(p1.x() == 1.1);
    assert!(p1.y() == 2.2);
    assert!(p1.z() == 3.3);

    //@todo missing tests for matrix multiplication
}
