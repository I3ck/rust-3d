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

//! Module for IO operations of the ptx file format

use crate::*;

use std::{io::BufRead, iter::FusedIterator, marker::PhantomData};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load a .ptx file
pub struct PtxIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + IsMatrix4Transformable + Default,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    n_points_to_fetch: usize,
    must_transform: bool,
    transformation: Matrix4,
    phantom_p: PhantomData<P>,
}

impl<P, R, const CHUNK_SIZE: usize> PtxIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + IsMatrix4Transformable + Default,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            is_done: false,
            i_line: 0,
            line_buffer: Vec::new(),
            n_points_to_fetch: 0,
            must_transform: false,
            transformation: Matrix4::identity(),
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_one(
        i_line: usize,
        line: &[u8],
        must_transform: bool,
        transformation: &Matrix4,
    ) -> IOResult<P> {
        let mut words = to_words_skip_empty(line);

        let x = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(IOError::Vertex(Some(i_line)))?;
        let y = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(IOError::Vertex(Some(i_line)))?;
        let z = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(IOError::Vertex(Some(i_line)))?;

        let mut p = P::new(x, y, z);

        if must_transform {
            p.transform(transformation)
        }

        Ok(p)
    }

    #[inline(always)]
    fn fetch_header(&mut self, columns: usize) -> IOResult<()> {
        let mut line = fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;

        let rows: usize = from_ascii(line).ok_or(IOError::Rows(self.i_line))?;

        // skip scanner position line
        fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;

        // skip scanner x-axis line
        fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;

        // skip scanner y-axis line
        fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;

        // skip scanner z-axis line
        fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;

        line = fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;
        let [m11, m12, m13, m14] = read_matrix_row(line).ok_or(IOError::Matrix(self.i_line))?;

        line = fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;
        let [m21, m22, m23, m24] = read_matrix_row(line).ok_or(IOError::Matrix(self.i_line))?;

        line = fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;
        let [m31, m32, m33, m34] = read_matrix_row(line).ok_or(IOError::Matrix(self.i_line))?;

        line = fetch_line(&mut self.read, &mut self.line_buffer)?;
        self.i_line += 1;
        let [m41, m42, m43, m44] = read_matrix_row(line).ok_or(IOError::Matrix(self.i_line))?;

        self.transformation = Matrix4 {
            data: [
                [m11, m12, m13, m14],
                [m21, m22, m23, m24],
                [m31, m32, m33, m34],
                [m41, m42, m43, m44],
            ],
        };

        self.must_transform = self.transformation != Matrix4::identity();

        self.n_points_to_fetch = rows * columns;

        Ok(())
    }
}

impl<P, R, const CHUNK_SIZE: usize> Iterator for PtxIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + IsMatrix4Transformable + Default,
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
            } else if self.n_points_to_fetch == 0 {
                let first_line = fetch_line(&mut self.read, &mut self.line_buffer);
                if first_line.is_err() {
                    self.is_done = true;
                    if chunk.has_data() {
                        return Some(Ok(chunk));
                    }
                    return None;
                }

                self.i_line += 1;
                // unwrap safe, is_err() checked above
                match from_ascii(first_line.as_ref().unwrap())
                    .ok_or(IOError::Columns(self.i_line))
                    .and_then(|columns| self.fetch_header(columns))
                {
                    Ok(()) => {
                        chunk
                            .push(DataReserve::Reserve(self.n_points_to_fetch))
                            .unwrap() // unwrap safe since we only call this if chunk.has_space()
                    }
                    Err(e) => {
                        self.is_done = true;
                        return Some(Err(e));
                    }
                }
            } else if self.n_points_to_fetch > 0 {
                self.n_points_to_fetch -= 1;
                match fetch_line(&mut self.read, &mut self.line_buffer) {
                    Ok(line) => {
                        self.i_line += 1;
                        match Self::fetch_one(
                            self.i_line,
                            line,
                            self.must_transform,
                            &self.transformation,
                        ) {
                            Err(e) => {
                                self.is_done = true;
                                return Some(Err(e));
                            }
                            Ok(x) => chunk.push(DataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
                        }
                    }
                    Err(e) => {
                        self.is_done = true;
                        return Some(Err(e.into()));
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

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for PtxIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + IsMatrix4Transformable + Default,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads points from .ptx file into IsPushable<Is3D>
pub fn load_ptx<IP, P, R, const CHUNK_SIZE: usize>(read: R, ip: &mut IP) -> IOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable + Default,
    R: BufRead,
{
    let iterator = PtxIterator::<_, _, CHUNK_SIZE>::new(read);

    for chunk in iterator {
        for x in chunk? {
            match x {
                DataReserve::Data(x) => ip.push(x),
                DataReserve::Reserve(x) => ip.reserve(x),
                DataReserve::ReserveExact(x) => ip.reserve_exact(x),
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
fn read_matrix_row(line: &[u8]) -> Option<[f64; 4]> {
    let mut words = to_words_skip_empty(line);

    let a = from_ascii(words.next()?)?;
    let b = from_ascii(words.next()?)?;
    let c = from_ascii(words.next()?)?;
    let d = from_ascii(words.next()?)?;

    Some([a, b, c, d])
}
