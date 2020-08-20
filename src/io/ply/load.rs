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

use std::io::{BufRead, Read};

use super::super::{byte_reader::*, types::*};

use super::{header::*, iterators::*, iterators_internal::*, types::*};

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
