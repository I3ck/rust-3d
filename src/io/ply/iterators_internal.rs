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

//! Module for internal iterators used to load .ply files

use crate::*;

use std::{
    io::{BufRead, Read},
    iter::FusedIterator,
    marker::PhantomData,
};

use super::super::{byte_reader::*, types::*, utils::*};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

pub struct PlyAsciiMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    header: FullHeader,
    p_iter: Option<PlyAsciiPointsIterator<P, R>>,
    f_iter: Option<PlyAsciiFacesIterator<R>>,
}

impl<P, R> PlyAsciiMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R, header: FullHeader, i_line: usize) -> Self {
        let partial_header: PartialHeader = header.clone().into();
        Self {
            header,
            p_iter: Some(PlyAsciiPointsIterator::new(read, partial_header, i_line)),
            f_iter: None,
        }
    }
}

impl<P, R> Iterator for PlyAsciiMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PlyIOResult<io::types::FaceData<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut p_iter) = self.p_iter {
            match p_iter.next() {
                Some(x) => return Some(x.map(|x| io::types::FaceData::Data(x))),
                None => {
                    // point iteration done, switch to face iteration
                    // unwrap safe, since in if let Some()
                    let p_iter = self.p_iter.take().unwrap();
                    let (read, i_line) = p_iter.destruct();
                    self.f_iter = Some(PlyAsciiFacesIterator::new(
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

impl<P, R> FusedIterator for PlyAsciiMeshIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

pub struct PlyBinaryMeshIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    header: FullHeader,
    p_iter: Option<PlyBinaryPointsIterator<BR, P, R>>,
    f_iter: Option<PlyBinaryFacesIterator<BR, R>>,
}

impl<BR, P, R> PlyBinaryMeshIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: FullHeader) -> Self {
        let partial_header: PartialHeader = header.clone().into();
        Self {
            header,
            p_iter: Some(PlyBinaryPointsIterator::new(read, partial_header)),
            f_iter: None,
        }
    }
}

impl<BR, P, R> Iterator for PlyBinaryMeshIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    type Item = PlyResult<io::types::FaceData<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut p_iter) = self.p_iter {
            match p_iter.next() {
                Some(x) => return Some(x.map(|x| io::types::FaceData::Data(x))),
                None => {
                    // point iteration done, switch to face iteration
                    // unwrap safe, since in if let Some()
                    let p_iter = self.p_iter.take().unwrap();
                    let read = p_iter.destruct();
                    self.f_iter = Some(PlyBinaryFacesIterator::new(read, self.header.clone()));
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

impl<BR, P, R> FusedIterator for PlyBinaryMeshIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

pub enum BinaryOrAsciiPlyPointsInteralIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    Ascii(PlyAsciiPointsIterator<P, R>),
    BinaryLittle(PlyBinaryPointsIterator<LittleReader, P, R>),
    BinaryBig(PlyBinaryPointsIterator<BigReader, P, R>),
}

//------------------------------------------------------------------------------

pub enum BinaryOrAsciiPlyMeshInteralIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    Ascii(PlyAsciiMeshIterator<P, R>),
    BinaryLittle(PlyBinaryMeshIterator<LittleReader, P, R>),
    BinaryBig(PlyBinaryMeshIterator<BigReader, P, R>),
}

//------------------------------------------------------------------------------

pub struct PlyBinaryPointsIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    read: R,
    is_done: bool,
    header: PartialHeader,
    current: usize,
    phantom_p: PhantomData<P>,
    phantom_br: PhantomData<BR>,
}

impl<BR, P, R> PlyBinaryPointsIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: PartialHeader) -> Self {
        Self {
            read,
            is_done: false,
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

impl<BR, P, R> Iterator for PlyBinaryPointsIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
    type Item = PlyResult<P>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        if self.current < self.header.vertex.count {
            self.current += 1;
            Some(self.fetch_one().map_err(|e| {
                self.is_done = true;
                e
            }))
        } else {
            self.is_done = true;
            None
        }
    }
}

impl<BR, P, R> FusedIterator for PlyBinaryPointsIterator<BR, P, R>
where
    P: IsBuildable3D,
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

pub struct PlyAsciiPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    is_done: bool,
    header: PartialHeader,
    current: usize,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom: PhantomData<P>,
}

impl<P, R> PlyAsciiPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R, header: PartialHeader, i_line: usize) -> Self {
        Self {
            read,
            is_done: false,
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

impl<P, R> Iterator for PlyAsciiPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PlyIOResult<P>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        if self.current < self.header.vertex.count {
            self.current += 1;
            while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                self.i_line += 1;
                return Some(
                    Self::fetch_one(&self.header, line, self.i_line).map_err(|e| {
                        self.is_done = true;
                        e
                    }),
                );
            }
        }

        self.is_done = true;

        if self.current != self.header.vertex.count {
            Some(Err(PlyError::LoadVertexCountIncorrect).simple())
        } else {
            None
        }
    }
}

impl<P, R> FusedIterator for PlyAsciiPointsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

pub struct PlyAsciiFacesIterator<R>
where
    R: BufRead,
{
    read: R,
    is_done: bool,
    header: FullHeader,
    current: usize,
    i_line: usize,
    line_buffer: Vec<u8>,
}

impl<R> PlyAsciiFacesIterator<R>
where
    R: BufRead,
{
    pub fn new(read: R, header: FullHeader, i_line: usize) -> Self {
        Self {
            read,
            is_done: false,
            header,
            current: 0,
            i_line,
            line_buffer: Vec::new(),
        }
    }
}

impl<R> Iterator for PlyAsciiFacesIterator<R>
where
    R: BufRead,
{
    type Item = PlyIOResult<[usize; 3]>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        if self.current < self.header.face.count {
            self.current += 1;
            while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                self.i_line += 1;
                return Some(
                    collect_index_line(&line)
                        .ok_or(PlyError::FaceStructure)
                        .line(self.i_line, line)
                        .map_err(|e| {
                            self.is_done = true;
                            e
                        }),
                );
            }
        }

        self.is_done = true;

        None
    }
}

impl<R> FusedIterator for PlyAsciiFacesIterator<R> where R: BufRead {}

//------------------------------------------------------------------------------

pub struct PlyBinaryFacesIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
    read: R,
    is_done: bool,
    header: FullHeader,
    current: usize,
    phantom: PhantomData<BR>,
}

impl<BR, R> PlyBinaryFacesIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
    pub fn new(read: R, header: FullHeader) -> Self {
        Self {
            read,
            is_done: false,
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

impl<BR, R> Iterator for PlyBinaryFacesIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
    type Item = PlyResult<[usize; 3]>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        if self.current < self.header.face.count {
            self.current += 1;
            Some(self.fetch_one().map_err(|e| {
                self.is_done = true;
                e
            }))
        } else {
            self.is_done = true;
            None
        }
    }
}

impl<BR, R> FusedIterator for PlyBinaryFacesIterator<BR, R>
where
    R: Read,
    BR: IsByteReader,
{
}
