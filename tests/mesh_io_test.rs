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

extern crate rust_3d;

use rust_3d::traits::is_mesh_3d::*;
use rust_3d::traits::is_editable_mesh_3d::*;
use rust_3d::point_3d::*;
use rust_3d::mesh_3d::*;
use rust_3d::io::ply::*;
use rust_3d::io::stl::*;

#[test]
fn mesh_io_test() {
    let mut m = Mesh3D::<Point3D>::new();
    load_ply_ascii(&mut m, "tests/data/torus_only_vertex_data.ply");
    assert!(m.num_faces()    == 1152);
    assert!(m.num_vertices() == 3456);


    save_ply_ascii(&m, "tests/tmp/torus_only_vertex_data.ply");

    let mut m = Mesh3D::<Point3D>::new();
    load_ply_ascii(&mut m, "tests/tmp/torus_only_vertex_data.ply");
    assert!(m.num_faces()    == 1152);
    assert!(m.num_vertices() == 3456);
}
