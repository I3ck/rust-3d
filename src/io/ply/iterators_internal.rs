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

pub struct PlyAsciiMeshIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    header: FullHeader,
    p_iter: Option<PlyAsciiPointsIterator<P, R, CHUNK_SIZE>>,
    f_iter: Option<PlyAsciiFacesIterator<R, P, CHUNK_SIZE>>, //@todo order type args
}

impl<P, R, const CHUNK_SIZE: usize> PlyAsciiMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
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

impl<P, R, const CHUNK_SIZE: usize> Iterator for PlyAsciiMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    type Item = IOResult<StackVec<FaceDataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut p_iter) = self.p_iter {
            match p_iter.next() {
                Some(x) => return Some(x.map(|x| x.convert())),
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
            .map(|x| x.map(|x| x.convert()))
    }
}

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for PlyAsciiMeshIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

pub struct PlyBinaryMeshIterator<BR, P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: Read,
    BR: IsByteReader,
{
    header: FullHeader,
    p_iter: Option<PlyBinaryPointsIterator<BR, P, R, CHUNK_SIZE>>,
    f_iter: Option<PlyBinaryFacesIterator<BR, P, R, CHUNK_SIZE>>,
}

impl<BR, P, R, const CHUNK_SIZE: usize> PlyBinaryMeshIterator<BR, P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
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

impl<BR, P, R, const CHUNK_SIZE: usize> Iterator for PlyBinaryMeshIterator<BR, P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
    BR: IsByteReader,
{
    type Item = IOResult<StackVec<FaceDataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut p_iter) = self.p_iter {
            match p_iter.next() {
                Some(x) => return Some(x.map(|x| x.convert())),
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
            .map(|x| x.map(|x| x.convert()))
    }
}

impl<BR, P, R, const CHUNK_SIZE: usize> FusedIterator
    for PlyBinaryMeshIterator<BR, P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

pub enum BinaryOrAsciiPlyPointsInteralIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    Ascii(PlyAsciiPointsIterator<P, R, CHUNK_SIZE>),
    BinaryLittle(PlyBinaryPointsIterator<LittleReader, P, R, CHUNK_SIZE>),
    BinaryBig(PlyBinaryPointsIterator<BigReader, P, R, CHUNK_SIZE>),
}

//------------------------------------------------------------------------------

pub enum BinaryOrAsciiPlyMeshInteralIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    Ascii(PlyAsciiMeshIterator<P, R, CHUNK_SIZE>),
    BinaryLittle(PlyBinaryMeshIterator<LittleReader, P, R, CHUNK_SIZE>),
    BinaryBig(PlyBinaryMeshIterator<BigReader, P, R, CHUNK_SIZE>),
}

//------------------------------------------------------------------------------

pub struct PlyBinaryPointsIterator<BR, P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
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

impl<BR, P, R, const CHUNK_SIZE: usize> PlyBinaryPointsIterator<BR, P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
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
    fn fetch_one(&mut self) -> IOResult<P> {
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

impl<BR, P, R, const CHUNK_SIZE: usize> Iterator for PlyBinaryPointsIterator<BR, P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
    BR: IsByteReader,
{
    type Item = IOResult<StackVec<DataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if self.current < self.header.vertex.count {
                self.current += 1;
                match self.fetch_one() {
                    Err(e) => {
                        self.is_done = true;
                        return Some(Err(e));
                    }
                    Ok(x) => chunk.push(DataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                }
            } else {
                self.is_done = true;
                if chunk.has_data() {
                    return Some(Ok(chunk));
                }
                return None;
            }
        }
    }
}

impl<BR, P, R, const CHUNK_SIZE: usize> FusedIterator
    for PlyBinaryPointsIterator<BR, P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
    BR: IsByteReader,
{
}

//------------------------------------------------------------------------------

pub struct PlyAsciiPointsIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
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

impl<P, R, const CHUNK_SIZE: usize> PlyAsciiPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
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
    fn fetch_one(header: &PartialHeader, i_line: usize, line: &[u8]) -> IOResult<P> {
        let mut words = to_words_skip_empty(line);

        skip_n(&mut words, header.vertex.format.before.words);

        let first = words
            .next()
            .and_then(|w| fast_float::parse(w).ok())
            .ok_or(IOError::Vertex(Some(i_line)))?;

        skip_n(&mut words, header.vertex.format.between_first_snd.words);

        let snd = words
            .next()
            .and_then(|w| fast_float::parse(w).ok())
            .ok_or(IOError::Vertex(Some(i_line)))?;

        skip_n(&mut words, header.vertex.format.between_snd_third.words);

        let third = words
            .next()
            .and_then(|w| fast_float::parse(w).ok())
            .ok_or(IOError::Vertex(Some(i_line)))?;

        // no need to skip 'after' since we're done with this line anyway

        Ok(point_with_order(
            first,
            snd,
            third,
            header.vertex.format.order,
        ))
    }
}

impl<P, R, const CHUNK_SIZE: usize> Iterator for PlyAsciiPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
    type Item = IOResult<StackVec<DataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if self.current < self.header.vertex.count {
                //@todo error handling here might now diverge from previous version, double check
                //@todo outer else should already cause failure?
                self.current += 1;
                if let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                    self.i_line += 1;
                    match Self::fetch_one(&self.header, self.i_line, line) {
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                        Ok(x) => chunk.push(DataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                    }
                }
            } else {
                self.is_done = true;

                if self.current != self.header.vertex.count {
                    return Some(Err(IOError::VertexCount(Some(self.i_line))));
                } else {
                    if chunk.has_data() {
                        return Some(Ok(chunk));
                    }
                    return None;
                }
            }
        }
    }
}

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for PlyAsciiPointsIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

pub struct PlyAsciiFacesIterator<R, P, const CHUNK_SIZE: usize>
where
    R: BufRead,
{
    read: R,
    is_done: bool,
    header: FullHeader,
    current: usize,
    i_line: usize,
    line_buffer: Vec<u8>,
    phantom: PhantomData<P>,
}

impl<R, P, const CHUNK_SIZE: usize> PlyAsciiFacesIterator<R, P, CHUNK_SIZE>
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
            phantom: PhantomData::default(),
        }
    }
}

impl<R, P, const CHUNK_SIZE: usize> Iterator for PlyAsciiFacesIterator<R, P, CHUNK_SIZE>
where
    R: BufRead,
{
    type Item = IOResult<StackVec<FaceData<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if self.current < self.header.face.count {
                //@todo error handling here might now diverge from previous version, double check
                //@todo outer else should already cause failure?
                self.current += 1;
                if let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
                    self.i_line += 1;

                    match collect_index_line(&line) {
                        None => {
                            self.is_done = true;
                            return Some(Err(IOError::Face(Some(self.i_line))));
                        }
                        Some(x) => chunk.push(FaceData::Face(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                    }
                }
            } else {
                self.is_done = true;
                if chunk.has_data() {
                    return Some(Ok(chunk));
                }

                return None;
            }
        }
    }
}

impl<R, P, const CHUNK_SIZE: usize> FusedIterator for PlyAsciiFacesIterator<R, P, CHUNK_SIZE> where
    R: BufRead
{
}

//------------------------------------------------------------------------------

pub struct PlyBinaryFacesIterator<BR, P, R, const CHUNK_SIZE: usize>
where
    R: Read,
    BR: IsByteReader,
{
    read: R,
    is_done: bool,
    header: FullHeader,
    current: usize,
    phantom_br: PhantomData<BR>,
    phantom_p: PhantomData<P>,
}

impl<BR, P, R, const CHUNK_SIZE: usize> PlyBinaryFacesIterator<BR, P, R, CHUNK_SIZE>
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
            phantom_br: PhantomData,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_one(&mut self) -> IOResult<[usize; 3]> {
        skip_bytes(&mut self.read, self.header.face.format.before.bytes)?;

        let element_count = read_face_type::<BR, _>(&mut self.read, self.header.face.format.count)?;

        if element_count != 3 {
            return Err(IOError::Face(None));
        }

        let a = read_face_type::<BR, _>(&mut self.read, self.header.face.format.index)?;
        let b = read_face_type::<BR, _>(&mut self.read, self.header.face.format.index)?;
        let c = read_face_type::<BR, _>(&mut self.read, self.header.face.format.index)?;

        skip_bytes(&mut self.read, self.header.face.format.after.bytes)?;

        Ok([a, b, c])
    }
}

impl<BR, P, R, const CHUNK_SIZE: usize> Iterator for PlyBinaryFacesIterator<BR, P, R, CHUNK_SIZE>
where
    R: Read,
    BR: IsByteReader,
{
    type Item = IOResult<StackVec<FaceData<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if self.current < self.header.face.count {
                self.current += 1;
                match self.fetch_one() {
                    Err(e) => {
                        self.is_done = true;
                        return Some(Err(e));
                    }
                    Ok(x) => chunk.push(FaceData::Face(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                }
            } else {
                self.is_done = true;
                if chunk.has_data() {
                    return Some(Ok(chunk));
                }
                return None;
            }
        }
    }
}

impl<BR, P, R, const CHUNK_SIZE: usize> FusedIterator
    for PlyBinaryFacesIterator<BR, P, R, CHUNK_SIZE>
where
    R: Read,
    BR: IsByteReader,
{
}
