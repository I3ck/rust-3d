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

//! Module for iterators used to load .ply files

use crate::*;

use std::{io::BufRead, iter::FusedIterator};

use super::super::types::*;

use super::{header::*, iterators_internal::*, types::*};

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
                    PlyAsciiMeshIterator::new(read, header, i_line),
                ),
                Format::LittleEndian => BinaryOrAsciiPlyMeshInteralIterator::BinaryLittle(
                    PlyBinaryMeshIterator::new(read, header),
                ),
                Format::BigEndian => BinaryOrAsciiPlyMeshInteralIterator::BinaryBig(
                    PlyBinaryMeshIterator::new(read, header),
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
    #[inline(always)]
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
                PlyAsciiPointsIterator::new(read, header, i_line),
            ),
            Format::LittleEndian => BinaryOrAsciiPlyPointsInteralIterator::BinaryLittle(
                PlyBinaryPointsIterator::new(read, header),
            ),
            Format::BigEndian => BinaryOrAsciiPlyPointsInteralIterator::BinaryBig(
                PlyBinaryPointsIterator::new(read, header),
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
    #[inline(always)]
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
