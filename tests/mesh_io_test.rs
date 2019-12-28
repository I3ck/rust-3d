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

use rust_3d::{io::*, *};

use std::fs::File;

#[test]
fn mesh_io_test() {
    let mut m = Mesh3D::<Point3D>::default();
    load_ply_ascii(
        &mut File::open("tests/data/torus_only_vertex_data.ply").unwrap(),
        &mut m,
    )
    .unwrap();
    assert!(m.num_faces() == 1152);
    assert!(m.num_vertices() == 576);

    save_ply_ascii(
        &mut File::create("tests/tmp/torus_only_vertex_data.ply").unwrap(),
        &m,
    )
    .unwrap();

    let mut m = Mesh3D::<Point3D>::default();
    load_ply_ascii(
        &mut File::open("tests/tmp/torus_only_vertex_data.ply").unwrap(),
        &mut m,
    )
    .unwrap();
    assert!(m.num_faces() == 1152);
    assert!(m.num_vertices() == 576);
}
