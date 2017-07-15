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

use strong_types::*;
use result::*;
use traits::is_buildable_3d::*;
use traits::is_mesh_3d::*;
use traits::is_editable_mesh::*;
use utils::to_words;

use self::core::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<M, P>(mesh: &M, filepath: &str) -> Result<()> where
    M: IsMesh3D<P>,
    P: IsBuildable3D {

    let mut f = File::create(filepath).map_err(|e| e.to_error_kind())?;

    let header = "ply\n".to_string()
                   + "format ascii 1.0\n"
                   + "comment Created by rust-3d\n"
                   + "element vertex " + &mesh.num_vertices().to_string() + "\n"
                   + "property float x\n"
                   + "property float y\n"
                   + "property float z\n"
                   + "element face " + &mesh.num_faces().to_string() + "\n"
                   + "property list uchar uint vertex_indices\n"
                   + "end_header\n";
    f.write_all(header.as_bytes()).map_err(|e| e.to_error_kind())?;

    for i in 0..mesh.num_vertices() {
        let vertex = mesh.vertex(VId{val: i})?;
        f.write_all((vertex.to_str() + "\n").as_bytes()).map_err(|e| e.to_error_kind())?;
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId{val: i})?;
        f.write_all(("3 ".to_string() + &face.a.to_string() + " " + &face.b.to_string() + " " + &face.c.to_string() + "\n").as_bytes()).map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

// @todo allows incorrect headers and might fail on correct ones
/// Loads an IsMesh3D from the ASCII .ply file format
pub fn load_ply_ascii<EM, P>(mesh: &mut EM, filepath: &str) -> Result<()> where
    EM: IsEditableMesh<P>,
    P: IsBuildable3D + Clone {

        let mut f       = File::open(filepath)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let lines       = content.split("\n");

        let mut found_ply            = false;
        let mut format_found         = false;
        let mut vertex_indices_found = false;
        let mut header_ended         = false;

        let mut counted_properties   = 0;

        let mut vertex_count: Option<usize> = None;
        let mut face_count:   Option<usize> = None;

        let mut vertices = Vec::<P>::new();
        let mut indices  = Vec::<usize>::new();


        for line in lines {
            if !header_ended {
                if !found_ply {
                    if line == "ply" {
                        found_ply = true;
                        continue;
                    }
                    return Err(ErrorKind::PlyError(PlyError::LoadStartNotFound));
                }

                if !format_found {
                    if line == "format ascii 1.0" {
                        format_found = true;
                        continue;
                    }
                    return Err(ErrorKind::PlyError(PlyError::LoadFormatNotFound));
                }

                if line.starts_with("comment") {
                    continue;
                }

                match vertex_count {
                    None => {
                        if line.starts_with("element vertex") {
                            let words = to_words(line);
                            match words.len() {
                                3 => {
                                    vertex_count = Some(usize::from_str(words[2]).map_err(|e| e.to_error_kind())?);
                                    continue;
                                },
                                _ => return Err(ErrorKind::PlyError(PlyError::LoadError))
                            }
                        }
                        return Err(ErrorKind::PlyError(PlyError::LoadError));
                    }
                    Some(_) => {}
                }

                if line.starts_with("property float") {
                    counted_properties += 1;
                    continue;
                }

                if counted_properties < 3 {
                    return Err(ErrorKind::PlyError(PlyError::LoadWrongPropertyCount));
                }

                match face_count {
                    None => {
                        if line.starts_with("element face") {
                            let words = to_words(line);
                            match words.len() {
                                3 => {
                                    face_count = Some(usize::from_str(words[2]).map_err(|e| e.to_error_kind())?);
                                    continue;
                                },
                                _ => return Err(ErrorKind::PlyError(PlyError::LoadError))
                            }
                        }
                        return Err(ErrorKind::PlyError(PlyError::LoadError));
                    }
                    Some(_) => {}
                }

                if !vertex_indices_found {
                    if line.ends_with("vertex_indices") {
                        vertex_indices_found = true;
                        continue;
                    }
                    return Err(ErrorKind::PlyError(PlyError::LoadVertexIndexDefinitionNotFound));
                }

                if !header_ended {
                    if line == "end_header" {
                        header_ended = true;
                        continue;
                    }
                    return Err(ErrorKind::PlyError(PlyError::LoadHeaderEndNotFound));
                }
            }

            match vertex_count {
                None => { return Err(ErrorKind::PlyError(PlyError::LoadVertexCountNotFound)); }
                Some(x) => {
                    if x > vertices.len() {
                        let p = P::parse(line.to_string())?;
                        vertices.push(*p);
                        continue;
                    }
                }
            }

            match face_count {
                None => { return Err(ErrorKind::PlyError(PlyError::LoadFaceCountNotFound)); }
                Some(x) => {
                    if x > indices.len() / 3 {
                        collect_index_line(line, &mut indices)?;
                        continue;
                    }
                }
            }
        }

        match vertex_count {
            None => { return Err(ErrorKind::PlyError(PlyError::LoadVertexCountNotFound)); }
            Some(x) => {
                if x != vertices.len() {
                    return Err(ErrorKind::PlyError(PlyError::LoadVertexCountIncorrect));
                }
            }
        }

        fill_mesh(&vertices, &indices, mesh)
    }

fn collect_index_line(line: &str, indices: &mut Vec<usize>) -> Result<()> {
    let words = to_words(line);
    match words.len() {
        4 => {
            //@todo ensure words[0] == 3 for tri faces
            let a = usize::from_str(words[1]).map_err(|e| e.to_error_kind())?;
            let b = usize::from_str(words[2]).map_err(|e| e.to_error_kind())?;
            let c = usize::from_str(words[3]).map_err(|e| e.to_error_kind())?;
            indices.push(a);
            indices.push(b);
            indices.push(c);
            Ok(())
        },
        _ => Err(ErrorKind::PlyError(PlyError::LoadError))
    }
}

fn fill_mesh<EM, P>(vertices: &Vec<P>, indices: &Vec<usize>, mesh: &mut EM) -> Result<()> where
    EM: IsEditableMesh<P>,
    P: IsBuildable3D + Clone {

    let n_vertices = vertices.len();

    if indices.len() == 0 {
        return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
    }

    if indices.len() % 3 != 0 {
        return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
    }

    for chunk in indices.chunks(3) {
        if chunk.len() == 3 {
            if chunk[0] < n_vertices && chunk[1] < n_vertices && chunk[2] < n_vertices {
                mesh.add_face(vertices[chunk[0]].clone(), vertices[chunk[1]].clone(), vertices[chunk[2]].clone());
            } else {
                return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
            }
        } else {
            return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
        }
    }

    Ok(())
}
