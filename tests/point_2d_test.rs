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

use std::f64::consts;

use rust_3d::prelude::*;

#[test]
fn point_2d_test() {

    let eps = 0.0000001;
    let origin = Point2D::default();


    let mut p1 = Point2D::default();

    assert!(p1.x() == 0.0);
    assert!(p1.y() == 0.0);
    assert!(p1.abs().get() == 0.0);

    let mut p2 = *Point2D::new(1.0, 0.0);
    assert!(p2.x() == 1.0);
    assert!(p2.y() == 0.0);
    assert!(p2.abs().get() == 1.0);

    assert!(p1.rad_to(&p2).val == 0.0);
    assert!(p2.rad_to(&p1).val == consts::PI);

    let p3 = *Point2D::new(2.0, 2.0);
    assert!(p1.cross(&p2) == 0.0);
    assert!(p1.dot(&p2) == 0.0);
    assert!(p2.cross(&p3) == 2.0);
    assert!(p2.dot(&p3) == 2.0);

    assert!(p2.pos() == (p2.x(), p2.y()));
    let mut p2_clone = p2.clone();
    assert!(p2_clone.pos() == p2.pos());
    assert!(p2.to_str() == "1 0");

    p2_clone.from(p1.clone());
    assert!(p2_clone.pos() == p1.pos());

    let p1_norm = p1.normalized();
    assert!(p1_norm.is_err());

    let p3_norm = p3.normalized();
    assert!(p3_norm.is_ok());

    match p3_norm {
        Err(_) => {},
        Ok(n) => {
            assert!((1.0 - n.abs().get()).abs() < eps) ;
            assert!(n.x() == p3.x() / p3.abs().get());
            assert!(n.y() == p3.y() / p3.abs().get());
        }
    }

    p1.set_x(3.0);
    p1.set_y(10.0);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 10.0);

    p1.set_pos(3.0, 11.0);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 11.0);

    p2.set_pos(1.0, 2.0);
    p1.add(&p2);
    assert!(p1.x() == 4.0);
    assert!(p1.y() == 13.0);

    p1.substract(&p2);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 11.0);

    p1.scale(2.0);
    assert!(p1.x() == 6.0);
    assert!(p1.y() == 22.0);

    p1.set_pos(1.0, 0.0);
    p1.rotate(Rad{val: 0.0}, &origin);
    assert!(p1.x() == 1.0);
    assert!(p1.y() == 0.0);

    p1.rotate(Rad{val: 2.0 * consts::PI}, &origin);
    assert!((1.0 - p1.x()).abs() < eps);
    assert!((0.0 - p1.y()).abs() < eps);


    p1.rotate(Rad{val: consts::PI}, &origin);
    assert!((-1.0 - p1.x()).abs() < eps);
    assert!((0.0 - p1.y()).abs() < eps);

    match Point2D::parse("1.3 7.9".to_string()) {
        Err(_) => assert!(false),
        Ok(bp) => {
            assert!(bp.x() == 1.3);
            assert!(bp.y() == 7.9);
        }
    }

    p1.set_pos(1.0, 2.0);
    p1.move_by(0.1, 0.2);
    assert!(p1.x() == 1.1);
    assert!(p1.y() == 2.2);
}
