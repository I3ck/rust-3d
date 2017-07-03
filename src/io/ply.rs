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

//! Module for IO operations of the ply file format

extern crate core;

use result::*;
use traits::is_buildable_3d::*;
use traits::is_mesh_3d::*;
use traits::is_editable_mesh_3d::*;

use self::core::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

//@todo change to take IsMesh as template
/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<P>(mesh: &IsMesh3D<P>, filepath: &str) -> Result<()> where
    P: IsBuildable3D {

    let mut f = File::create(filepath).map_err(|e| e.to_error_kind())?;

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
    f.write_all(header.as_bytes()).map_err(|e| e.to_error_kind())?;

    for i in 0..mesh.num_vertices() {
        let vertex = mesh.vertex(i)?;
        f.write_all((vertex.to_str() + "\n").as_bytes()).map_err(|e| e.to_error_kind())?;
    }

    for i in 0..mesh.num_faces() {
        let (vid1, vid2, vid3) = mesh.face_vertex_ids(i)?;
        f.write_all(("3 ".to_string() + &vid1.to_string() + " " + &vid2.to_string() + " " + &vid3.to_string() + "\n").as_bytes()).map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

#[allow(unused_assignments)] //@todo vertex_count and face_count create this warning. Find way to fix this without supressing
// @todo impl is really ugly currently, rework it
// @todo allows incorrect headers and might fail on correct ones
/// Loads an IsMesh3D from the ASCII .ply file format
pub fn load_ply_ascii<EM, P>(mesh: &mut EM, filepath: &str) -> Result<()> where
    EM: IsEditableMesh3D<P>,
    P: IsBuildable3D + Clone {

        //@todo either assume the mesh is empty and fail
        //@todo or make algorithm work for appending

        let mut f = File::open(filepath)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let lines = content.split("\n");

        let mut found_ply = false;
        let mut format_found = false;
        let mut vertex_count: Option<usize> = None;
        let mut counted_properties = 0;
        let mut face_count: Option<usize> = None;
        let mut vertex_indices_found = false;
        let mut header_ended = false;

        let mut vertices = Vec::<P>::new();
        let mut indices = Vec::<usize>::new();


        for line in lines {

            if !found_ply {
                if line == "ply" {
                    found_ply = true;
                    continue;
                }
                return Err(ErrorKind::ParseError);
            }

            if !format_found {
                if line == "format ascii 1.0" {
                    format_found = true;
                    continue;
                }
                return Err(ErrorKind::ParseError);
            }

            if line.starts_with("comment") {
                continue;
            }

            match vertex_count {
                None => {
                    if line.starts_with("element vertex") {
                        let split = line.split(" ");
                        let words = split.collect::<Vec<&str>>();
                        match words.len() {
                            3 => {
                                vertex_count = Some(usize::from_str(words[2]).map_err(|e| e.to_error_kind())?);
                            },
                            _ => return Err(ErrorKind::ParseError)
                        }
                    }
                    return Err(ErrorKind::ParseError);
                }
                Some(_) => {}
            }

            if line.starts_with("property float") {
                counted_properties += 1;
                continue;
            }

            if counted_properties != 3 {
                return Err(ErrorKind::ParseError);
            }

            match face_count {
                None => {
                    if line.starts_with("element face") {
                        let split = line.split(" ");
                        let words = split.collect::<Vec<&str>>();
                        match words.len() {
                            3 => {
                                face_count = Some(usize::from_str(words[2]).map_err(|e| e.to_error_kind())?);
                            },
                            _ => return Err(ErrorKind::ParseError)
                        }
                    }
                    return Err(ErrorKind::ParseError);
                }
                Some(_) => {}
            }

            if !vertex_indices_found {
                if line.ends_with("vertex_indices") {
                    vertex_indices_found = true;
                    continue;
                }
                return Err(ErrorKind::ParseError);
            }

            if !header_ended {
                if line == "end_header" {
                    header_ended = true;
                    continue;
                }
                return Err(ErrorKind::ParseError);
            }

            match vertex_count {
                None => { return Err(ErrorKind::ParseError); }
                Some(x) => {
                    if x >= vertices.len() {
                        let p = P::parse(line.to_string())?;
                        vertices.push(*p);
                        continue;
                    }
                }
            }

            match face_count {
                None => { return Err(ErrorKind::ParseError); }
                Some(x) => {
                    if x >= indices.len() / 3 {
                        let split = line.split(" ");
                        let words = split.collect::<Vec<&str>>();

                        match words.len() {
                            4 => {
                                //@todo ensure words[0] == 3 for tri faces
                                let a = usize::from_str(words[1]).map_err(|e| e.to_error_kind())?;
                                let b = usize::from_str(words[2]).map_err(|e| e.to_error_kind())?;
                                let c = usize::from_str(words[3]).map_err(|e| e.to_error_kind())?;
                                indices.push(a);
                                indices.push(b);
                                indices.push(c);
                            },
                            _ => { return Err(ErrorKind::ParseError); }
                        }
                    }
                }
            }
            //@todo fail if too many lines?
        }

        //@todo assert all sizes correct (counts with vec sizes, indices divisible by 3 etc)

        //@todo ensure the index access here is correct (ply could contain incorrect ids)

        ///@todo ensure size > 0 since chunks panics otherwise
        let n_vertices = vertices.len();

        match vertex_count {
            None => { return Err(ErrorKind::ParseError); }
            Some(x) => {
                if x != n_vertices {
                    return Err(ErrorKind::ParseError);
                }
            }
        }

        if indices.len() == 0 {
            return Err(ErrorKind::ParseError);
        }

        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                if chunk[0] < n_vertices && chunk[1] < n_vertices && chunk[2] < n_vertices {
                    mesh.add_face(vertices[chunk[0]].clone(), vertices[chunk[1]].clone(), vertices[chunk[2]].clone());
                } //@todo else error
            } //@todo else error
        }

        //@todo use collected data to fill mesh

        Ok(())
    }
