use traits::is_3d::Is3D;
use std::io::prelude::*;
use std::fs::File;

pub trait IsMesh3D<P> where
    P: Is3D {

    fn num_faces(&self) -> usize;

    fn num_vertices(&self) -> usize;

    fn face_vertex_ids(&self, faceid: usize) -> Option<(usize, usize, usize)>;

    fn face_vertices(&self, faceid: usize) -> Option<(P, P, P)>;

    fn vertex(&self, vertexid: usize) -> Option<P>;

    fn save_stl_ascii(&self, filepath: &str) -> bool { //@todo .stl cant have negative coordinates, therefore must be offset by BB (or show error)

        let mut f = match File::create(filepath) {
            Err(_) => return false,
            Ok(f) => f
        };

        //@todo precision?
        f.write_all(b"solid \n");

        for i in 0..self.num_faces() {
            let (v1, v2, v3) = match self.face_vertices(i) {
                None => return false,
                Some((v1, v2, v3)) => (v1, v2, v3)
            };
            f.write_all(b"facet normal 0 0 0\n"); //@todo normals missing
            f.write_all(b"    outer loop\n");
            f.write_all(("        vertex ".to_string() + &v1.to_str() + "\n").as_bytes());
            f.write_all(("        vertex ".to_string() + &v2.to_str() + "\n").as_bytes());
            f.write_all(("        vertex ".to_string() + &v3.to_str() + "\n").as_bytes());
            f.write_all(b"    end loop\n");
            f.write_all(b"endfacet\n");
        }
        f.write_all(b"solid \n");
        true
    }

    fn save_ply_ascii(&self, filepath: &str) -> bool {

        let mut f = match File::create(filepath) {
            Err(_) => return false,
            Ok(f) => f
        };

        let nVertexString = "element vertex ".to_string() + &self.num_vertices().to_string() + "\n";
        let nFacesString = "element face ".to_string() + &self.num_faces().to_string() + "\n";

        //@todo better header, or let caller decide
        f.write_all(b"ply\n");
        f.write_all(b"format ascii 1.0           { ascii/binary, format version number }\n");
        f.write_all(b"comment made by Greg Turk  { comments keyword specified, like all lines }\n");
        f.write_all(b"comment this file is a cube\n");
        f.write_all(nVertexString.as_bytes());
        f.write_all(b"property float x           { vertex contains float \"x\" coordinate }\n");
        f.write_all(b"property float y           { y coordinate is also a vertex property }\n");
        f.write_all(b"property float z           { z coordinate, too }\n");
        f.write_all(nFacesString.as_bytes());
        f.write_all(b"property list uchar int vertex_index { \"vertex_indices\" is a list of ints }\n");
        f.write_all(b"end_header                 { delimits the end of the header }\n");

        for i in 0..self.num_vertices() {
            let vertex = match self.vertex(i) {
                None => return false,
                Some(v) => v
            };
            f.write_all((vertex.to_str() + "\n").as_bytes());
        }

        for i in 0..self.num_faces() {
            let (vid1, vid2, vid3) = match self.face_vertex_ids(i) {
                None => return false,
                Some((vid1, vid2, vid3)) => (vid1, vid2, vid3)
            };
            f.write_all(("3 ".to_string() + &vid1.to_string() + " " + &vid2.to_string() + " " + &vid3.to_string() + "\n").as_bytes());
        }
        return true;
    }
}