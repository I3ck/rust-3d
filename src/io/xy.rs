/*
Copyright 2017 Martin Buck

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

//! Module for IO of the xy file format

use crate::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError, Write},
    iter::FusedIterator,
    marker::PhantomData,
};

use super::{types::*, utils::*};

//------------------------------------------------------------------------------

/// Iterator to incrementally load a .xy file
pub struct XyIterator<P, R>
where
    P: IsBuildable2D,
    R: BufRead,
{
    read: R,
    is_done: bool,
    i_line: usize,
    line_buffer: Vec<u8>,
    delim_determined: bool,
    delim: u8,
    phantom_p: PhantomData<P>,
}

impl<P, R> XyIterator<P, R>
where
    P: IsBuildable2D,
    R: BufRead,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            is_done: false,
            i_line: 0,
            line_buffer: Vec::new(),
            delim_determined: false,
            delim: 0,
            phantom_p: PhantomData,
        }
    }

    #[inline(always)]
    fn fetch_one(delim_determined: &mut bool, delim: &mut u8, line: &[u8]) -> XyResult<P> {
        if !*delim_determined {
            *delim = estimate_delimiter(1, &line).ok_or(XyError::EstimateDelimiter)?;
            *delim_determined = true;
        }

        let mut words = line.split(|x| *x == *delim).skip_empty();

        let x = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(XyError::Vertex)?;

        let y = words
            .next()
            .and_then(|word| from_ascii(word))
            .ok_or(XyError::Vertex)?;

        Ok(P::new(x, y))
    }
}

impl<P, R> Iterator for XyIterator<P, R>
where
    P: IsBuildable2D,
    R: BufRead,
{
    type Item = XyIOResult<P>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        if let Ok(line) = fetch_line(&mut self.read, &mut self.line_buffer) {
            self.i_line += 1;
            Some(
                Self::fetch_one(&mut self.delim_determined, &mut self.delim, line)
                    .line(self.i_line, line)
                    .map_err(|e| {
                        self.is_done = true;
                        e
                    }),
            )
        } else {
            self.is_done = true;
            None
        }
    }
}

impl<P, R> FusedIterator for XyIterator<P, R>
where
    P: IsBuildable2D,
    R: BufRead,
{
}

//------------------------------------------------------------------------------

/// Saves an IsRandomAccessible<Is2D> as x y coordinates with a specified delimiter between coordinates and positions. E.g. used to create the .xy file format or .csv files
pub fn save_xy<RA, P, W>(write: &mut W, ra: &RA, delim_coord: &str, delim_pos: &str) -> XyResult<()>
where
    RA: IsRandomAccessible<P>,
    P: Is2D,
    W: Write,
{
    let n = ra.len();
    for i in 0..n {
        let ref p = ra[i];
        let buffer = p.x().to_string() + delim_coord + &p.y().to_string() + delim_pos;
        write.write_all(buffer.as_bytes())?;
    }
    Ok(())
}

/// Loads a IsPushable<Is2D> as x y coordinates. E.g. used to load the .xy file format or .csv files
pub fn load_xy<IP, P, R>(read: R, ip: &mut IP) -> XyIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable2D,
    R: BufRead,
{
    let iterator = XyIterator::new(read);

    for p in iterator {
        ip.push(p?)
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .xy file operations
pub enum XyError {
    EstimateDelimiter,
    AccessFile,
    Vertex,
}

/// Result type for .xy file operations
pub type XyResult<T> = std::result::Result<T, XyError>;

/// Result type for .xy file operations
pub type XyIOResult<T> = IOResult<T, XyError>;

impl fmt::Debug for XyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Vertex => write!(f, "Unable to parse vertex"),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::EstimateDelimiter => write!(f, "Unable to estimate delimiter"),
        }
    }
}

impl fmt::Display for XyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ioError> for XyError {
    fn from(_error: ioError) -> Self {
        XyError::AccessFile
    }
}
