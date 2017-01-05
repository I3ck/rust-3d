/*
Copyright 2016 Martin Buck
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

use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_normalized_3d::IsNormalized3D;
use functions::{conn, cross};
use norm_3d::Norm3D;
use std::io::prelude::*;
use std::fs::File;

pub trait IsMesh3D<P> where
    P: IsBuildable3D {

    fn num_faces(&self) -> usize;

    fn num_vertices(&self) -> usize;

    fn face_vertex_ids(&self, faceid: usize) -> Option<(usize, usize, usize)>;

    fn face_vertices(&self, faceid: usize) -> Option<(P, P, P)>;

    fn vertex(&self, vertexid: usize) -> Option<P>;

    fn face_normal(&self, faceid: usize) -> Option<Norm3D> {
        let (v1, v2, v3) = match self.face_vertices(faceid) {
            None => return None,
            Some((v1, v2, v3)) => (v1, v2, v3)
        };

        let v12 = conn(&v1, &v2);
        let v23 = conn(&v2, &v3);

        let n = cross(&v12, &v23);

        match Norm3D::new(*n) {
            None => None,
            Some(b) => Some(*b)
        }
    }

    //@todo change signatures of these methods to return Result instead of bool and use try! for write_all
    fn save_stl_ascii(&self, filepath: &str) -> bool { //@todo .stl cant have negative coordinates, therefore must be offset by BB (or show error)

        let mut f = match File::create(filepath) {
            Err(_) => return false,
            Ok(f) => f
        };

        //@todo precision?
        match f.write_all(b"solid \n") {
            Err(_) => return false,
            Ok(_) => {}
        }

        for i in 0..self.num_faces() {
            let (v1, v2, v3) = match self.face_vertices(i) {
                None => return false,
                Some((v1, v2, v3)) => (v1, v2, v3)
            };
            let n = match self.face_normal(i) {
                None => return false,
                Some(n) => n
            };
            let buffer = "facet normal ".to_string() + &n.to_str() + "\n"
                           + "    outer loop\n"
                           + "        vertex " + &v1.to_str() + "\n"
                           + "        vertex " + &v2.to_str() + "\n"
                           + "        vertex " + &v3.to_str() + "\n"
                           + "    end loop\n"
                           + "endfacet\n";
            match f.write_all(buffer.as_bytes()) {
                Err(_) => return false,
                Ok(_) => {}
            }
        }
        match f.write_all(b"solid \n") {
            Err(_) => return false,
            Ok(_) => return true
        }
    }

    fn save_ply_ascii(&self, filepath: &str) -> bool {

        let mut f = match File::create(filepath) {
            Err(_) => return false,
            Ok(f) => f
        };

        //@todo remove unnecessary comments in header
        //@todo better header, or let caller decide
        let header = "ply\n".to_string()
                       + "format ascii 1.0           { ascii/binary, format version number }\n"
                       + "TODO comment  { comments keyword specified, like all lines }\n"
                       + "TODO more comments \n"
                       + "element vertex " + &self.num_vertices().to_string() + "\n"
                       + "property float x           { vertex contains float \"x\" coordinate }\n"
                       + "property float y           { y coordinate is also a vertex property }\n"
                       + "property float z           { z coordinate, too }\n"
                       + "element face " + &self.num_faces().to_string() + "\n"
                       + "property list uchar int vertex_index { \"vertex_indices\" is a list of ints }\n"
                       + "end_header                 { delimits the end of the header }\n";
        match f.write_all(header.as_bytes()) {
            Err(_) => return false,
            Ok(_) => {}
        }

        for i in 0..self.num_vertices() {
            let vertex = match self.vertex(i) {
                None => return false,
                Some(v) => v
            };
            match f.write_all((vertex.to_str() + "\n").as_bytes()) {
                Err(_) => return false,
                Ok(_) => {}
            }
        }

        for i in 0..self.num_faces() {
            let (vid1, vid2, vid3) = match self.face_vertex_ids(i) {
                None => return false,
                Some((vid1, vid2, vid3)) => (vid1, vid2, vid3)
            };
            match f.write_all(("3 ".to_string() + &vid1.to_string() + " " + &vid2.to_string() + " " + &vid3.to_string() + "\n").as_bytes()) {
                Err(_) => return false,
                Ok(_) => {}
            }
        }
        return true;
    }
}
