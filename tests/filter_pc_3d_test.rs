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



use rust_3d::prelude::*;

#[test]
fn filter_pc_3d_test() {
    let center = Point3D {
        x: 10.0,
        y: -5.0,
        z: 1.0,
    };
    let size_x = Positive::new(3.0).unwrap();
    let size_y = Positive::new(5.0).unwrap();
    let size_z = Positive::new(10.0).unwrap();
    let filter = FilterBox3D::new(Box3D {
        center: center,
        size_x: size_x,
        size_y: size_y,
        size_z: size_z,
    });

    let mut pc = PointCloud3D::<Point3D>::new();
    let mut view = View::Full;
    pc.push(Point3D::new(10.0, -5.0, 1.0)); // 0 YES
    pc.push(Point3D::new(9.0, -5.0, 1.0)); // 1 YES
    pc.push(Point3D::new(8.0, -5.0, 1.0)); // 2 NO
    pc.push(Point3D::new(11.0, -5.0, 1.0)); // 3 YES
    pc.push(Point3D::new(12.0, -5.0, 1.0)); // 4 NO
    pc.push(Point3D::new(10.0, -3.0, 1.0)); // 5 YES
    pc.push(Point3D::new(10.0, -2.0, 1.0)); // 6 NO
    pc.push(Point3D::new(10.0, -7.0, 1.0)); // 7 YES
    pc.push(Point3D::new(10.0, -8.0, 1.0)); // 8 NO
    pc.push(Point3D::new(10.0, -5.0, -3.0)); // 9 YES
    pc.push(Point3D::new(10.0, -5.0, -5.0)); // 10 NO
    pc.push(Point3D::new(10.0, -5.0, 5.0)); // 11 YES
    pc.push(Point3D::new(10.0, -5.0, 7.0)); // 12 NO
                                            // => 0 1 3 5 7 9 11  in => len = 7

    let filter_pc = FilterRandomAccessible::new(filter);
    filter_pc.filter(&pc, &mut view);

    match view {
        View::Full => assert!(false),
        View::Restricted(indices) => {
            assert!(indices.len() == 7);

            assert!(indices.contains(&(0 as usize)));
            assert!(indices.contains(&(1 as usize)));
            assert!(!indices.contains(&(2 as usize)));
            assert!(indices.contains(&(3 as usize)));
            assert!(!indices.contains(&(4 as usize)));
            assert!(indices.contains(&(5 as usize)));
            assert!(!indices.contains(&(6 as usize)));
            assert!(indices.contains(&(7 as usize)));
            assert!(!indices.contains(&(8 as usize)));
            assert!(indices.contains(&(9 as usize)));
            assert!(!indices.contains(&(10 as usize)));
            assert!(indices.contains(&(11 as usize)));
            assert!(!indices.contains(&(12 as usize)));
        }
    }
}
