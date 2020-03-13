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

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use core::{convert::TryFrom, str::FromStr};

use std::io::{BufRead, Read, Write};

//------------------------------------------------------------------------------

/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<M, P, W>(write: &mut W, mesh: &M) -> PlyResult<()>
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
    write.write_all(header.as_bytes())?;

    for i in 0..mesh.num_vertices() {
        let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating num_vertices
        write.write_all((vertex.to_str() + "\n").as_bytes())?;
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating num_faces
        write.write_all(
            ("3 ".to_string()
                + &face.a.to_string()
                + " "
                + &face.b.to_string()
                + " "
                + &face.c.to_string()
                + "\n")
                .as_bytes(),
        )?;
    }
    Ok(())
}

//------------------------------------------------------------------------------

/// Saves an IsMesh3D in the ASCII .ply file format with additional colors
pub fn save_ply_ascii_colored<M, P, W>(write: &mut W, mesh: &M, colors: &Vec<Rgb>) -> PlyResult<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let n_vertices = mesh.num_vertices();
    let n_faces = mesh.num_faces();

    if n_vertices != colors.len() {
        return Err(PlyError::ColorArrayIncorrectLength);
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
    write.write_all(header.as_bytes())?;

    for i in 0..n_vertices {
        let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating n_vertices
        let color = &colors[i];
        write.write_all(
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
        )?;
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating n_faces
        write.write_all(
            ("3 ".to_string()
                + &face.a.to_string()
                + " "
                + &face.b.to_string()
                + " "
                + &face.c.to_string()
                + "\n")
                .as_bytes(),
        )?;
    }
    Ok(())
}

//------------------------------------------------------------------------------

/// Saves an IsMesh3D in the binary .ply file format
pub fn save_ply_binary<M, P, W>(write: &mut W, mesh: &M, precision: &Precision) -> PlyResult<()>
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
                + "property float x\n"
                + "property float y\n"
                + "property float z\n"
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
                + "property double x\n"
                + "property double y\n"
                + "property double z\n"
                + "element face "
                + &mesh.num_faces().to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    write.write_all(header.as_bytes())?;

    match precision {
        Precision::P32 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating num_vertices
                write.write_f32::<BigEndian>(vertex.x() as f32)?;
                write.write_f32::<BigEndian>(vertex.y() as f32)?;
                write.write_f32::<BigEndian>(vertex.z() as f32)?;
            }
        }

        Precision::P64 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating num_vertices
                write.write_f64::<BigEndian>(vertex.x())?;
                write.write_f64::<BigEndian>(vertex.y())?;
                write.write_f64::<BigEndian>(vertex.z())?;
            }
        }
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating num_faces
        write.write_u8(3)?;
        write.write_u32::<BigEndian>(face.a.val as u32)?;
        write.write_u32::<BigEndian>(face.b.val as u32)?;
        write.write_u32::<BigEndian>(face.c.val as u32)?;
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Saves an IsMesh3D in the binary .ply file format with additional colors
pub fn save_ply_binary_colored<M, P, W>(
    write: &mut W,
    mesh: &M,
    precision: &Precision,
    colors: &Vec<Rgb>,
) -> PlyResult<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let n_vertices = mesh.num_vertices();
    let n_faces = mesh.num_faces();

    if n_vertices != colors.len() {
        return Err(PlyError::ColorArrayIncorrectLength);
    }

    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
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
                + "property double x\n"
                + "property double y\n"
                + "property double z\n"
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

    write.write_all(header.as_bytes())?;

    match precision {
        Precision::P32 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating n_vertices
                let color = &colors[i];
                write.write_f32::<BigEndian>(vertex.x() as f32)?;
                write.write_f32::<BigEndian>(vertex.y() as f32)?;
                write.write_f32::<BigEndian>(vertex.z() as f32)?;
                write.write_u8(color.r)?;
                write.write_u8(color.g)?;
                write.write_u8(color.b)?;
            }
        }

        Precision::P64 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating n_vertices
                let color = &colors[i];
                write.write_f64::<BigEndian>(vertex.x())?;
                write.write_f64::<BigEndian>(vertex.y())?;
                write.write_f64::<BigEndian>(vertex.z())?;
                write.write_u8(color.r)?;
                write.write_u8(color.g)?;
                write.write_u8(color.b)?;
            }
        }
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating n_faces
        write.write_u8(3)?;
        write.write_u32::<BigEndian>(face.a.val as u32)?;
        write.write_u32::<BigEndian>(face.b.val as u32)?;
        write.write_u32::<BigEndian>(face.c.val as u32)?;
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the .ply file format
pub fn load_ply<EM, P, R>(read: &mut R, mesh: &mut EM) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = String::new();
    let mut i_line = 0;

    let header = load_header(read, &mut line_buffer, &mut i_line)?;

    //println!("{:?}", header);

    mesh.reserve_vertices(header.n_vertices);
    mesh.reserve_faces(header.n_faces);

    match header.format {
        Format::Ascii => load_ascii(read, mesh, &header, &mut line_buffer, &mut i_line),
        Format::LittleEndian => load_binary::<LittleEndian, _, _, _>(read, mesh, &header),
        Format::BigEndian => load_binary::<BigEndian, _, _, _>(read, mesh, &header),
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

fn load_header<R>(read: &mut R, line_buffer: &mut String, i_line: &mut usize) -> PlyResult<Header>
where
    R: BufRead,
{
    let mut vertex_order = [Xyz::X, Xyz::X, Xyz::X];
    let mut i_vertex_order = 0;

    let mut ply_found = false;
    let mut read_state = HeaderReadState::Meta;
    let mut format = None;
    let mut n_vertices: Option<usize> = None;
    let mut n_faces: Option<usize> = None;

    let mut x_type = None;
    let mut y_type = None;
    let mut z_type = None;
    let mut n_types_found = 0;
    //@todo rename these, since now misleading whether vertex or face
    let mut before = BytesWords::default();
    let mut between_first_snd = BytesWords::default();
    let mut between_snd_third = BytesWords::default();
    let mut after = BytesWords::default();

    let mut face_count_type = None;
    let mut face_index_type = None;
    let mut face_before = BytesWords::default();
    let mut face_after = BytesWords::default();
    let mut face_structure_found = false;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        *i_line += 1;

        if line.starts_with("comment") {
            continue;
        }

        if line.starts_with("obj_info") {
            continue;
        }

        if !ply_found {
            if line == "ply" {
                ply_found = true;
                continue;
            }
            return Err(PlyError::LoadStartNotFound);
        }

        if format.is_none() {
            format = Some(match line {
                "format ascii 1.0" => Format::Ascii,
                "format binary_little_endian 1.0" => Format::LittleEndian,
                "format binary_big_endian 1.0" => Format::BigEndian,
                _ => return Err(PlyError::LoadFormatNotFound),
            });
            continue;
        }

        match n_vertices {
            None => {
                if line.starts_with("element vertex") {
                    read_state = HeaderReadState::Vertex;
                    let mut words = to_words(&line);
                    match words.clone().count() {
                        3 => {
                            n_vertices = Some(
                                usize::from_str(words.nth(2).unwrap())
                                    .map_err(|_| PlyError::LineParse(*i_line))?,
                            );
                            continue;
                        }
                        _ => return Err(PlyError::LineParse(*i_line)),
                    }
                }
            }
            Some(_) => {}
        }

        match n_faces {
            None => {
                if line.starts_with("element face") {
                    read_state = HeaderReadState::Face;
                    let mut words = to_words(&line);
                    match words.clone().count() {
                        3 => {
                            n_faces = Some(
                                usize::from_str(words.nth(2).unwrap())
                                    .map_err(|_| PlyError::LineParse(*i_line))?,
                            );
                            continue;
                        }
                        _ => return Err(PlyError::LineParse(*i_line)),
                    }
                }
            }
            Some(_) => {}
        }

        if line.starts_with("property") {
            match read_state {
                HeaderReadState::Vertex => {
                    let mut words = to_words(line);
                    skip_n(&mut words, 1); // skip "property"

                    let t = Type::try_from(words.next().unwrap())?; //@todo error handling, invalid property line
                    let id = words.next().unwrap(); //@todo see above
                    if id == "x" {
                        x_type = Some(VertexType::try_from(t)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::X;
                        i_vertex_order += 1;
                    } else if id == "y" {
                        y_type = Some(VertexType::try_from(t)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::Y;
                        i_vertex_order += 1;
                    } else if id == "z" {
                        z_type = Some(VertexType::try_from(t)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::Z;
                        i_vertex_order += 1;
                    } else {
                        if n_types_found == 0 {
                            before.bytes += t.size_bytes();
                            before.words += 1;
                        } else if n_types_found == 1 {
                            between_first_snd.bytes += t.size_bytes();
                            between_first_snd.words += 1;
                        } else if n_types_found == 2 {
                            between_snd_third.bytes += t.size_bytes();
                            between_snd_third.words += 1;
                        } else {
                            after.bytes += t.size_bytes();
                            after.words += 1;
                        }
                    }
                }
                HeaderReadState::Face => {
                    if line.starts_with("property list") {
                        if line.contains("vertex_indices") || line.contains("vertex_index") {
                            //@todo is this properly defined?
                            let mut words = to_words(line);
                            skip_n(&mut words, 2); // skip "property" and "list"

                            let t_count = FaceType::try_from(Type::try_from(
                                words.next().ok_or(PlyError::InvalidProperty(*i_line))?,
                            )?)?;
                            let t_index = FaceType::try_from(Type::try_from(
                                words.next().ok_or(PlyError::InvalidProperty(*i_line))?,
                            )?)?;

                            face_count_type = Some(t_count);
                            face_index_type = Some(t_index);

                            //@todo remove bool
                            //@todo later parse the real structure here
                            face_structure_found = true
                        } else {
                            //@todo better error
                            //@todo currently can't handle multiple property list lines (also unlikely)
                            return Err(PlyError::LineParse(*i_line));
                        }
                    } else {
                        let mut words = to_words(line);
                        skip_n(&mut words, 1); // skip "property"
                        let t = Type::try_from(
                            words.next().ok_or(PlyError::InvalidProperty(*i_line))?,
                        )?;
                        if face_structure_found {
                            face_after.bytes += t.size_bytes();
                            face_after.words += 1;
                        } else {
                            face_before.bytes += t.size_bytes();
                            face_before.words += 1;
                        }
                    }
                    //@todo
                }
                _ => return Err(PlyError::LineParse(*i_line)), //@todo better error
            }

            continue;
        }

        //@todo use if let
        if line == "end_header"
            && ply_found
            && format.is_some()
            && n_vertices.is_some()
            && n_faces.is_some()
            && x_type.is_some()
            && y_type.is_some()
            && z_type.is_some()
            && face_count_type.is_some()
            && face_index_type.is_some()
        {
            //@todo nicer way to write this
            // safe due to if above
            return Ok(Header {
                format: format.unwrap(),
                n_vertices: n_vertices.unwrap(),
                n_faces: n_faces.unwrap(),
                vertex_format: VertexFormat {
                    order: VertexOrder::from_arr(vertex_order).unwrap(), //@todo this unwrap is currently not safe
                    first: x_type.unwrap(),
                    snd: y_type.unwrap(),
                    third: z_type.unwrap(),
                    before,
                    between_first_snd,
                    between_snd_third,
                    after,
                },
                face_format: FaceFormat {
                    before: face_before,
                    after: face_after,
                    count: face_count_type.unwrap(),
                    index: face_index_type.unwrap(),
                },
            });
        }

        //@todo better error (header could not be parsed / incorrect)
        return Err(PlyError::LoadHeaderEndNotFound);
    }

    Err(PlyError::LoadHeaderEndNotFound)
}

//------------------------------------------------------------------------------

fn load_binary<BO, EM, P, R>(read: &mut R, mesh: &mut EM, header: &Header) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
    BO: ByteOrder,
{
    for _ in 0..header.n_vertices {
        skip_bytes(read, header.vertex_format.before.bytes);

        let first = read_vertex_type::<BO, _>(read, &header.vertex_format.first)?;

        skip_bytes(read, header.vertex_format.between_first_snd.bytes);

        let snd = read_vertex_type::<BO, _>(read, &header.vertex_format.snd)?;

        skip_bytes(read, header.vertex_format.between_snd_third.bytes);

        let third = read_vertex_type::<BO, _>(read, &header.vertex_format.third)?;

        skip_bytes(read, header.vertex_format.after.bytes);

        //@todo duplicate code, write helper
        let p = match header.vertex_format.order {
            VertexOrder::Xyz => P::new(first, snd, third),
            VertexOrder::Xzy => P::new(first, third, snd),
            VertexOrder::Yxz => P::new(snd, first, third),
            VertexOrder::Yzx => P::new(snd, third, first),
            VertexOrder::Zxy => P::new(third, first, snd),
            VertexOrder::Zyx => P::new(third, snd, first),
        };
        mesh.add_vertex(p);
    }

    for _ in 0..header.n_faces {
        skip_bytes(read, header.face_format.before.bytes);

        let element_count = read_face_type::<BO, _>(read, &header.face_format.count)?;

        if element_count != 3 {
            return Err(PlyError::LineParse(0)); //@todo incorrect face structure
        }

        let a = read_face_type::<BO, _>(read, &header.face_format.index)?;
        let b = read_face_type::<BO, _>(read, &header.face_format.index)?;
        let c = read_face_type::<BO, _>(read, &header.face_format.index)?;

        skip_bytes(read, header.face_format.after.bytes);

        //@todo new error without line information!?
        mesh.try_add_connection(
            VId { val: a as usize },
            VId { val: b as usize },
            VId { val: c as usize },
        )
        .map_err(|_| PlyError::InvalidMeshIndices(0))?;
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_ascii<EM, P, R>(
    read: &mut R,
    mesh: &mut EM,
    header: &Header,
    line_buffer: &mut String,
    i_line: &mut usize,
) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    loop {
        line_buffer.clear();
        let n_read = read.read_line(line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        *i_line += 1;

        if header.n_vertices > mesh.num_vertices() {
            let mut words = line.split(" ").skip_empty_string();

            skip_n(&mut words, header.vertex_format.before.words);

            let first = f64::from_str(words.next().unwrap()).unwrap(); //@todo unwrap

            skip_n(&mut words, header.vertex_format.between_first_snd.words);

            let snd = f64::from_str(words.next().unwrap()).unwrap(); //@todo unwrap

            skip_n(&mut words, header.vertex_format.between_snd_third.words);

            let third = f64::from_str(words.next().unwrap()).unwrap(); //@todo unwrap
                                                                       // no need to skip 'after' since we're done with this line anyway

            //@todo duplicate code, write helper
            let p = match header.vertex_format.order {
                VertexOrder::Xyz => P::new(first, snd, third),
                VertexOrder::Xzy => P::new(first, third, snd),
                VertexOrder::Yxz => P::new(snd, first, third),
                VertexOrder::Yzx => P::new(snd, third, first),
                VertexOrder::Zxy => P::new(third, first, snd),
                VertexOrder::Zyx => P::new(third, snd, first),
            };
            mesh.add_vertex(p);

            continue;
        }

        if header.n_faces > mesh.num_faces() {
            let [a, b, c] = collect_index_line(&line).ok_or(PlyError::LineParse(*i_line))?;
            mesh.try_add_connection(VId { val: a }, VId { val: b }, VId { val: c })
                .map_err(|_| PlyError::InvalidMeshIndices(*i_line))?;
            continue;
        }
    }

    if header.n_vertices != mesh.num_vertices() {
        return Err(PlyError::LoadVertexCountIncorrect);
    }

    Ok(())
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

fn collect_index_line(line: &str) -> Option<[usize; 3]> {
    let mut words = to_words(line);
    if words.next()? != "3" {
        return None;
    }

    let a = usize::from_str(words.next()?).ok()?;
    let b = usize::from_str(words.next()?).ok()?;
    let c = usize::from_str(words.next()?).ok()?;

    Some([a, b, c])
}

//------------------------------------------------------------------------------

fn read_face_type<BO, R>(read: &mut R, t: &FaceType) -> PlyResult<usize>
where
    BO: ByteOrder,
    R: Read,
{
    Ok(match t {
        FaceType::Char => read.read_i8()? as usize,
        FaceType::UChar => read.read_u8()? as usize,
        FaceType::Short => read.read_i16::<BO>()? as usize,
        FaceType::UShort => read.read_u16::<BO>()? as usize,
        FaceType::Int => read.read_i32::<BO>()? as usize,
        FaceType::UInt => read.read_u32::<BO>()? as usize,
    })
}

//------------------------------------------------------------------------------

fn read_vertex_type<BO, R>(read: &mut R, t: &VertexType) -> PlyResult<f64>
where
    BO: ByteOrder,
    R: Read,
{
    Ok(match t {
        VertexType::Float => read.read_f32::<BO>()? as f64,
        VertexType::Double => read.read_f64::<BO>()?,
    })
}

//------------------------------------------------------------------------------

fn skip_bytes<R>(read: &mut R, n_bytes: usize)
where
    R: Read,
{
    for _ in 0..n_bytes {
        let _ = read.read_u8();
    }
}

fn skip_n<I>(i: &mut I, n: usize)
where
    I: Iterator,
{
    for _ in 0..n {
        i.next();
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

enum Type {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}

impl Type {
    pub fn size_bytes(&self) -> usize {
        match self {
            Self::Char => 1,
            Self::UChar => 1,
            Self::Short => 2,
            Self::UShort => 2,
            Self::Int => 4,
            Self::UInt => 4,
            Self::Float => 4,
            Self::Double => 8,
        }
    }
}

impl TryFrom<&str> for Type {
    type Error = PlyError;

    fn try_from(x: &str) -> PlyResult<Self> {
        match x {
            "char" => Ok(Self::Char),
            "uchar" => Ok(Self::UChar),
            "short" => Ok(Self::Short),
            "ushort" => Ok(Self::UShort),
            "int" => Ok(Self::Int),
            "uint" => Ok(Self::UInt),
            "float" | "float32" => Ok(Self::Float),
            "double" | "float64" => Ok(Self::Double),
            _ => Err(PlyError::InvalidType(x.to_string())),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
enum Xyz {
    X,
    Y,
    Z,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
enum VertexOrder {
    Xyz,
    Xzy,
    Yxz,
    Yzx,
    Zxy,
    Zyx,
}

impl VertexOrder {
    //@todo try_from
    pub fn from_arr(x: [Xyz; 3]) -> Option<Self> {
        println!("{:?}", x);
        match x {
            [Xyz::X, Xyz::Y, Xyz::Z] => Some(Self::Xyz),
            [Xyz::X, Xyz::Z, Xyz::Y] => Some(Self::Xzy),
            [Xyz::Y, Xyz::X, Xyz::Z] => Some(Self::Yxz),
            [Xyz::Y, Xyz::Z, Xyz::X] => Some(Self::Yzx),
            [Xyz::Z, Xyz::X, Xyz::Y] => Some(Self::Zxy),
            [Xyz::Z, Xyz::Y, Xyz::X] => Some(Self::Zyx),
            _ => None,
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Default, Debug)]
struct BytesWords {
    pub bytes: usize,
    pub words: usize,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
enum VertexType {
    Float,
    Double,
}

impl TryFrom<Type> for VertexType {
    type Error = PlyError;

    fn try_from(x: Type) -> PlyResult<Self> {
        match x {
            Type::Float => Ok(Self::Float),
            Type::Double => Ok(Self::Double),
            _ => Err(PlyError::InvalidVertexType),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
enum FaceType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
}

impl TryFrom<Type> for FaceType {
    type Error = PlyError;

    fn try_from(x: Type) -> PlyResult<Self> {
        match x {
            Type::Char => Ok(Self::Char),
            Type::UChar => Ok(Self::UChar),
            Type::Short => Ok(Self::Short),
            Type::UShort => Ok(Self::UShort),
            Type::Int => Ok(Self::Int),
            Type::UInt => Ok(Self::UInt),
            _ => Err(PlyError::InvalidFaceType),
        }
    }
}

//------------------------------------------------------------------------------

//@todo property list must also be considered
//@todo must consider case where properties / to skip are defined per face and not per vertex
//@todo settings this must track its scope (if after element vertex or element face)
#[derive(Debug)]
struct VertexFormat {
    pub order: VertexOrder,
    pub first: VertexType,
    pub snd: VertexType,
    pub third: VertexType,
    pub before: BytesWords,
    pub between_first_snd: BytesWords,
    pub between_snd_third: BytesWords,
    pub after: BytesWords,
}

//------------------------------------------------------------------------------

//@todo must also check structure itself, not just padding
#[derive(Debug)]
struct FaceFormat {
    pub count: FaceType,
    pub index: FaceType,
    pub before: BytesWords,
    pub after: BytesWords,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
enum Format {
    Ascii,
    LittleEndian,
    BigEndian,
}

//------------------------------------------------------------------------------

enum HeaderReadState {
    Meta,
    Vertex,
    Face,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
struct Header {
    pub format: Format,
    pub n_vertices: usize,
    pub n_faces: usize,
    pub vertex_format: VertexFormat,
    pub face_format: FaceFormat,
}
