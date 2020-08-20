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
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    match load_header(read, &mut line_buffer, &mut i_line)? {
        Header::Full(header) => {
            mesh.reserve_vertices(header.vertex.count);
            mesh.reserve_faces(header.face.count);

            match header.format {
                Format::Ascii => load_mesh_ascii(read, mesh, header, &mut i_line),
                Format::LittleEndian => {
                    load_mesh_binary::<LittleReader, _, _, _>(read, mesh, header).simple()
                }
                Format::BigEndian => {
                    load_mesh_binary::<BigReader, _, _, _>(read, mesh, header).simple()
                }
            }?;

            Ok(MeshOrPoints::Mesh)
        }
        Header::Partial(header) => {
            ip.reserve(header.vertex.count);

            match header.format {
                Format::Ascii => load_points_ascii(read, ip, header, i_line),
                Format::LittleEndian => {
                    load_points_binary::<LittleReader, _, _, _>(read, ip, header).simple()
                }
                Format::BigEndian => {
                    load_points_binary::<BigReader, _, _, _>(read, ip, header).simple()
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
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = PlyMeshIterator::new(read)?;

    for fr in iterator {
        match fr? {
            FaceDataReserve::Data(p) => {
                mesh.add_vertex(p);
            }
            FaceDataReserve::ReserveDataFaces(n_d, n_f) => {
                mesh.reserve_vertices(n_d);
                mesh.reserve_faces(n_f);
            }
            FaceDataReserve::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .or(Err(PlyError::InvalidMeshIndices))
                    .simple()?;
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Loads the points from the .ply file into IsPushable<Is3D>
pub fn load_ply_points<IP, P, R>(read: &mut R, ip: &mut IP) -> PlyIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = PlyPointsIterator::new(read)?;

    for rp in iterator {
        match rp? {
            DataReserve::Reserve(x) => ip.reserve(x),
            DataReserve::Data(x) => ip.push(x),
        }
    }

    Ok(())
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

struct PlyMeshAsciiIteratorInternal<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    header: FullHeader,
    p_iter: Option<PlyAsciiPointsInternalIterator<P, R>>,
    f_iter: Option<PlyAsciiFacesInternalIterator<R>>,
}

impl<P, R> PlyMeshAsciiIteratorInternal<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R, header: FullHeader, i_line: usize) -> Self {
        let partial_header: PartialHeader = header.clone().into();
        Self {
            header,
            p_iter: Some(PlyAsciiPointsInternalIterator::new(
                read,
                partial_header,
                i_line,
            )),
            f_iter: None,
        }
    }
}

impl<P, R> Iterator for PlyMeshAsciiIteratorInternal<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PlyIOResult<io::types::FaceData<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut p_iter) = self.p_iter {
            match p_iter.next() {
                Some(x) => return Some(x.map(|x| io::types::FaceData::Data(x))),
                None => {
                    // point iteration done, switch to face iteration
                    // unwrap safe, since in if let Some()
                    let p_iter = self.p_iter.take().unwrap();
                    let (read, i_line) = p_iter.destruct();
                    self.f_iter = Some(PlyAsciiFacesInternalIterator::new(
                        read,
                        self.header.clone(),
                        i_line,
                    ));
                }
            }
        }
        //unwrap safe, either is always constructed
        self.f_iter
            .as_mut()
            .unwrap()
            .next()
            .map(|x| x.map(|x| io::types::FaceData::Face(x)))
    }
}

impl<P, R> FusedIterator for PlyMeshAsciiIteratorInternal<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

struct PlyMeshBinaryIteratorInternal<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    header: FullHeader,
    p_iter: Option<PlyBinaryPointsInternalIterator<BR, P, R>>,
    f_iter: Option<PlyBinaryFacesInternalIterator<BR, R>>,
}

impl<BR, P, R> PlyMeshBinaryIteratorInternal<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: FullHeader) -> Self {
        let partial_header: PartialHeader = header.clone().into();
        Self {
            header,
            p_iter: Some(PlyBinaryPointsInternalIterator::new(read, partial_header)),
            f_iter: None,
        }
    }
}

impl<BR, P, R> Iterator for PlyMeshBinaryIteratorInternal<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    type Item = PlyResult<io::types::FaceData<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut p_iter) = self.p_iter {
            match p_iter.next() {
                Some(x) => return Some(x.map(|x| io::types::FaceData::Data(x))),
                None => {
                    // point iteration done, switch to face iteration
                    // unwrap safe, since in if let Some()
                    let p_iter = self.p_iter.take().unwrap();
                    let read = p_iter.destruct();
                    self.f_iter = Some(PlyBinaryFacesInternalIterator::new(
                        read,
                        self.header.clone(),
                    ));
                }
            }
        }
        //unwrap safe, either is always constructed
        self.f_iter
            .as_mut()
            .unwrap()
            .next()
            .map(|x| x.map(|x| io::types::FaceData::Face(x)))
    }
}

impl<BR, P, R> FusedIterator for PlyMeshBinaryIteratorInternal<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load a mesh from a .ply file
pub struct PlyMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    inner: BinaryOrAsciiPlyMeshInteralIterator<P, R>,
    to_reserve_data_faces: Option<(usize, usize)>,
}

impl<P, R> PlyMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(mut read: R) -> PlyIOResult<Self> {
        let mut line_buffer = Vec::new();
        let mut i_line = 0;

        if let Header::Full(header) = load_header(&mut read, &mut line_buffer, &mut i_line)? {
            let to_reserve_data_faces = Some((header.vertex.count, header.face.count));

            let inner = match header.format {
                Format::Ascii => BinaryOrAsciiPlyMeshInteralIterator::Ascii(
                    PlyMeshAsciiIteratorInternal::new(read, header, i_line),
                ),
                Format::LittleEndian => BinaryOrAsciiPlyMeshInteralIterator::BinaryLittle(
                    PlyMeshBinaryIteratorInternal::new(read, header),
                ),
                Format::BigEndian => BinaryOrAsciiPlyMeshInteralIterator::BinaryBig(
                    PlyMeshBinaryIteratorInternal::new(read, header),
                ),
            };

            Ok(Self {
                inner,
                to_reserve_data_faces,
            })
        } else {
            Err(PlyError::LoadHeaderInvalid).simple()
        }
    }
}

impl<P, R> Iterator for PlyMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PlyIOResult<FaceDataReserve<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(to_reserve_data_faces) = self.to_reserve_data_faces {
            self.to_reserve_data_faces = None;
            Some(Ok(FaceDataReserve::ReserveDataFaces(
                to_reserve_data_faces.0,
                to_reserve_data_faces.1,
            )))
        } else {
            match &mut self.inner {
                BinaryOrAsciiPlyMeshInteralIterator::Ascii(x) => {
                    x.next().map(|x| x.map(|x| x.into()))
                }
                BinaryOrAsciiPlyMeshInteralIterator::BinaryLittle(x) => {
                    x.next().map(|x| x.map(|x| x.into()).simple())
                }
                BinaryOrAsciiPlyMeshInteralIterator::BinaryBig(x) => {
                    x.next().map(|x| x.map(|x| x.into()).simple())
                }
            }
        }
    }
}

impl<P, R> FusedIterator for PlyMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load points from a .ply file
pub struct PlyPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    inner: BinaryOrAsciiPlyPointsInteralIterator<P, R>,
    to_reserve: Option<usize>,
}

impl<P, R> PlyPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(mut read: R) -> PlyIOResult<Self> {
        let mut line_buffer = Vec::new();
        let mut i_line = 0;

        let header: PartialHeader = load_header(&mut read, &mut line_buffer, &mut i_line)?.into();
        let to_reserve = Some(header.vertex.count);

        let inner = match header.format {
            Format::Ascii => BinaryOrAsciiPlyPointsInteralIterator::Ascii(
                PlyAsciiPointsInternalIterator::new(read, header, i_line),
            ),
            Format::LittleEndian => BinaryOrAsciiPlyPointsInteralIterator::BinaryLittle(
                PlyBinaryPointsInternalIterator::new(read, header),
            ),
            Format::BigEndian => BinaryOrAsciiPlyPointsInteralIterator::BinaryBig(
                PlyBinaryPointsInternalIterator::new(read, header),
            ),
        };

        Ok(Self { inner, to_reserve })
    }
}

impl<P, R> Iterator for PlyPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PlyIOResult<DataReserve<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(to_reserve) = self.to_reserve {
            self.to_reserve = None;
            Some(Ok(DataReserve::Reserve(to_reserve)))
        } else {
            match &mut self.inner {
                BinaryOrAsciiPlyPointsInteralIterator::Ascii(x) => x.next(),
                BinaryOrAsciiPlyPointsInteralIterator::BinaryLittle(x) => {
                    x.next().map(|x| x.simple())
                }
                BinaryOrAsciiPlyPointsInteralIterator::BinaryBig(x) => x.next().map(|x| x.simple()),
            }
            .map(|x| x.map(|x| DataReserve::Data(x)))
        }
    }
}

impl<P, R> FusedIterator for PlyPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

enum BinaryOrAsciiPlyPointsInteralIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    Ascii(PlyAsciiPointsInternalIterator<P, R>),
    BinaryLittle(PlyBinaryPointsInternalIterator<LittleReader, P, R>),
    BinaryBig(PlyBinaryPointsInternalIterator<BigReader, P, R>),
}

//------------------------------------------------------------------------------

enum BinaryOrAsciiPlyMeshInteralIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    //@todo naming convention between the iterators is mixed (AsciiPoints vs MeshAscii)
    Ascii(PlyMeshAsciiIteratorInternal<P, R>),
    BinaryLittle(PlyMeshBinaryIteratorInternal<LittleReader, P, R>),
    BinaryBig(PlyMeshBinaryIteratorInternal<BigReader, P, R>),
}

//------------------------------------------------------------------------------

struct PlyBinaryPointsInternalIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    read: R,
    header: PartialHeader,
    current: usize,
    phantom_p: PhantomData<P>,
    phantom_br: PhantomData<BR>,
}

impl<BR, P, R> PlyBinaryPointsInternalIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: PartialHeader) -> Self {
        Self {
            read,
            header,
            current: 0,
            phantom_p: PhantomData,
            phantom_br: PhantomData,
        }
    }

    pub fn destruct(self) -> R {
        self.read
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

impl<BR, P, R> Iterator for PlyBinaryPointsInternalIterator<BR, P, R>
where
    P: IsBuildable3D,
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

impl<BR, P, R> FusedIterator for PlyBinaryPointsInternalIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

struct PlyAsciiPointsInternalIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    header: PartialHeader,
    current: usize,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom: PhantomData<P>,
}

impl<P, R> PlyAsciiPointsInternalIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R, header: PartialHeader, i_line: usize) -> Self {
        Self {
            read,
            header,
            current: 0,
            i_line,
            line_buffer: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn destruct(self) -> (R, usize) {
        (self.read, self.i_line)
    }

    #[inline(always)]
    fn fetch_one(header: &PartialHeader, line: &[u8], i_line: usize) -> PlyIOResult<P> {
        let mut words = to_words_skip_empty(line);

        skip_n(&mut words, header.vertex.format.before.words);

        let first = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(PlyError::InvalidVertex)
            .line(i_line, line)?;

        skip_n(&mut words, header.vertex.format.between_first_snd.words);

        let snd = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(PlyError::InvalidVertex)
            .line(i_line, line)?;

        skip_n(&mut words, header.vertex.format.between_snd_third.words);

        let third = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(PlyError::InvalidVertex)
            .line(i_line, line)?;

        // no need to skip 'after' since we're done with this line anyway

        Ok(point_with_order(
            first,
            snd,
            third,
            header.vertex.format.order,
        ))
    }
}

impl<P, R> Iterator for PlyAsciiPointsInternalIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PlyIOResult<P>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.header.vertex.count {
            self.current += 1;
            while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                self.i_line += 1;
                return Some(Self::fetch_one(&self.header, line, self.i_line));
            }
        }

        if self.current != self.header.vertex.count {
            Some(Err(PlyError::LoadVertexCountIncorrect).simple())
        } else {
            None
        }
    }
}

impl<P, R> FusedIterator for PlyAsciiPointsInternalIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

struct PlyAsciiFacesInternalIterator<R>
where
    R: BufRead,
{
    read: R,
    header: FullHeader,
    current: usize,
    i_line: usize,
    line_buffer: Vec<u8>,
}

impl<R> PlyAsciiFacesInternalIterator<R>
where
    R: BufRead,
{
    pub fn new(read: R, header: FullHeader, i_line: usize) -> Self {
        Self {
            read,
            header,
            current: 0,
            i_line,
            line_buffer: Vec::new(),
        }
    }
}

impl<R> Iterator for PlyAsciiFacesInternalIterator<R>
where
    R: BufRead,
{
    type Item = PlyIOResult<[usize; 3]>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.header.face.count {
            self.current += 1;
            while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                self.i_line += 1;
                return Some(
                    collect_index_line(&line)
                        .ok_or(PlyError::FaceStructure)
                        .line(self.i_line, line),
                );
            }
        }

        None
    }
}

impl<R> FusedIterator for PlyAsciiFacesInternalIterator<R> where R: BufRead {}

//------------------------------------------------------------------------------

struct PlyBinaryFacesInternalIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
    read: R,
    header: FullHeader,
    current: usize,
    phantom: PhantomData<BR>,
}

impl<BR, R> PlyBinaryFacesInternalIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: FullHeader) -> Self {
        Self {
            read,
            header,
            current: 0,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_one(&mut self) -> PlyResult<[usize; 3]> {
        skip_bytes(&mut self.read, self.header.face.format.before.bytes)?;

        let element_count = read_face_type::<BR, _>(&mut self.read, self.header.face.format.count)?;

        if element_count != 3 {
            return Err(PlyError::FaceStructure);
        }

        let a = read_face_type::<BR, _>(&mut self.read, self.header.face.format.index)?;
        let b = read_face_type::<BR, _>(&mut self.read, self.header.face.format.index)?;
        let c = read_face_type::<BR, _>(&mut self.read, self.header.face.format.index)?;

        skip_bytes(&mut self.read, self.header.face.format.after.bytes)?;

        Ok([a, b, c])
    }
}

impl<BR, R> Iterator for PlyBinaryFacesInternalIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
    type Item = PlyResult<[usize; 3]>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.header.face.count {
            self.current += 1;
            Some(self.fetch_one())
        } else {
            None
        }
    }
}

impl<BR, R> FusedIterator for PlyBinaryFacesInternalIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

fn load_points_binary<BR, IP, P, R>(
    read: &mut R,
    ip: &mut IP,
    header: PartialHeader,
) -> PlyResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    let iterator = PlyBinaryPointsInternalIterator::<BR, _, _>::new(read, header);

    for p in iterator {
        ip.push(p?)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_points_ascii<IP, P, R>(
    read: &mut R,
    ip: &mut IP,
    header: PartialHeader,
    i_line: usize,
) -> PlyIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = PlyAsciiPointsInternalIterator::new(read, header, i_line);

    for p in iterator {
        ip.push(p?)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_mesh_binary<BR, EM, P, R>(read: &mut R, mesh: &mut EM, header: FullHeader) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    let iterator = PlyMeshBinaryIteratorInternal::<BR, _, _>::new(read, header);

    for fd in iterator {
        match fd? {
            io::types::FaceData::Data(p) => {
                mesh.add_vertex(p);
            }
            io::types::FaceData::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .or(Err(PlyError::InvalidMeshIndices))?;
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_mesh_ascii<EM, P, R>(
    read: &mut R,
    mesh: &mut EM,
    header: FullHeader,
    i_line: &mut usize,
) -> PlyIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = PlyMeshAsciiIteratorInternal::new(read, header, *i_line);

    for fd in iterator {
        match fd? {
            io::types::FaceData::Data(p) => {
                mesh.add_vertex(p);
            }
            io::types::FaceData::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .or(Err(PlyError::InvalidMeshIndices))
                    .simple()?;
            }
        }
    }

    Ok(())
}
