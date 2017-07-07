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

use rust_3d::strong_types::*;
use rust_3d::traits::is_3d::*;
use rust_3d::traits::is_buildable_3d::*;
use rust_3d::traits::is_mesh_3d::*;
use rust_3d::traits::is_editable_mesh_3d::*;
use rust_3d::point_3d::*;
use rust_3d::mesh_3d::*;

#[test]
fn mesh_test() {
    let mut mesh = Mesh3D::<Point3D>::default();

    assert!(mesh.num_faces() == 0);
    assert!(mesh.num_vertices() == 0);
    assert!(mesh.face_vertex_ids(FId{val: 0}).is_err());
    assert!(mesh.face_vertices(FId{val: 0}).is_err());
    assert!(mesh.vertex(VId{val: 0}).is_err());
    assert!(mesh.face_normal(FId{val: 0}).is_err());

    mesh.add_vertex(*Point3D::new(0.0, 0.1, 0.2));
    assert!(mesh.num_vertices() == 1);
    assert!(mesh.num_faces() == 0);

    mesh.add_vertex(*Point3D::new(0.1, 0.2, 0.3));
    mesh.add_vertex(*Point3D::new(0.2, 0.3, 0.4));
    assert!(mesh.num_vertices() == 3);
    assert!(mesh.num_faces() == 0);

    assert!(mesh.try_add_connection(VId{val: 0}, VId{val: 0}, VId{val: 0}).is_err());
    assert!(mesh.try_add_connection(VId{val: 0}, VId{val: 1}, VId{val: 1}).is_err());
    assert!(mesh.try_add_connection(VId{val: 0}, VId{val: 1}, VId{val: 3}).is_err());
    assert!(mesh.try_add_connection(VId{val: 0}, VId{val: 1}, VId{val: 2}).is_ok());
    assert!(mesh.num_vertices() == 3);
    assert!(mesh.num_faces() == 1);

    assert!(mesh.add_face(
        *Point3D::new(1.0, 1.0, 1.0),
        *Point3D::new(2.0, 2.0, 2.0),
        *Point3D::new(3.0, 3.0, 3.0)).val
        == 1);
    assert!(mesh.num_vertices() == 6);
    assert!(mesh.num_faces() == 2);

    match mesh.face_vertex_ids(FId{val: 0}) {
        Err(_) => assert!(false),
        Ok(face) => assert!(face.a.val == 0 && face.b.val == 1 && face.c.val == 2)
    };

    match mesh.face_vertex_ids(FId{val: 1}) {
        Err(_) => assert!(false),
        Ok(face) => assert!(face.a.val == 3 && face.b.val == 4 && face.c.val == 5)
    };

    match mesh.face_vertices(FId{val: 0}) {
        Err(_) => assert!(false),
        Ok((p1, p2, p3)) => assert!(
               p1.x() == 0.0
            && p2.x() == 0.1
            && p3.x() == 0.2
        )
    };

    match mesh.face_vertices(FId{val: 1}) {
        Err(_) => assert!(false),
        Ok((p1, p2, p3)) => assert!(
               p1.x() == 1.0
            && p2.x() == 2.0
            && p3.x() == 3.0
        )
    };
}
