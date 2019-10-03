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

use rust_3d::{prelude::*, test_helper::*};

#[test]
fn filter_box_3d_test() {
    let center = Point3D {
        x: 10.0,
        y: 10.0,
        z: 10.0,
    };
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(2.1).unwrap();
    let size_z = Positive::new(2.1).unwrap();
    test_filter_3d::<_, Point3D>(
        FilterBox3D::new(Box3D {
            center: center,
            size_x: size_x,
            size_y: size_y,
            size_z: size_z,
        }),
        "tests/data/expected_filter_box_3d.xyz",
        "box3d",
    );
}

#[test]
fn filter_sphere_test() {
    let center = Point3D {
        x: 10.0,
        y: 10.0,
        z: 10.0,
    };
    let radius = Positive::new(4.0).unwrap();
    let filter = FilterSphere::new(Sphere {
        center: center,
        radius: radius,
    });
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_sphere.xyz", "sphere");
}
