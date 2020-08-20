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

//! Module containing functions for the .ply header

use crate::*;

use std::{convert::TryFrom, io::BufRead};

use super::super::{types::*, utils::*};

use super::types::*;

//------------------------------------------------------------------------------

/// Loading a .ply header
pub fn load_header<R>(
    read: &mut R,
    line_buffer: &mut Vec<u8>,
    i_line: &mut usize,
) -> PlyIOResult<Header>
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

    while let Ok(line) = fetch_line(read, line_buffer) {
        *i_line += 1;

        if line.starts_with(b"comment") {
            continue;
        }

        if line.starts_with(b"obj_info") {
            continue;
        }

        if !ply_found {
            if line == b"ply" {
                ply_found = true;
                continue;
            }
            return Err(PlyError::LoadStartNotFound).line(*i_line, line);
        }

        if opt_format.is_none() {
            opt_format = Some(match line {
                b"format ascii 1.0" => Format::Ascii,
                b"format binary_little_endian 1.0" => Format::LittleEndian,
                b"format binary_big_endian 1.0" => Format::BigEndian,
                _ => return Err(PlyError::LoadFormatNotFound).line(*i_line, line),
            });
            continue;
        }

        match opt_n_vertices {
            None => {
                if line.starts_with(b"element vertex") {
                    read_state = HeaderReadState::Vertex;
                    let mut words = to_words_skip_empty(line);
                    opt_n_vertices = Some(
                        words
                            .nth(2)
                            .and_then(|w| from_ascii(w))
                            .ok_or(PlyError::VertexElement)
                            .line(*i_line, line)?,
                    );
                    continue;
                }
            }
            Some(_) => {}
        }

        match opt_n_faces {
            None => {
                if line.starts_with(b"element face") {
                    read_state = HeaderReadState::Face;
                    let mut words = to_words_skip_empty(line);
                    opt_n_faces = Some(
                        words
                            .nth(2)
                            .and_then(|w| from_ascii(w))
                            .ok_or(PlyError::FaceElement)
                            .line(*i_line, line)?,
                    );
                    continue;
                }
            }
            Some(_) => {}
        }

        if line.starts_with(b"property") {
            match read_state {
                HeaderReadState::Vertex => {
                    let mut words = to_words_skip_empty(line);
                    skip_n(&mut words, 1); // skip "property"

                    let t = words
                        .next()
                        .ok_or(PlyError::InvalidProperty)
                        .and_then(|w| Type::try_from(w))
                        .line(*i_line, line)?;
                    let id = words
                        .next()
                        .ok_or(PlyError::InvalidProperty)
                        .line(*i_line, line)?;
                    if id == b"x" {
                        opt_fst_type = Some(VertexType::try_from(t).line(*i_line, line)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::X;
                        i_vertex_order += 1;
                    } else if id == b"y" {
                        opt_snd_type = Some(VertexType::try_from(t).line(*i_line, line)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::Y;
                        i_vertex_order += 1;
                    } else if id == b"z" {
                        opt_third_type = Some(VertexType::try_from(t).line(*i_line, line)?);
                        n_types_found += 1;
                        vertex_order[i_vertex_order] = Xyz::Z;
                        i_vertex_order += 1;
                    } else if n_types_found == 0 {
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
                HeaderReadState::Face => {
                    if line.starts_with(b"property list") {
                        if contains(line, b"vertex_indices") || contains(line, b"vertex_index") {
                            let mut words = to_words_skip_empty(line);
                            skip_n(&mut words, 2); // skip "property" and "list"

                            let t_count = words
                                .next()
                                .ok_or(PlyError::InvalidProperty)
                                .and_then(|x| Type::try_from(x))
                                .and_then(|x| FaceType::try_from(x))
                                .line(*i_line, line)?;
                            let t_index = words
                                .next()
                                .ok_or(PlyError::InvalidProperty)
                                .and_then(|x| Type::try_from(x))
                                .and_then(|x| FaceType::try_from(x))
                                .line(*i_line, line)?;

                            opt_face_count_type = Some(t_count);
                            opt_face_index_type = Some(t_index);
                        }
                    } else {
                        let mut words = to_words_skip_empty(line);
                        skip_n(&mut words, 1); // skip "property"
                        let t = words
                            .next()
                            .ok_or(PlyError::InvalidProperty)
                            .and_then(|x| Type::try_from(x))
                            .line(*i_line, line)?;
                        if opt_face_count_type.is_some() {
                            face_after.bytes += t.size_bytes();
                            face_after.words += 1;
                        } else {
                            face_before.bytes += t.size_bytes();
                            face_before.words += 1;
                        }
                    }
                }
                _ => {
                    return Err(PlyError::PropertyLineLocation).line(*i_line, line);
                }
            }

            continue;
        }

        if line == b"end_header" && ply_found {
            if let (Some(format), Some(n_vertices), Some(x_type), Some(y_type), Some(z_type)) = (
                opt_format,
                opt_n_vertices,
                opt_fst_type,
                opt_snd_type,
                opt_third_type,
            ) {
                let vertex_data = VertexData {
                    count: n_vertices,
                    format: VertexFormat {
                        order: VertexOrder::try_from(vertex_order).line(*i_line, line)?,
                        first: x_type,
                        snd: y_type,
                        third: z_type,
                        before: vertex_before,
                        between_first_snd: vertex_between_first_snd,
                        between_snd_third: vertex_between_snd_third,
                        after,
                    },
                };

                if let (Some(n_faces), Some(face_count_type), Some(face_index_type)) =
                    (opt_n_faces, opt_face_count_type, opt_face_index_type)
                {
                    return Ok(Header::Full(FullHeader {
                        format,
                        vertex: vertex_data,
                        face: super::types::FaceData {
                            count: n_faces,
                            format: FaceFormat {
                                before: face_before,
                                after: face_after,
                                count: face_count_type,
                                index: face_index_type,
                            },
                        },
                    }));
                } else {
                    return Ok(Header::Partial(PartialHeader {
                        format,
                        vertex: vertex_data,
                    }));
                }
            }
        }

        return Err(PlyError::LoadHeaderInvalid).line(*i_line, line);
    }

    Err(PlyError::LoadHeaderInvalid).simple()
}

//------------------------------------------------------------------------------
