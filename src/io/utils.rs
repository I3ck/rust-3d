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
};

//------------------------------------------------------------------------------

pub trait ByteReader {
    fn read_i8<R>(read: &mut R) -> std::io::Result<i8>
    where
        R: Read;

    fn read_u8<R>(read: &mut R) -> std::io::Result<u8>
    where
        R: Read;

    fn read_i16<R>(read: &mut R) -> std::io::Result<i16>
    where
        R: Read;

    fn read_u16<R>(read: &mut R) -> std::io::Result<u16>
    where
        R: Read;

    fn read_i32<R>(read: &mut R) -> std::io::Result<i32>
    where
        R: Read;

    fn read_u32<R>(read: &mut R) -> std::io::Result<u32>
    where
        R: Read;

    fn read_f32<R>(read: &mut R) -> std::io::Result<f32>
    where
        R: Read;

    fn read_i64<R>(read: &mut R) -> std::io::Result<i64>
    where
        R: Read;

    fn read_u64<R>(read: &mut R) -> std::io::Result<u64>
    where
        R: Read;

    fn read_f64<R>(read: &mut R) -> std::io::Result<f64>
    where
        R: Read;
}

pub struct LittleReader {}

impl ByteReader for LittleReader {
    fn read_i8<R>(read: &mut R) -> std::io::Result<i8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(i8::from_le_bytes(buffer))
    }

    fn read_u8<R>(read: &mut R) -> std::io::Result<u8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }

    fn read_i16<R>(read: &mut R) -> std::io::Result<i16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(i16::from_le_bytes(buffer))
    }

    fn read_u16<R>(read: &mut R) -> std::io::Result<u16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    fn read_i32<R>(read: &mut R) -> std::io::Result<i32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }

    fn read_u32<R>(read: &mut R) -> std::io::Result<u32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    fn read_f32<R>(read: &mut R) -> std::io::Result<f32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(f32::from_le_bytes(buffer))
    }

    fn read_i64<R>(read: &mut R) -> std::io::Result<i64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(i64::from_le_bytes(buffer))
    }

    fn read_u64<R>(read: &mut R) -> std::io::Result<u64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    fn read_f64<R>(read: &mut R) -> std::io::Result<f64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(f64::from_le_bytes(buffer))
    }
}

pub struct BigReader {}

impl ByteReader for BigReader {
    fn read_i8<R>(read: &mut R) -> std::io::Result<i8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(i8::from_be_bytes(buffer))
    }

    fn read_u8<R>(read: &mut R) -> std::io::Result<u8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(u8::from_be_bytes(buffer))
    }

    fn read_i16<R>(read: &mut R) -> std::io::Result<i16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    fn read_u16<R>(read: &mut R) -> std::io::Result<u16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_i32<R>(read: &mut R) -> std::io::Result<i32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(i32::from_be_bytes(buffer))
    }

    fn read_u32<R>(read: &mut R) -> std::io::Result<u32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    fn read_f32<R>(read: &mut R) -> std::io::Result<f32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(f32::from_be_bytes(buffer))
    }

    fn read_i64<R>(read: &mut R) -> std::io::Result<i64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }

    fn read_u64<R>(read: &mut R) -> std::io::Result<u64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    fn read_f64<R>(read: &mut R) -> std::io::Result<f64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(f64::from_be_bytes(buffer))
    }
}

//------------------------------------------------------------------------------

/// Skip number of bytes
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
pub fn skip_n<I>(i: &mut I, n: usize)
where
    I: Iterator,
{
    for _ in 0..n {
        i.next();
    }
}

//------------------------------------------------------------------------------

/// Fetch a single line
pub fn fetch_line<'a, R>(read: &mut R, line_buffer: &'a mut String) -> FetchLineResult<&'a str>
where
    R: BufRead,
{
    line_buffer.clear();
    let n_read = read.read_line(line_buffer)?;
    if n_read == 0 {
        return Err(FetchLineError);
    }

    Ok(line_buffer.trim_end())
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
