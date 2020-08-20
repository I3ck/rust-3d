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

use std::{
    io::{BufRead, Read},
    iter::FusedIterator,
};

use super::super::{byte_reader::*, types::*};

use super::{header::*, iterators_internal::*, types::*};

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
