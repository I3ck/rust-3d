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
    assert!(mesh.face_vertex_ids(0).is_err());
    assert!(mesh.face_vertices(0).is_err());
    assert!(mesh.vertex(0).is_err());
    assert!(mesh.face_normal(0).is_err());

    mesh.add_vertex(*Point3D::new(0.0, 0.1, 0.2));
    assert!(mesh.num_vertices() == 1);
    assert!(mesh.num_faces() == 0);

    mesh.add_vertex(*Point3D::new(0.1, 0.2, 0.3));
    mesh.add_vertex(*Point3D::new(0.2, 0.3, 0.4));
    assert!(mesh.num_vertices() == 3);
    assert!(mesh.num_faces() == 0);

    assert!(mesh.try_add_connection(0, 0, 0).is_err());
    assert!(mesh.try_add_connection(0, 1, 1).is_err());
    assert!(mesh.try_add_connection(0, 1, 3).is_err());
    assert!(mesh.try_add_connection(0, 1, 2).is_ok());
    assert!(mesh.num_vertices() == 3);
    assert!(mesh.num_faces() == 1);

    assert!(mesh.add_face(
        *Point3D::new(1.0, 1.0, 1.0),
        *Point3D::new(2.0, 2.0, 2.0),
        *Point3D::new(3.0, 3.0, 3.0))
        == 1);
    assert!(mesh.num_vertices() == 6);
    assert!(mesh.num_faces() == 2);

    match mesh.face_vertex_ids(0) {
        Err(_) => assert!(false),
        Ok((id1, id2, id3)) => assert!(id1 == 0 && id2 == 1 && id3 == 2)
    };

    match mesh.face_vertex_ids(1) {
        Err(_) => assert!(false),
        Ok((id1, id2, id3)) => assert!(id1 == 3 && id2 == 4 && id3 == 5)
    };

    match mesh.face_vertices(0) {
        Err(_) => assert!(false),
        Ok((p1, p2, p3)) => assert!(
               p1.x() == 0.0
            && p2.x() == 0.1
            && p3.x() == 0.2
        )
    };

    match mesh.face_vertices(1) {
        Err(_) => assert!(false),
        Ok((p1, p2, p3)) => assert!(
               p1.x() == 1.0
            && p2.x() == 2.0
            && p3.x() == 3.0
        )
    };

    //@todo missing tests for fileIO

}
