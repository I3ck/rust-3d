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

//! Module for IO operations on meshes

use result::*;
use traits::is_3d::*;
use traits::is_buildable_3d::*;
use traits::is_mesh_3d::*;

use std::io::prelude::*;
use std::fs::File;

/// Saves an IsMesh3D in the ASCII .stl file format
pub fn save_stl_ascii<P>(mesh: &IsMesh3D<P>, filepath: &str) -> Result<()> where
    P: IsBuildable3D { //@todo .stl cant have negative coordinates, therefore must be offset by BB (or show error)

    let mut f = try!(File::create(filepath).map_err(|e| e.to_error_kind()));

    //@todo precision?
    try!(f.write_all(b"solid \n").map_err(|e| e.to_error_kind()));

    for i in 0..mesh.num_faces() {
        let (v1, v2, v3) = try!(mesh.face_vertices(i));
        let n = try!(mesh.face_normal(i));
        let buffer = "facet normal ".to_string() + &n.to_str() + "\n"
                       + "    outer loop\n"
                       + "        vertex " + &v1.to_str() + "\n"
                       + "        vertex " + &v2.to_str() + "\n"
                       + "        vertex " + &v3.to_str() + "\n"
                       + "    end loop\n"
                       + "endfacet\n";
        try!(f.write_all(buffer.as_bytes()).map_err(|e| e.to_error_kind()));
    }
    f.write_all(b"solid \n").map_err(|e| e.to_error_kind())
}

/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<P>(mesh: &IsMesh3D<P>, filepath: &str) -> Result<()> where
    P: IsBuildable3D {

    let mut f = try!(File::create(filepath).map_err(|e| e.to_error_kind()));

    //@todo remove unnecessary comments in header
    //@todo better header, or let caller decide
    let header = "ply\n".to_string()
                   + "format ascii 1.0           { ascii/binary, format version number }\n"
                   + "TODO comment  { comments keyword specified, like all lines }\n"
                   + "TODO more comments \n"
                   + "element vertex " + &mesh.num_vertices().to_string() + "\n"
                   + "property float x           { vertex contains float \"x\" coordinate }\n"
                   + "property float y           { y coordinate is also a vertex property }\n"
                   + "property float z           { z coordinate, too }\n"
                   + "element face " + &mesh.num_faces().to_string() + "\n"
                   + "property list uchar int vertex_index { \"vertex_indices\" is a list of ints }\n"
                   + "end_header                 { delimits the end of the header }\n";
    try!(f.write_all(header.as_bytes()).map_err(|e| e.to_error_kind()));

    for i in 0..mesh.num_vertices() {
        let vertex = try!(mesh.vertex(i));
        try!(f.write_all((vertex.to_str() + "\n").as_bytes()).map_err(|e| e.to_error_kind()));
    }

    for i in 0..mesh.num_faces() {
        let (vid1, vid2, vid3) = try!(mesh.face_vertex_ids(i));
        try!(f.write_all(("3 ".to_string() + &vid1.to_string() + " " + &vid2.to_string() + " " + &vid3.to_string() + "\n").as_bytes()).map_err(|e| e.to_error_kind()));
    }
    Ok(())
}
