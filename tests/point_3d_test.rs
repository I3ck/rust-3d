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

use rust_3d::*;

#[test]
fn point_3d_test() {
    let eps = 0.0000001;

    let mut p1 = Point3D::default();

    assert!(p1.x() == 0.0);
    assert!(p1.y() == 0.0);
    assert!(p1.z() == 0.0);
    assert!(p1.abs().get() == 0.0);

    let mut p2 = Point3D::new(1.0, 0.0, 0.0);
    assert!(p2.x() == 1.0);
    assert!(p2.y() == 0.0);
    assert!(p2.z() == 0.0);
    assert!(p2.abs().get() == 1.0);

    let p3 = Point3D::new(2.0, 2.0, 2.0);

    let cross12 = cross(&p1, &p2);
    assert!(cross12.x() == 0.0);
    assert!(cross12.y() == 0.0);
    assert!(cross12.z() == 0.0);

    assert!(p1.dot(&p2) == 0.0);

    let cross23 = cross(&p2, &p3);
    assert!(cross23.x() == 0.0 * 2.0 - 0.0 * 2.0);
    assert!(cross23.y() == 0.0 * 2.0 - 1.0 * 2.0);
    assert!(cross23.z() == 1.0 * 2.0 - 0.0 * 2.0);

    assert!(p2.xyz() == [p2.x(), p2.y(), p2.z()]);
    let mut p2_clone = p2.clone();
    assert!(p2_clone.xyz() == p2.xyz());
    assert!(p2.to_str() == "1 0 0");

    p2_clone.from(&p1.clone());
    assert!(p2_clone.xyz() == p1.xyz());

    let p1_norm = p1.normalized();
    assert!(p1_norm.is_err());

    let p3_norm = p3.normalized();
    assert!(p3_norm.is_ok());

    match p3_norm {
        Err(_) => {}
        Ok(n) => {
            assert!((1.0 - n.abs().get()).abs() < eps);
            assert!(n.x() == p3.x() / p3.abs().get());
            assert!(n.y() == p3.y() / p3.abs().get());
            assert!(n.z() == p3.z() / p3.abs().get());
        }
    }

    p1.set_x(3.0);
    p1.set_y(10.0);
    p1.set_z(11.0);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 10.0);
    assert!(p1.z() == 11.0);

    p1.set_xyz(3.0, 11.0, 14.0);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 11.0);
    assert!(p1.z() == 14.0);

    p2.set_xyz(1.0, 2.0, 3.0);
    p1.add(&p2);
    assert!(p1.x() == 4.0);
    assert!(p1.y() == 13.0);
    assert!(p1.z() == 17.0);

    p1.subtract(&p2);
    assert!(p1.x() == 3.0);
    assert!(p1.y() == 11.0);
    assert!(p1.z() == 14.0);

    p1.scale_pos(2.0);
    assert!(p1.x() == 6.0);
    assert!(p1.y() == 22.0);
    assert!(p1.z() == 28.0);

    match Point3D::parse("1.3 7.9 13.7".to_string()) {
        Err(_) => assert!(false),
        Ok(bp) => {
            assert!(bp.x() == 1.3);
            assert!(bp.y() == 7.9);
            assert!(bp.z() == 13.7);
        }
    }

    p1.set_xyz(1.0, 2.0, 3.0);
    p1.move_by(0.1, 0.2, 0.3);
    assert!(p1.x() == 1.1);
    assert!(p1.y() == 2.2);
    assert!(p1.z() == 3.3);

    //@todo missing tests for matrix multiplication
}
