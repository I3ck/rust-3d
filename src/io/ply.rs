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

//! Module for IO operations of the ply file format

use crate::*;

use byteorder::{BigEndian, WriteBytesExt};

use core::str::FromStr;

use std::io::{BufRead, Write};

/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<M, P, W>(write: &mut W, mesh: &M) -> Result<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let header = "ply\n".to_string()
        + "format ascii 1.0\n"
        + "comment Created by rust-3d\n"
        + "element vertex "
        + &mesh.num_vertices().to_string()
        + "\n"
        + "property float x\n"
        + "property float y\n"
        + "property float z\n"
        + "element face "
        + &mesh.num_faces().to_string()
        + "\n"
        + "property list uchar uint vertex_indices\n"
        + "end_header\n";
    write
        .write_all(header.as_bytes())
        .map_err(|e| e.to_error_kind())?;

    for i in 0..mesh.num_vertices() {
        let vertex = mesh.vertex(VId { val: i })?;
        write
            .write_all((vertex.to_str() + "\n").as_bytes())
            .map_err(|e| e.to_error_kind())?;
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId { val: i })?;
        write
            .write_all(
                ("3 ".to_string()
                    + &face.a.to_string()
                    + " "
                    + &face.b.to_string()
                    + " "
                    + &face.c.to_string()
                    + "\n")
                    .as_bytes(),
            )
            .map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

/// Saves an IsMesh3D in the ASCII .ply file format with additional colors
pub fn save_ply_ascii_colored<M, P, W>(write: &mut W, mesh: &M, colors: &Vec<Rgb>) -> Result<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let n_vertices = mesh.num_vertices();
    let n_faces = mesh.num_faces();

    if n_vertices != colors.len() {
        return Err(ErrorKind::ColorArrayIncorrectLength);
    }

    let header = "ply\n".to_string()
        + "format ascii 1.0\n"
        + "comment Created by rust-3d\n"
        + "element vertex "
        + &n_vertices.to_string()
        + "\n"
        + "property float x\n"
        + "property float y\n"
        + "property float z\n"
        + "property uchar red\n"
        + "property uchar green\n"
        + "property uchar blue\n"
        + "element face "
        + &n_faces.to_string()
        + "\n"
        + "property list uchar uint vertex_indices\n"
        + "end_header\n";
    write
        .write_all(header.as_bytes())
        .map_err(|e| e.to_error_kind())?;

    for i in 0..n_vertices {
        let vertex = mesh.vertex(VId { val: i })?;
        let color = &colors[i];
        write
            .write_all(
                format!(
                    "{} {} {} {} {} {}\n",
                    vertex.x(),
                    vertex.y(),
                    vertex.z(),
                    color.r,
                    color.g,
                    color.b
                )
                .as_bytes(),
            )
            .map_err(|e| e.to_error_kind())?;
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId { val: i })?;
        write
            .write_all(
                ("3 ".to_string()
                    + &face.a.to_string()
                    + " "
                    + &face.b.to_string()
                    + " "
                    + &face.c.to_string()
                    + "\n")
                    .as_bytes(),
            )
            .map_err(|e| e.to_error_kind())?;
    }
    Ok(())
}

/// Saves an IsMesh3D in the binary .ply file format
pub fn save_ply_binary<M, P, W>(write: &mut W, mesh: &M, precision: &Precision) -> Result<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &mesh.num_vertices().to_string()
                + "\n"
                + "property float32 x\n"
                + "property float32 y\n"
                + "property float32 z\n"
                + "element face "
                + &mesh.num_faces().to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
        Precision::P64 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &mesh.num_vertices().to_string()
                + "\n"
                + "property float64 x\n"
                + "property float64 y\n"
                + "property float64 z\n"
                + "element face "
                + &mesh.num_faces().to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    write
        .write_all(header.as_bytes())
        .map_err(|e| e.to_error_kind())?;

    match precision {
        Precision::P32 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId { val: i })?;
                write
                    .write_f32::<BigEndian>(vertex.position_nd(0)? as f32)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f32::<BigEndian>(vertex.position_nd(1)? as f32)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f32::<BigEndian>(vertex.position_nd(2)? as f32)
                    .map_err(|e| e.to_error_kind())?;
            }
        }

        Precision::P64 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId { val: i })?;
                write
                    .write_f64::<BigEndian>(vertex.position_nd(0)?)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f64::<BigEndian>(vertex.position_nd(1)?)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f64::<BigEndian>(vertex.position_nd(2)?)
                    .map_err(|e| e.to_error_kind())?;
            }
        }
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId { val: i })?;
        write.write_u8(3).map_err(|e| e.to_error_kind())?;
        write
            .write_u32::<BigEndian>(face.a.val as u32)
            .map_err(|e| e.to_error_kind())?;
        write
            .write_u32::<BigEndian>(face.b.val as u32)
            .map_err(|e| e.to_error_kind())?;
        write
            .write_u32::<BigEndian>(face.c.val as u32)
            .map_err(|e| e.to_error_kind())?;
    }

    Ok(())
}

/// Saves an IsMesh3D in the binary .ply file format with additional colors
pub fn save_ply_binary_colored<M, P, W>(
    write: &mut W,
    mesh: &M,
    precision: &Precision,
    colors: &Vec<Rgb>,
) -> Result<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let n_vertices = mesh.num_vertices();
    let n_faces = mesh.num_faces();

    if n_vertices != colors.len() {
        return Err(ErrorKind::ColorArrayIncorrectLength);
    }

    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &n_vertices.to_string()
                + "\n"
                + "property float32 x\n"
                + "property float32 y\n"
                + "property float32 z\n"
                + "property uchar red\n"
                + "property uchar green\n"
                + "property uchar blue\n"
                + "element face "
                + &n_faces.to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
        Precision::P64 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &n_vertices.to_string()
                + "\n"
                + "property float64 x\n"
                + "property float64 y\n"
                + "property float64 z\n"
                + "property uchar red\n"
                + "property uchar green\n"
                + "property uchar blue\n"
                + "element face "
                + &n_faces.to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    write
        .write_all(header.as_bytes())
        .map_err(|e| e.to_error_kind())?;

    match precision {
        Precision::P32 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId { val: i })?;
                let color = &colors[i];
                write
                    .write_f32::<BigEndian>(vertex.position_nd(0)? as f32)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f32::<BigEndian>(vertex.position_nd(1)? as f32)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f32::<BigEndian>(vertex.position_nd(2)? as f32)
                    .map_err(|e| e.to_error_kind())?;
                write.write_u8(color.r).map_err(|e| e.to_error_kind())?;
                write.write_u8(color.g).map_err(|e| e.to_error_kind())?;
                write.write_u8(color.b).map_err(|e| e.to_error_kind())?;
            }
        }

        Precision::P64 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId { val: i })?;
                let color = &colors[i];
                write
                    .write_f64::<BigEndian>(vertex.position_nd(0)?)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f64::<BigEndian>(vertex.position_nd(1)?)
                    .map_err(|e| e.to_error_kind())?;
                write
                    .write_f64::<BigEndian>(vertex.position_nd(2)?)
                    .map_err(|e| e.to_error_kind())?;
                write.write_u8(color.r).map_err(|e| e.to_error_kind())?;
                write.write_u8(color.g).map_err(|e| e.to_error_kind())?;
                write.write_u8(color.b).map_err(|e| e.to_error_kind())?;
            }
        }
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId { val: i })?;
        write.write_u8(3).map_err(|e| e.to_error_kind())?;
        write
            .write_u32::<BigEndian>(face.a.val as u32)
            .map_err(|e| e.to_error_kind())?;
        write
            .write_u32::<BigEndian>(face.b.val as u32)
            .map_err(|e| e.to_error_kind())?;
        write
            .write_u32::<BigEndian>(face.c.val as u32)
            .map_err(|e| e.to_error_kind())?;
    }

    Ok(())
}

// @todo allows incorrect headers and might fail on correct ones
/// Loads an IsMesh3D from the ASCII .ply file format
pub fn load_ply_ascii<EM, P, R>(read: &mut R, mesh: &mut EM) -> Result<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut found_ply = false;
    let mut format_found = false;
    let mut vertex_indices_found = false;
    let mut header_ended = false;

    let mut counted_properties = 0;

    let mut vertex_count: Option<usize> = None;
    let mut face_count: Option<usize> = None;

    for line_result in read.lines() {
        let line = &line_result?;
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
                                vertex_count =
                                    Some(usize::from_str(words[2]).map_err(|e| e.to_error_kind())?);
                                continue;
                            }
                            _ => return Err(ErrorKind::PlyError(PlyError::LoadError)),
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
                                face_count =
                                    Some(usize::from_str(words[2]).map_err(|e| e.to_error_kind())?);
                                continue;
                            }
                            _ => return Err(ErrorKind::PlyError(PlyError::LoadError)),
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
                return Err(ErrorKind::PlyError(
                    PlyError::LoadVertexIndexDefinitionNotFound,
                ));
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
            None => {
                return Err(ErrorKind::PlyError(PlyError::LoadVertexCountNotFound));
            }
            Some(x) => {
                if x > mesh.num_vertices() {
                    let p = P::parse(line)?;
                    mesh.add_vertex(p);
                    continue;
                }
            }
        }

        match face_count {
            None => {
                return Err(ErrorKind::PlyError(PlyError::LoadFaceCountNotFound));
            }
            Some(x) => {
                if x > mesh.num_faces() {
                    let [a, b, c] = collect_index_line(line)?;
                    mesh.try_add_connection(VId { val: a }, VId { val: b }, VId { val: c })?;
                    continue;
                }
            }
        }
    }

    match vertex_count {
        None => {
            return Err(ErrorKind::PlyError(PlyError::LoadVertexCountNotFound));
        }
        Some(x) => {
            if x != mesh.num_vertices() {
                return Err(ErrorKind::PlyError(PlyError::LoadVertexCountIncorrect));
            }
        }
    }

    Ok(())
}

fn collect_index_line(line: &str) -> Result<[usize; 3]> {
    let words = to_words(line);
    match words.len() {
        4 => {
            if words[0] != "3" {
                return Err(ErrorKind::PlyError(PlyError::IncorrectFaceData));
            }
            let a = usize::from_str(words[1]).map_err(|e| e.to_error_kind())?;
            let b = usize::from_str(words[2]).map_err(|e| e.to_error_kind())?;
            let c = usize::from_str(words[3]).map_err(|e| e.to_error_kind())?;
            Ok([a, b, c])
        }
        _ => Err(ErrorKind::PlyError(PlyError::LoadError)),
    }
}
