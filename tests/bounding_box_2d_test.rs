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

#[test]
fn bounding_box_2d_test() {
    let mut pc1 = PointCloud2D::<Point2D>::new();
    let mut pc2 = PointCloud2D::<Point2D>::new();
    let mut pc3 = PointCloud2D::<Point2D>::new();
    let mut pc4 = PointCloud2D::<Point2D>::new();

    pc1.push(Point2D::new(0.0, 0.0));
    pc1.push(Point2D::new(1.0, 1.0));

    pc2.push(Point2D::new(0.0, 0.0));
    pc2.push(Point2D::new(0.5, 0.5));

    pc3.push(Point2D::new(-1.0, -1.0));
    pc3.push(Point2D::new(2.0, 2.0));

    pc4.push(Point2D::new(-10.0, -10.0));
    pc4.push(Point2D::new(-11.0, -11.0));

    let bb1 = pc1.bounding_box().unwrap();
    let bb2 = pc2.bounding_box().unwrap();
    let bb3 = pc3.bounding_box().unwrap();
    let bb4 = pc4.bounding_box().unwrap();

    assert!(bb1.min_p().x() == 0.0);
    assert!(bb1.min_p().y() == 0.0);
    assert!(bb1.max_p().x() == 1.0);
    assert!(bb1.max_p().y() == 1.0);

    assert!(bb3.min_p().x() == -1.0);
    assert!(bb3.min_p().y() == -1.0);
    assert!(bb3.max_p().x() == 2.0);
    assert!(bb3.max_p().y() == 2.0);

    assert!(!bb4.is_inside(&bb1));
    assert!(!bb4.is_inside(&bb2));
    assert!(!bb4.is_inside(&bb3));

    assert!(!bb1.is_inside(&bb2));
    assert!(!bb1.has_inside(&bb2));

    assert!(!bb2.is_inside(&bb1));
    assert!(!bb2.has_inside(&bb1));

    assert!(bb1.collides_with(&bb2));
    assert!(bb2.collides_with(&bb1));

    assert!(bb3.has_inside(&bb1));
    assert!(bb3.has_inside(&bb2));
    assert!(bb3.collides_with(&bb1));
    assert!(bb3.collides_with(&bb2));

    assert!(!bb1.contains(&Point2D::new(5.0, 5.0)));
    assert!(bb1.contains(&Point2D::new(0.5, 0.5)));
}
