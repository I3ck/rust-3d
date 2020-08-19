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

use core::convert::TryFrom;

use std::{
    io::{BufRead, Read},
    iter::FusedIterator,
    marker::PhantomData,
};

use super::super::{byte_reader::*, types::*, utils::*};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the .ply file format if possible, otherwise tries loading point data. Returning which of the two was possible
pub fn load_ply_either<EM, IP, P, R>(
    read: &mut R,
    mesh: &mut EM,
    ip: &mut IP,
) -> PlyIOResult<MeshOrPoints>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    IP: IsPushable<P>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    match load_header(read, &mut line_buffer, &mut i_line)? {
        Header::Full(header) => {
            mesh.reserve_vertices(header.vertex.count);
            mesh.reserve_faces(header.face.count);

            match header.format {
                Format::Ascii => {
                    load_mesh_ascii(read, mesh, &header, &mut line_buffer, &mut i_line)
                }
                Format::LittleEndian => {
                    load_mesh_binary::<LittleReader, _, _, _>(read, mesh, &header).simple()
                }
                Format::BigEndian => {
                    load_mesh_binary::<BigReader, _, _, _>(read, mesh, &header).simple()
                }
            }?;

            Ok(MeshOrPoints::Mesh)
        }
        Header::Partial(header) => {
            ip.reserve(header.vertex.count);

            match header.format {
                Format::Ascii => {
                    load_points_ascii(read, ip, &header, &mut line_buffer, &mut i_line)
                }
                Format::LittleEndian => {
                    load_points_binary::<LittleReader, _, _, _>(read, ip, &header).simple()
                }
                Format::BigEndian => {
                    load_points_binary::<BigReader, _, _, _>(read, ip, &header).simple()
                }
            }?;

            Ok(MeshOrPoints::Points)
        }
    }
}

/// Loads an IsMesh3D from the .ply file format
pub fn load_ply_mesh<EM, P, R>(read: &mut R, mesh: &mut EM) -> PlyIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    if let Header::Full(header) = load_header(read, &mut line_buffer, &mut i_line)? {
        mesh.reserve_vertices(header.vertex.count);
        mesh.reserve_faces(header.face.count);

        match header.format {
            Format::Ascii => load_mesh_ascii(read, mesh, &header, &mut line_buffer, &mut i_line),
            Format::LittleEndian => {
                load_mesh_binary::<LittleReader, _, _, _>(read, mesh, &header).simple()
            }
            Format::BigEndian => {
                load_mesh_binary::<BigReader, _, _, _>(read, mesh, &header).simple()
            }
        }
    } else {
        Err(PlyError::LoadHeaderInvalid).simple()
    }
}

//------------------------------------------------------------------------------

/// Loads the points from the .ply file into IsPushable<Is3D>
pub fn load_ply_points<IP, P, R>(read: &mut R, ip: &mut IP) -> PlyIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    let header: PartialHeader = load_header(read, &mut line_buffer, &mut i_line)?.into();
    ip.reserve(header.vertex.count);

    match header.format {
        Format::Ascii => load_points_ascii(read, ip, &header, &mut line_buffer, &mut i_line),
        Format::LittleEndian => {
            load_points_binary::<LittleReader, _, _, _>(read, ip, &header).simple()
        }
        Format::BigEndian => load_points_binary::<BigReader, _, _, _>(read, ip, &header).simple(),
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

fn load_header<R>(
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

struct PlyBinaryPointsIterator<'a, BR, P, R>
where
    P: IsBuildable3D + Clone, //@todo Clone required?
    R: Read,
    BR: IsByteReader,
{
    read: R,
    header: &'a PartialHeader,
    current: usize,
    phantom_p: PhantomData<P>,
    phantom_br: PhantomData<BR>,
}

impl<'a, BR, P, R> PlyBinaryPointsIterator<'a, BR, P, R>
where
    P: IsBuildable3D + Clone, //@todo Clone required?
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: &'a PartialHeader) -> Self {
        Self {
            read,
            header,
            current: 0,
            phantom_p: PhantomData,
            phantom_br: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_one(&mut self) -> PlyResult<P> {
        skip_bytes(&mut self.read, self.header.vertex.format.before.bytes)?;

        let first = read_vertex_type::<BR, _>(&mut self.read, self.header.vertex.format.first)?;

        skip_bytes(
            &mut self.read,
            self.header.vertex.format.between_first_snd.bytes,
        )?;

        let snd = read_vertex_type::<BR, _>(&mut self.read, self.header.vertex.format.snd)?;

        skip_bytes(
            &mut self.read,
            self.header.vertex.format.between_snd_third.bytes,
        )?;

        let third = read_vertex_type::<BR, _>(&mut self.read, self.header.vertex.format.third)?;

        skip_bytes(&mut self.read, self.header.vertex.format.after.bytes)?;

        Ok(point_with_order(
            first,
            snd,
            third,
            self.header.vertex.format.order,
        ))
    }
}

impl<'a, BR, P, R> Iterator for PlyBinaryPointsIterator<'a, BR, P, R>
where
    P: IsBuildable3D + Clone, //@todo Clone required?
    R: Read,
    BR: IsByteReader,
{
    type Item = PlyResult<P>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.header.vertex.count {
            self.current += 1;
            Some(self.fetch_one())
        } else {
            None
        }
    }
}

impl<'a, BR, P, R> FusedIterator for PlyBinaryPointsIterator<'a, BR, P, R>
where
    P: IsBuildable3D + Clone, //@todo Clone required?
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

fn load_points_binary<BR, IP, P, R>(
    read: &mut R,
    ip: &mut IP,
    header: &PartialHeader,
) -> PlyResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + Clone, //@todo Clone required?
    R: Read,
    BR: IsByteReader,
{
    let iterator = PlyBinaryPointsIterator::<BR, _, _>::new(read, header);

    for p in iterator {
        ip.push(p?)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_points_ascii<IP, P, R>(
    read: &mut R,
    ip: &mut IP,
    header: &PartialHeader,
    line_buffer: &mut Vec<u8>,
    i_line: &mut usize,
) -> PlyIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut n_pushed = 0;

    while let Ok(line) = fetch_line(read, line_buffer) {
        *i_line += 1;

        if header.vertex.count > n_pushed {
            let mut words = to_words_skip_empty(line);

            skip_n(&mut words, header.vertex.format.before.words);

            let first = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(PlyError::InvalidVertex)
                .line(*i_line, line)?;

            skip_n(&mut words, header.vertex.format.between_first_snd.words);

            let snd = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(PlyError::InvalidVertex)
                .line(*i_line, line)?;

            skip_n(&mut words, header.vertex.format.between_snd_third.words);

            let third = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(PlyError::InvalidVertex)
                .line(*i_line, line)?;

            // no need to skip 'after' since we're done with this line anyway

            ip.push(point_with_order(
                first,
                snd,
                third,
                header.vertex.format.order,
            ));

            n_pushed += 1;

            continue;
        }
    }

    if header.vertex.count != n_pushed {
        return Err(PlyError::LoadVertexCountIncorrect).simple();
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_mesh_binary<BR, EM, P, R>(read: &mut R, mesh: &mut EM, header: &FullHeader) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
    BR: IsByteReader,
{
    for _ in 0..header.vertex.count {
        skip_bytes(read, header.vertex.format.before.bytes)?;

        let first = read_vertex_type::<BR, _>(read, header.vertex.format.first)?;

        skip_bytes(read, header.vertex.format.between_first_snd.bytes)?;

        let snd = read_vertex_type::<BR, _>(read, header.vertex.format.snd)?;

        skip_bytes(read, header.vertex.format.between_snd_third.bytes)?;

        let third = read_vertex_type::<BR, _>(read, header.vertex.format.third)?;

        skip_bytes(read, header.vertex.format.after.bytes)?;

        mesh.add_vertex(point_with_order(
            first,
            snd,
            third,
            header.vertex.format.order,
        ));
    }

    for _ in 0..header.face.count {
        skip_bytes(read, header.face.format.before.bytes)?;

        let element_count = read_face_type::<BR, _>(read, header.face.format.count)?;

        if element_count != 3 {
            return Err(PlyError::FaceStructure);
        }

        let a = read_face_type::<BR, _>(read, header.face.format.index)?;
        let b = read_face_type::<BR, _>(read, header.face.format.index)?;
        let c = read_face_type::<BR, _>(read, header.face.format.index)?;

        skip_bytes(read, header.face.format.after.bytes)?;

        mesh.try_add_connection(VId(a as usize), VId(b as usize), VId(c as usize))
            .or(Err(PlyError::InvalidMeshIndices))?;
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_mesh_ascii<EM, P, R>(
    read: &mut R,
    mesh: &mut EM,
    header: &FullHeader,
    line_buffer: &mut Vec<u8>,
    i_line: &mut usize,
) -> PlyIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    while let Ok(line) = fetch_line(read, line_buffer) {
        *i_line += 1;

        if header.vertex.count > mesh.num_vertices() {
            let mut words = to_words_skip_empty(line);

            skip_n(&mut words, header.vertex.format.before.words);

            let first = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(PlyError::InvalidVertex)
                .line(*i_line, line)?;

            skip_n(&mut words, header.vertex.format.between_first_snd.words);

            let snd = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(PlyError::InvalidVertex)
                .line(*i_line, line)?;

            skip_n(&mut words, header.vertex.format.between_snd_third.words);

            let third = words
                .next()
                .and_then(|w| from_ascii(w))
                .ok_or(PlyError::InvalidVertex)
                .line(*i_line, line)?;

            // no need to skip 'after' since we're done with this line anyway

            mesh.add_vertex(point_with_order(
                first,
                snd,
                third,
                header.vertex.format.order,
            ));

            continue;
        }

        if header.face.count > mesh.num_faces() {
            let [a, b, c] = collect_index_line(&line)
                .ok_or(PlyError::FaceStructure)
                .line(*i_line, line)?;
            mesh.try_add_connection(VId(a), VId(b), VId(c))
                .or(Err(PlyError::InvalidMeshIndices))
                .line(*i_line, line)?;
            continue;
        }
    }

    if header.vertex.count != mesh.num_vertices() {
        return Err(PlyError::LoadVertexCountIncorrect).simple();
    }

    Ok(())
}
