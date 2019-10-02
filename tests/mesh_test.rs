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
fn mesh_test() {
    let mut mesh = Mesh3D::<Point3D>::default();

    assert!(mesh.num_faces() == 0);
    assert!(mesh.num_vertices() == 0);
    assert!(mesh.face_vertex_ids(FId { val: 0 }).is_err());
    assert!(mesh.face_vertices(FId { val: 0 }).is_err());
    assert!(mesh.vertex(VId { val: 0 }).is_err());
    assert!(mesh.face_normal(FId { val: 0 }).is_err());

    mesh.add_vertex(Point3D::new(0.0, 0.1, 0.2));
    assert!(mesh.num_vertices() == 1);
    assert!(mesh.num_faces() == 0);

    mesh.add_vertex(Point3D::new(0.1, 0.2, 0.3));
    mesh.add_vertex(Point3D::new(0.2, 0.3, 0.4));
    assert!(mesh.num_vertices() == 3);
    assert!(mesh.num_faces() == 0);

    assert!(mesh
        .try_add_connection(VId { val: 0 }, VId { val: 0 }, VId { val: 0 })
        .is_err());
    assert!(mesh
        .try_add_connection(VId { val: 0 }, VId { val: 1 }, VId { val: 1 })
        .is_err());
    assert!(mesh
        .try_add_connection(VId { val: 0 }, VId { val: 1 }, VId { val: 3 })
        .is_err());
    assert!(mesh
        .try_add_connection(VId { val: 0 }, VId { val: 1 }, VId { val: 2 })
        .is_ok());
    assert!(mesh.num_vertices() == 3);
    assert!(mesh.num_faces() == 1);

    assert!(
        mesh.add_face(
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(2.0, 2.0, 2.0),
            Point3D::new(3.0, 3.0, 3.0)
        )
        .val == 1
    );
    assert!(mesh.num_vertices() == 6);
    assert!(mesh.num_faces() == 2);

    match mesh.face_vertex_ids(FId { val: 0 }) {
        Err(_) => assert!(false),
        Ok(face) => assert!(face.a.val == 0 && face.b.val == 1 && face.c.val == 2),
    };

    match mesh.face_vertex_ids(FId { val: 1 }) {
        Err(_) => assert!(false),
        Ok(face) => assert!(face.a.val == 3 && face.b.val == 4 && face.c.val == 5),
    };

    match mesh.face_vertices(FId { val: 0 }) {
        Err(_) => assert!(false),
        Ok((p1, p2, p3)) => assert!(p1.x() == 0.0 && p2.x() == 0.1 && p3.x() == 0.2),
    };

    match mesh.face_vertices(FId { val: 1 }) {
        Err(_) => assert!(false),
        Ok((p1, p2, p3)) => assert!(p1.x() == 1.0 && p2.x() == 2.0 && p3.x() == 3.0),
    };
}
