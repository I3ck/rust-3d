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

use rust_3d::{test_helper::*, *};

#[test]
fn filter_circle_test() {
    let center = Point2D { x: 10.0, y: 10.0 };
    let size = Positive::new(2.1).unwrap();
    test_filter_2d::<_, Point2D>(
        FilterCircle::new(Circle {
            center: center,
            radius: size,
        }),
        "tests/data/expected_filter_circle.xy",
        "circle",
    );
}

#[test]
fn filter_box_2d_test() {
    let center = Point2D { x: 10.0, y: 10.0 };
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(4.1).unwrap();
    test_filter_2d::<_, Point2D>(
        FilterBox2D::new(Box2D {
            center: center,
            size_x: size_x,
            size_y: size_y,
        }),
        "tests/data/expected_filter_box_2d.xy",
        "box_2d",
    );
}
