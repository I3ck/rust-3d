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

//! Module for interal utility functions for IO operations

use std::{
    fmt,
    io::{BufRead, Error as ioError, Read},
    str::FromStr,
};

//------------------------------------------------------------------------------

/// Skip number of bytes
#[inline(always)]
pub fn skip_bytes<R>(read: &mut R, n_bytes: usize) -> std::io::Result<()>
where
    R: Read,
{
    let mut buffer = [0u8; 1];
    for _ in 0..n_bytes {
        let _ = read.read_exact(&mut buffer)?;
    }

    Ok(())
}

/// Skip number of elements
#[inline(always)]
pub fn skip_n<I>(i: &mut I, n: usize)
where
    I: Iterator,
{
    for _ in 0..n {
        i.next();
    }
}

//------------------------------------------------------------------------------

/// Trims white space at the start of the input
pub fn trim_start(text: &[u8]) -> &[u8] {
    let mut to_drop = 0;
    for c in text {
        if !(*c == b' ' || *c == b'\t') {
            break;
        }
        to_drop += 1;
    }

    &text[to_drop..]
}

//------------------------------------------------------------------------------

/// Reads a FromStr from bytes
#[inline(always)]
pub fn from_bytes<T>(bytes: &[u8]) -> Option<T>
where
    T: FromStr,
{
    std::str::from_utf8(bytes)
        .ok()
        .and_then(|x| T::from_str(x).ok())
}

//------------------------------------------------------------------------------

/// Fetch a single line
#[inline(always)]
pub fn fetch_line<'a, R>(read: &mut R, line_buffer: &'a mut Vec<u8>) -> FetchLineResult<&'a [u8]>
where
    R: BufRead,
{
    line_buffer.clear();
    let n_read = read.read_until(b'\n', line_buffer)?;
    if n_read == 0 {
        return Err(FetchLineError);
    }

    // We must drop the '\n' we read_until for sure
    // And might also have to drop additional whitespace
    let mut ignore_end = 1;
    for i in 1..line_buffer.len() {
        if (line_buffer[line_buffer.len() - i - 1] as char).is_whitespace() {
            ignore_end += 1;
        } else {
            break;
        }
    }

    Ok(&line_buffer[0..line_buffer.len() - ignore_end])
}

//------------------------------------------------------------------------------

/// Error type for the fetch_line function
pub struct FetchLineError;

/// Result type for the fetch_line function
pub type FetchLineResult<T> = std::result::Result<T, FetchLineError>;

impl fmt::Debug for FetchLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unable to fetch line")
    }
}

impl From<ioError> for FetchLineError {
    fn from(_error: ioError) -> Self {
        FetchLineError {}
    }
}
