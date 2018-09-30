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
extern crate byteorder;

use prelude::*;

use utils::to_words;
use rgb::Rgb;

use self::core::str::FromStr;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use self::byteorder::{BigEndian, WriteBytesExt};

/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<M, P>(mesh: &M, filepath: &str) -> Result<()> where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D {

    let mut f = BufWriter::new(File::create(filepath).map_err(|e| e.to_error_kind())?);

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

/// Saves an IsMesh3D in the ASCII .ply file format with additional colors
pub fn save_ply_ascii_colored<M, P>(mesh: &M, filepath: &str, colors: &Vec<Rgb>) -> Result<()> where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D {

    let n_vertices = mesh.num_vertices();
    let n_faces    = mesh.num_faces();

    if n_vertices != colors.len() { return Err(ErrorKind::ColorArrayIncorrectLength) }

    let mut f = BufWriter::new(File::create(filepath).map_err(|e| e.to_error_kind())?);

    let header = "ply\n".to_string()
                   + "format ascii 1.0\n"
                   + "comment Created by rust-3d\n"
                   + "element vertex " + &n_vertices.to_string() + "\n"
                   + "property float x\n"
                   + "property float y\n"
                   + "property float z\n"
                   + "property uchar red\n"
                   + "property uchar green\n"
                   + "property uchar blue\n"
                   + "element face " + &n_faces.to_string() + "\n"
                   + "property list uchar uint vertex_indices\n"
                   + "end_header\n";
    f.write_all(header.as_bytes()).map_err(|e| e.to_error_kind())?;

    for i in 0..n_vertices {
        let vertex = mesh.vertex(VId{val: i})?;
        let color = &colors[i];
        f.write_all(format!("{} {} {} {} {} {}\n", vertex.x(), vertex.y(), vertex.z(), color.r, color.g, color.b).as_bytes()).map_err(|e| e.to_error_kind())?;
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId{val: i})?;
        f.write_all(("3 ".to_string() + &face.a.to_string() + " " + &face.b.to_string() + " " + &face.c.to_string() + "\n").as_bytes()).map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

/// Saves an IsMesh3D in the binary .ply file format
pub fn save_ply_binary<M, P>(mesh: &M, filepath: &str, precision: &Precision) -> Result<()> where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D {

    let mut f = BufWriter::new(File::create(filepath).map_err(|e| e.to_error_kind())?);

    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex " + &mesh.num_vertices().to_string() + "\n"
                + "property float32 x\n"
                + "property float32 y\n"
                + "property float32 z\n"
                + "element face " + &mesh.num_faces().to_string() + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        },
        Precision::P64 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex " + &mesh.num_vertices().to_string() + "\n"
                + "property float64 x\n"
                + "property float64 y\n"
                + "property float64 z\n"
                + "element face " + &mesh.num_faces().to_string() + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    f.write_all(header.as_bytes()).map_err(|e| e.to_error_kind())?;

    match precision {
        Precision::P32 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId{val: i})?;
                f.write_f32::<BigEndian>(vertex.get_position(0)? as f32).map_err(|e| e.to_error_kind())?;
                f.write_f32::<BigEndian>(vertex.get_position(1)? as f32).map_err(|e| e.to_error_kind())?;
                f.write_f32::<BigEndian>(vertex.get_position(2)? as f32).map_err(|e| e.to_error_kind())?;
            }
        },

        Precision::P64 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId{val: i})?;
                f.write_f64::<BigEndian>(vertex.get_position(0)?).map_err(|e| e.to_error_kind())?;
                f.write_f64::<BigEndian>(vertex.get_position(1)?).map_err(|e| e.to_error_kind())?;
                f.write_f64::<BigEndian>(vertex.get_position(2)?).map_err(|e| e.to_error_kind())?;
            }
        }
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId{val: i})?;
        f.write_u8(3).map_err(|e| e.to_error_kind())?;
        f.write_u32::<BigEndian>(face.a.val as u32).map_err(|e| e.to_error_kind())?;
        f.write_u32::<BigEndian>(face.b.val as u32).map_err(|e| e.to_error_kind())?;
        f.write_u32::<BigEndian>(face.c.val as u32).map_err(|e| e.to_error_kind())?;
    }

    Ok(())
}

/// Saves an IsMesh3D in the binary .ply file format with additional colors
pub fn save_ply_binary_colored<M, P>(mesh: &M, filepath: &str, precision: &Precision, colors: &Vec<Rgb>) -> Result<()> where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D {

    let n_vertices = mesh.num_vertices();
    let n_faces    = mesh.num_faces();

    if n_vertices != colors.len() { return Err(ErrorKind::ColorArrayIncorrectLength) }

    let mut f = BufWriter::new(File::create(filepath).map_err(|e| e.to_error_kind())?);

    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex " + &n_vertices.to_string() + "\n"
                + "property float32 x\n"
                + "property float32 y\n"
                + "property float32 z\n"
                + "property uchar red\n"
                + "property uchar green\n"
                + "property uchar blue\n"
                + "element face " + &n_faces.to_string() + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        },
        Precision::P64 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex " + &n_vertices.to_string() + "\n"
                + "property float64 x\n"
                + "property float64 y\n"
                + "property float64 z\n"
                + "property uchar red\n"
                + "property uchar green\n"
                + "property uchar blue\n"
                + "element face " + &n_faces.to_string() + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    f.write_all(header.as_bytes()).map_err(|e| e.to_error_kind())?;

    match precision {
        Precision::P32 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId{val: i})?;
                let color  = &colors[i];
                f.write_f32::<BigEndian>(vertex.get_position(0)? as f32).map_err(|e| e.to_error_kind())?;
                f.write_f32::<BigEndian>(vertex.get_position(1)? as f32).map_err(|e| e.to_error_kind())?;
                f.write_f32::<BigEndian>(vertex.get_position(2)? as f32).map_err(|e| e.to_error_kind())?;
                f.write_u8              (color.r).map_err(|e| e.to_error_kind())?;
                f.write_u8              (color.g).map_err(|e| e.to_error_kind())?;
                f.write_u8              (color.b).map_err(|e| e.to_error_kind())?;
            }
        },

        Precision::P64 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId{val: i})?;
                let color  = &colors[i];
                f.write_f64::<BigEndian>(vertex.get_position(0)?).map_err(|e| e.to_error_kind())?;
                f.write_f64::<BigEndian>(vertex.get_position(1)?).map_err(|e| e.to_error_kind())?;
                f.write_f64::<BigEndian>(vertex.get_position(2)?).map_err(|e| e.to_error_kind())?;
                f.write_u8              (color.r).map_err(|e| e.to_error_kind())?;
                f.write_u8              (color.g).map_err(|e| e.to_error_kind())?;
                f.write_u8              (color.b).map_err(|e| e.to_error_kind())?;
            }
        }
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId{val: i})?;
        f.write_u8(3).map_err(|e| e.to_error_kind())?;
        f.write_u32::<BigEndian>(face.a.val as u32).map_err(|e| e.to_error_kind())?;
        f.write_u32::<BigEndian>(face.b.val as u32).map_err(|e| e.to_error_kind())?;
        f.write_u32::<BigEndian>(face.c.val as u32).map_err(|e| e.to_error_kind())?;
    }

    Ok(())
}

// @todo allows incorrect headers and might fail on correct ones
/// Loads an IsMesh3D from the ASCII .ply file format
pub fn load_ply_ascii<EM, P>(mesh: &mut EM, filepath: &str) -> Result<()> where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone {

        let mut f       = BufReader::new(File::open(filepath)?);
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
                        vertices.push(p);
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

        fill_mesh(vertices, indices, mesh)
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

fn fill_mesh<EM, P>(vertices: Vec<P>, indices: Vec<usize>, mesh: &mut EM) -> Result<()> where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone {

    if indices.len() == 0 {
        return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
    }

    if indices.len() % 3 != 0 {
        return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
    }

    for vertex in vertices {
        mesh.add_vertex(vertex);
    }

    for chunk in indices.chunks(3) {
        if chunk.len() == 3 {
                mesh.try_add_connection(VId{val:chunk[0]}, VId{val:chunk[1]}, VId{val:chunk[2]})?;
        } else {
            return Err(ErrorKind::PlyError(PlyError::LoadVerticesIncorrect));
        }
    }

    Ok(())
}
