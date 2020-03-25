/*
Copyright 2020 Martin Buck

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

//! Module for load functions of the ply file format

use crate::*;

use core::{convert::TryFrom, str::FromStr};

use std::io::{BufRead, Read};

use super::super::utils::*;

use super::{types::*, utils::*};

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

    mesh.reserve_vertices(header.n_vertices);
    mesh.reserve_faces(header.n_faces);

    match header.format {
        Format::Ascii => load_ascii(read, mesh, &header, &mut line_buffer, &mut i_line),
        Format::LittleEndian => load_binary::<LittleReader, _, _, _>(read, mesh, &header),
        Format::BigEndian => load_binary::<BigReader, _, _, _>(read, mesh, &header),
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
    let mut opt_format = None;
    let mut opt_n_vertices: Option<usize> = None;
    let mut opt_n_faces: Option<usize> = None;

    let mut opt_fst_type = None;
    let mut opt_snd_type = None;
    let mut opt_third_type = None;
    let mut n_types_found = 0;
    let mut vertex_before = BytesWords::default();
    let mut vertex_between_first_snd = BytesWords::default();
    let mut vertex_between_snd_third = BytesWords::default();
    let mut after = BytesWords::default();

    let mut opt_face_count_type = None;
    let mut opt_face_index_type = None;
    let mut face_before = BytesWords::default();
    let mut face_after = BytesWords::default();

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

        if opt_format.is_none() {
            opt_format = Some(match line {
                "format ascii 1.0" => Format::Ascii,
                "format binary_little_endian 1.0" => Format::LittleEndian,
                "format binary_big_endian 1.0" => Format::BigEndian,
                _ => return Err(PlyError::LoadFormatNotFound),
            });
            continue;
        }

        match opt_n_vertices {
            None => {
                if line.starts_with("element vertex") {
                    read_state = HeaderReadState::Vertex;
                    let mut words = to_words(&line);
                    opt_n_vertices = Some(
                        usize::from_str(words.nth(2).ok_or(PlyError::LineParse(*i_line))?)
                            .map_err(|_| PlyError::LineParse(*i_line))?,
                    );
                    continue;
                }
            }
            Some(_) => {}
        }

        match opt_n_faces {
            None => {
                if line.starts_with("element face") {
                    read_state = HeaderReadState::Face;
                    let mut words = to_words(&line);
                    opt_n_faces = Some(
                        usize::from_str(words.nth(2).ok_or(PlyError::LineParse(*i_line))?)
                            .map_err(|_| PlyError::LineParse(*i_line))?,
                    );
                    continue;
                }
            }
            Some(_) => {}
        }

        if line.starts_with("property") {
            match read_state {
                HeaderReadState::Vertex => {
                    let mut words = to_words(line);
                    skip_n(&mut words, 1); // skip "property"

                    let t =
                        Type::try_from(words.next().ok_or(PlyError::InvalidProperty(*i_line))?)?;
                    let id = words.next().ok_or(PlyError::InvalidProperty(*i_line))?;
                    if id == "x" {
                        opt_fst_type = Some(VertexType::try_from(t)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::X;
                        i_vertex_order += 1;
                    } else if id == "y" {
                        opt_snd_type = Some(VertexType::try_from(t)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::Y;
                        i_vertex_order += 1;
                    } else if id == "z" {
                        opt_third_type = Some(VertexType::try_from(t)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::Z;
                        i_vertex_order += 1;
                    } else {
                        if n_types_found == 0 {
                            vertex_before.bytes += t.size_bytes();
                            vertex_before.words += 1;
                        } else if n_types_found == 1 {
                            vertex_between_first_snd.bytes += t.size_bytes();
                            vertex_between_first_snd.words += 1;
                        } else if n_types_found == 2 {
                            vertex_between_snd_third.bytes += t.size_bytes();
                            vertex_between_snd_third.words += 1;
                        } else {
                            after.bytes += t.size_bytes();
                            after.words += 1;
                        }
                    }
                }
                HeaderReadState::Face => {
                    if line.starts_with("property list") {
                        //@todo is this properly defined or are there other identifiers?
                        if line.contains("vertex_indices") || line.contains("vertex_index") {
                            let mut words = to_words(line);
                            skip_n(&mut words, 2); // skip "property" and "list"

                            let t_count = FaceType::try_from(Type::try_from(
                                words.next().ok_or(PlyError::InvalidProperty(*i_line))?,
                            )?)?;
                            let t_index = FaceType::try_from(Type::try_from(
                                words.next().ok_or(PlyError::InvalidProperty(*i_line))?,
                            )?)?;

                            opt_face_count_type = Some(t_count);
                            opt_face_index_type = Some(t_index);
                        }
                    } else {
                        let mut words = to_words(line);
                        skip_n(&mut words, 1); // skip "property"
                        let t = Type::try_from(
                            words.next().ok_or(PlyError::InvalidProperty(*i_line))?,
                        )?;
                        if opt_face_count_type.is_some() {
                            face_after.bytes += t.size_bytes();
                            face_after.words += 1;
                        } else {
                            face_before.bytes += t.size_bytes();
                            face_before.words += 1;
                        }
                    }
                }
                _ => return Err(PlyError::PropertyLineLocation(*i_line)),
            }

            continue;
        }

        if line == "end_header" && ply_found {
            if let (
                Some(format),
                Some(n_vertices),
                Some(n_faces),
                Some(x_type),
                Some(y_type),
                Some(z_type),
                Some(face_count_type),
                Some(face_index_type),
            ) = (
                opt_format,
                opt_n_vertices,
                opt_n_faces,
                opt_fst_type,
                opt_snd_type,
                opt_third_type,
                opt_face_count_type,
                opt_face_index_type,
            ) {
                return Ok(Header {
                    format,
                    n_vertices,
                    n_faces,
                    vertex_format: VertexFormat {
                        order: VertexOrder::try_from(vertex_order)?,
                        first: x_type,
                        snd: y_type,
                        third: z_type,
                        before: vertex_before,
                        between_first_snd: vertex_between_first_snd,
                        between_snd_third: vertex_between_snd_third,
                        after,
                    },
                    face_format: FaceFormat {
                        before: face_before,
                        after: face_after,
                        count: face_count_type,
                        index: face_index_type,
                    },
                });
            }
        }

        return Err(PlyError::LoadHeaderInvalid);
    }

    Err(PlyError::LoadHeaderInvalid)
}

//------------------------------------------------------------------------------

fn load_binary<BR, EM, P, R>(read: &mut R, mesh: &mut EM, header: &Header) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
    BR: ByteReader,
{
    for _ in 0..header.n_vertices {
        skip_bytes(read, header.vertex_format.before.bytes)?;

        let first = read_vertex_type::<BR, _>(read, header.vertex_format.first)?;

        skip_bytes(read, header.vertex_format.between_first_snd.bytes)?;

        let snd = read_vertex_type::<BR, _>(read, header.vertex_format.snd)?;

        skip_bytes(read, header.vertex_format.between_snd_third.bytes)?;

        let third = read_vertex_type::<BR, _>(read, header.vertex_format.third)?;

        skip_bytes(read, header.vertex_format.after.bytes)?;

        mesh.add_vertex(point_with_order(
            first,
            snd,
            third,
            header.vertex_format.order,
        ));
    }

    for _ in 0..header.n_faces {
        skip_bytes(read, header.face_format.before.bytes)?;

        let element_count = read_face_type::<BR, _>(read, header.face_format.count)?;

        if element_count != 3 {
            return Err(PlyError::FaceStructure);
        }

        let a = read_face_type::<BR, _>(read, header.face_format.index)?;
        let b = read_face_type::<BR, _>(read, header.face_format.index)?;
        let c = read_face_type::<BR, _>(read, header.face_format.index)?;

        skip_bytes(read, header.face_format.after.bytes)?;

        mesh.try_add_connection(
            VId { val: a as usize },
            VId { val: b as usize },
            VId { val: c as usize },
        )
        .map_err(|_| PlyError::InvalidMeshIndices(None))?;
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

            let first = f64::from_str(words.next().ok_or(PlyError::InvalidVertex(*i_line))?)
                .map_err(|_| PlyError::InvalidVertex(*i_line))?;

            skip_n(&mut words, header.vertex_format.between_first_snd.words);

            let snd = f64::from_str(words.next().ok_or(PlyError::InvalidVertex(*i_line))?)
                .map_err(|_| PlyError::InvalidVertex(*i_line))?;

            skip_n(&mut words, header.vertex_format.between_snd_third.words);

            let third = f64::from_str(words.next().ok_or(PlyError::InvalidVertex(*i_line))?)
                .map_err(|_| PlyError::InvalidVertex(*i_line))?;

            // no need to skip 'after' since we're done with this line anyway

            mesh.add_vertex(point_with_order(
                first,
                snd,
                third,
                header.vertex_format.order,
            ));

            continue;
        }

        if header.n_faces > mesh.num_faces() {
            let [a, b, c] = collect_index_line(&line).ok_or(PlyError::FaceStructure)?;
            mesh.try_add_connection(VId { val: a }, VId { val: b }, VId { val: c })
                .map_err(|_| PlyError::InvalidMeshIndices(Some(*i_line)))?;
            continue;
        }
    }

    if header.n_vertices != mesh.num_vertices() {
        return Err(PlyError::LoadVertexCountIncorrect);
    }

    Ok(())
}
