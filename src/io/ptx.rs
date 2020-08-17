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

use std::{
    fmt,
    io::{BufRead, Error as ioError},
    iter::FusedIterator,
    marker::PhantomData,
};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load a .ptx file
pub struct PtxIterator<IP, P, R>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
    read: R,
    i_line: usize,
    line_buffer: Vec<u8>,
    n_points_to_fetch: usize,
    must_transform: bool,
    transformation: Matrix4,
    phantom_ip: PhantomData<IP>,
    phantom_p: PhantomData<P>,
}

impl<IP, P, R> PtxIterator<IP, P, R>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            i_line: 0,
            line_buffer: Vec::new(),
            n_points_to_fetch: 0,
            must_transform: false,
            transformation: Matrix4::identity(),
            phantom_ip: PhantomData,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_one(
        i_line: usize,
        line: &[u8],
        must_transform: bool,
        transformation: &Matrix4,
    ) -> PtxResult<P> {
        let mut words = to_words_skip_empty(line);

        let x = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(PtxError::Point)
            .line(i_line, line)?;
        let y = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(PtxError::Point)
            .line(i_line, line)?;
        let z = words
            .next()
            .and_then(|w| from_ascii(w))
            .ok_or(PtxError::Point)
            .line(i_line, line)?;

        let mut p = P::new(x, y, z);

        if must_transform {
            p.transform(transformation)
        }

        Ok(p)
    }

    #[inline(always)]
    fn fetch_header(&mut self, columns: usize) -> PtxResult<()> {
        let mut line = fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;

        let rows: usize = from_ascii(line)
            .ok_or(PtxError::Rows)
            .line(self.i_line, line)?;

        // skip scanner position line
        fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;

        // skip scanner x-axis line
        fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;

        // skip scanner y-axis line
        fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;

        // skip scanner z-axis line
        fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;

        line = fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;
        let [m11, m12, m13, m14] = read_matrix_row(line)
            .ok_or(PtxError::Matrix)
            .line(self.i_line, line)?;

        line = fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;
        let [m21, m22, m23, m24] = read_matrix_row(line)
            .ok_or(PtxError::Matrix)
            .line(self.i_line, line)?;

        line = fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;
        let [m31, m32, m33, m34] = read_matrix_row(line)
            .ok_or(PtxError::Matrix)
            .line(self.i_line, line)?;

        line = fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line)?;
        self.i_line += 1;
        let [m41, m42, m43, m44] = read_matrix_row(line)
            .ok_or(PtxError::Matrix)
            .line(self.i_line, line)?;

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

impl<IP, P, R> Iterator for PtxIterator<IP, P, R>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
    type Item = PtxResult<ReserveOrData<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_points_to_fetch == 0 {
            let first_line = fetch_line(&mut self.read, &mut self.line_buffer);
            if first_line.is_err() {
                return None;
            }
            self.i_line += 1;
            // unwrap safe, is_err() checked above
            match from_ascii(first_line.as_ref().unwrap())
                .ok_or(PtxError::Columns)
                .index(self.i_line)
                .and_then(|columns| self.fetch_header(columns))
            {
                Ok(()) => return Some(Ok(ReserveOrData::Reserve(self.n_points_to_fetch))),
                Err(e) => return Some(Err(e)),
            }
        } else if self.n_points_to_fetch > 0 {
            self.n_points_to_fetch -= 1;
            match fetch_line(&mut self.read, &mut self.line_buffer).index(self.i_line) {
                Ok(line) => {
                    self.i_line += 1;
                    Some(
                        Self::fetch_one(
                            self.i_line,
                            line,
                            self.must_transform,
                            &self.transformation,
                        )
                        .map(|x| ReserveOrData::Data(x)),
                    )
                }
                Err(e) => Some(Err(e.into())),
            }
        } else {
            None
        }
    }
}

impl<IP, P, R> FusedIterator for PtxIterator<IP, P, R>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Loads points from .ptx file into IsPushable<Is3D>
pub fn load_ptx<IP, P, R>(read: R, ip: &mut IP) -> PtxResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
    let iterator = PtxIterator::<IP, P, R>::new(read);

    for rd in iterator {
        match rd? {
            ReserveOrData::Reserve(x) => ip.reserve(x),
            ReserveOrData::Data(x) => ip.push(x),
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

//------------------------------------------------------------------------------

/// Error type for .ptx file operations
pub enum PtxError {
    LoadFileEndReached,
    AccessFile,
    Columns,
    Rows,
    Matrix,
    Point,
}

/// Result type for .ptx file operations
pub type PtxResult<T> = IOResult<T, PtxError>;

impl fmt::Debug for PtxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadFileEndReached => write!(f, "Unexpected reach of .ptx file end"),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::Columns => write!(f, "Columns could not be parsed"),
            Self::Rows => write!(f, "Rows could not be parsed"),
            Self::Matrix => write!(f, "Transformation matrix could not be parsed"),
            Self::Point => write!(f, "Point could not be parsed"),
        }
    }
}

impl fmt::Display for PtxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ioError> for PtxError {
    fn from(_error: ioError) -> Self {
        PtxError::AccessFile
    }
}

impl From<FetchLineError> for PtxError {
    fn from(_error: FetchLineError) -> Self {
        PtxError::LoadFileEndReached
    }
}

impl From<WithLineInfo<FetchLineError>> for WithLineInfo<PtxError> {
    fn from(other: WithLineInfo<FetchLineError>) -> Self {
        match other {
            WithLineInfo::<FetchLineError>::None(x) => WithLineInfo::None(PtxError::from(x)),
            WithLineInfo::<FetchLineError>::Index(i, x) => {
                WithLineInfo::Index(i, PtxError::from(x))
            }
            WithLineInfo::<FetchLineError>::Line(i, l, x) => {
                WithLineInfo::Line(i, l, PtxError::from(x))
            }
        }
    }
}
