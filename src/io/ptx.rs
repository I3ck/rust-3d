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
};

use super::utils::*;

//------------------------------------------------------------------------------

/// Loads points from .ptx file into IsPushable<Is3D>
pub fn load_ptx<IP, P, R>(read: &mut R, ip: &mut IP) -> PtxResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
    let mut i_line = 0;
    let mut line_buffer = Vec::new();

    let mut line: &[u8];

    loop {
        let columns: usize;
        {
            let first_line = fetch_line(read, &mut line_buffer);
            if first_line.is_err() {
                break;
            }
            i_line += 1;

            columns = from_bytes(first_line.unwrap()).ok_or(PtxError::Columns(i_line))?;
            // safe, since first_line being err causing break
        }

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        let rows: usize = from_bytes(line).ok_or(PtxError::Rows(i_line))?;

        // skip scanner position line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        // skip scanner x-axis line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        // skip scanner y-axis line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        // skip scanner z-axis line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m11, m12, m13, m14] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m21, m22, m23, m24] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m31, m32, m33, m34] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m41, m42, m43, m44] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        let m = Matrix4 {
            data: [
                [m11, m12, m13, m14],
                [m21, m22, m23, m24],
                [m31, m32, m33, m34],
                [m41, m42, m43, m44],
            ],
        };

        let must_transform = m != Matrix4::identity();

        let n = columns * rows;

        ip.reserve(n);

        for _ in 0..n {
            line = fetch_line(read, &mut line_buffer)?;
            i_line += 1;

            //@todo also as helper?
            let mut words = line.split(|x| *x == b' ' || *x == b'\t').skip_empty();

            let x = words
                .next()
                .and_then(|w| from_bytes(w))
                .ok_or(PtxError::Point(i_line))?;
            let y = words
                .next()
                .and_then(|w| from_bytes(w))
                .ok_or(PtxError::Point(i_line))?;
            let z = words
                .next()
                .and_then(|w| from_bytes(w))
                .ok_or(PtxError::Point(i_line))?;

            let mut p = P::new(x, y, z);

            if must_transform {
                p.transform(&m)
            }
            ip.push(p)
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
fn read_matrix_row(line: &[u8]) -> Option<[f64; 4]> {
    //@todo also as helper?
    let mut words = line.split(|x| *x == b' ' || *x == b'\t').skip_empty();

    let a = from_bytes(words.next()?)?;
    let b = from_bytes(words.next()?)?;
    let c = from_bytes(words.next()?)?;
    let d = from_bytes(words.next()?)?;

    Some([a, b, c, d])
}

//------------------------------------------------------------------------------

/// Error type for .ptx file operations
pub enum PtxError {
    LoadFileEndReached,
    AccessFile,
    LineParse(usize),
    Columns(usize),
    Rows(usize),
    Matrix(usize),
    Point(usize),
}

/// Result type for .ptx file operations
pub type PtxResult<T> = std::result::Result<T, PtxError>;

impl fmt::Debug for PtxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadFileEndReached => write!(f, "Unexpected reach of .ptx file end"),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::Columns(x) => write!(f, "Columns could not be parsed on line {}", x),
            Self::Rows(x) => write!(f, "Rows could not be parsed on line {}", x),
            Self::Matrix(x) => write!(f, "Transformation matrix could not be parsed on line {}", x),
            Self::Point(x) => write!(f, "Point could not be parsed on line {}", x),
        }
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
