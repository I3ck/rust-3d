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

//! Module for IO operations of the .pts file format

use crate::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError},
    iter::FusedIterator,
    marker::PhantomData,
};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load a .pts file
pub struct PtsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    n_vertices: Option<usize>,
    n_vertices_added: usize,
    phantom_p: PhantomData<P>,
}

impl<P, R> PtsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            is_done: false,
            i_line: 0,
            line_buffer: Vec::new(),
            n_vertices: None,
            n_vertices_added: 0,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    pub fn fetch_one(line: &[u8]) -> PtsResult<P> {
        let mut words = to_words_skip_empty(line);

        let x = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(PtsError::Vertex)?;

        let y = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(PtsError::Vertex)?;

        let z = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(PtsError::Vertex)?;

        Ok(P::new(x, y, z))
    }
}

impl<P, R> Iterator for PtsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
    type Item = PtsIOResult<DataReserve<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        while let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;

            if line.is_empty() {
                continue;
            }

            match self.n_vertices {
                None => {
                    let mut words = to_words_skip_empty(line);
                    self.n_vertices = match words
                        .next()
                        .and_then(|word| from_ascii(word))
                        .ok_or(PtsError::VertexCount)
                        .line(self.i_line, line)
                    {
                        Ok(n) => Some(n),
                        Err(e) => {
                            self.is_done = true;
                            return Some(Err(e));
                        }
                    };
                    // unwrap safe since assigned above
                    return Some(Ok(DataReserve::Reserve(self.n_vertices.unwrap())));
                }
                Some(n) => {
                    if self.n_vertices_added < n {
                        self.n_vertices_added += 1;
                        return Some(
                            Self::fetch_one(line)
                                .map(|x| DataReserve::Data(x))
                                .line(self.i_line, line)
                                .map_err(|e| {
                                    self.is_done = true;
                                    e
                                }),
                        );
                    } else {
                        // New block
                        self.n_vertices_added = 0;
                        let mut words = to_words_skip_empty(line);
                        self.n_vertices = match words
                            .next()
                            .and_then(|word| from_ascii(word))
                            .ok_or(PtsError::VertexCount)
                            .line(self.i_line, line)
                        {
                            Ok(n) => Some(n),
                            Err(e) => {
                                self.is_done = true;
                                return Some(Err(e));
                            }
                        };
                        // unwrap safe since assigned above
                        return Some(Ok(DataReserve::Reserve(self.n_vertices.unwrap())));
                    }
                }
            }
        }

        self.is_done = true;

        None
    }
}

impl<P, R> FusedIterator for PtsIterator<P, R>
where
    P: IsBuildable3D,
    R: BufRead,
{
}

/// Loads IsPushable<Is3D> from the .pts file format
pub fn load_pts<IP, P, R>(read: R, ip: &mut IP) -> PtsIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let iterator = PtsIterator::new(read);

    for rd in iterator {
        match rd? {
            DataReserve::Reserve(x) => ip.reserve(x),
            DataReserve::Data(x) => ip.push(x),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .pts file operations
pub enum PtsError {
    AccessFile,
    VertexCount,
    Vertex,
}

/// Result type for .pts file operations
pub type PtsIOResult<T> = IOResult<T, PtsError>;
type PtsResult<T> = std::result::Result<T, PtsError>;

impl fmt::Debug for PtsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::VertexCount => write!(f, "Unable to parse vertex count"),
            Self::Vertex => write!(f, "Unable to parse vertex"),
        }
    }
}

impl fmt::Display for PtsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ioError> for PtsError {
    fn from(_error: ioError) -> Self {
        PtsError::AccessFile
    }
}
