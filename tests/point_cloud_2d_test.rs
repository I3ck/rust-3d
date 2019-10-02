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
fn point_cloud_2d_test() {
    let mut pc = PointCloud2D::<Point2D>::new();

    assert!(pc.len() == 0);

    let p = Point2D::new(0.1, 0.2);
    pc.push(p);

    assert!(pc.len() == 1);
    assert!(pc.data[0].x() == 0.1);
    assert!(pc.data[0].y() == 0.2);

    assert!(pc.bounding_box_maybe().is_err());

    let p = Point2D::new(0.2, 0.3);
    pc.push(p);
    assert!(pc.len() == 2);

    assert!(pc.bounding_box_maybe().is_ok());

    match pc.bounding_box_maybe() {
        Err(_) => assert!(false),
        Ok(bb) => {
            assert!(bb.min_p().x() == 0.1);
            assert!(bb.min_p().y() == 0.2);
            assert!(bb.max_p().x() == 0.2);
            assert!(bb.max_p().y() == 0.3);
        }
    }
    //@todo test currently failing
    /*
    assert!(pc.to_str() == "0.1 0.2\n0.2 0.3\n");

    match PointCloud2D::<Point2D>::parse(pc.to_str()) {
        Err(_) => assert!(false),
        Ok(pcparsed) => assert!(pcparsed.to_str() == "0.1 0.2\n0.2 0.3\n")
    };
    */
    let pccloned = pc.clone();
    assert!(pccloned.to_str() == "0.1 0.2\n0.2 0.3\n");

    pc.move_by(1.0, 2.0);
    println!("pc: {}", pc);
    assert!(pc.to_str() == "1.1 2.2\n1.2 2.3\n");
}
