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

//! Module containing the IsByteReader trait and Little/BigReader implementations for reading binary data

use std::io::{Read, Result as iRes};

//------------------------------------------------------------------------------

/// Trait for binary data readers
pub trait IsByteReader {
    fn read_i8<R>(read: &mut R) -> iRes<i8>
    where
        R: Read;

    fn read_u8<R>(read: &mut R) -> iRes<u8>
    where
        R: Read;

    fn read_i16<R>(read: &mut R) -> iRes<i16>
    where
        R: Read;

    fn read_u16<R>(read: &mut R) -> iRes<u16>
    where
        R: Read;

    fn read_i32<R>(read: &mut R) -> iRes<i32>
    where
        R: Read;

    fn read_u32<R>(read: &mut R) -> iRes<u32>
    where
        R: Read;

    fn read_f32<R>(read: &mut R) -> iRes<f32>
    where
        R: Read;

    fn read_i64<R>(read: &mut R) -> iRes<i64>
    where
        R: Read;

    fn read_u64<R>(read: &mut R) -> iRes<u64>
    where
        R: Read;

    fn read_f64<R>(read: &mut R) -> iRes<f64>
    where
        R: Read;
}

//------------------------------------------------------------------------------

/// Reader for binary data using little endian
pub struct LittleReader {}

impl IsByteReader for LittleReader {
    #[inline(always)]
    fn read_i8<R>(read: &mut R) -> iRes<i8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(i8::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_u8<R>(read: &mut R) -> iRes<u8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(u8::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_i16<R>(read: &mut R) -> iRes<i16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(i16::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_u16<R>(read: &mut R) -> iRes<u16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_i32<R>(read: &mut R) -> iRes<i32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_u32<R>(read: &mut R) -> iRes<u32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_f32<R>(read: &mut R) -> iRes<f32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(f32::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_i64<R>(read: &mut R) -> iRes<i64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(i64::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_u64<R>(read: &mut R) -> iRes<u64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    #[inline(always)]
    fn read_f64<R>(read: &mut R) -> iRes<f64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(f64::from_le_bytes(buffer))
    }
}

//------------------------------------------------------------------------------

/// Reader for binary data using big endian
pub struct BigReader {}

impl IsByteReader for BigReader {
    #[inline(always)]
    fn read_i8<R>(read: &mut R) -> iRes<i8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(i8::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_u8<R>(read: &mut R) -> iRes<u8>
    where
        R: Read,
    {
        let mut buffer = [0u8; 1];
        read.read_exact(&mut buffer)?;
        Ok(u8::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_i16<R>(read: &mut R) -> iRes<i16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_u16<R>(read: &mut R) -> iRes<u16>
    where
        R: Read,
    {
        let mut buffer = [0u8; 2];
        read.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_i32<R>(read: &mut R) -> iRes<i32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(i32::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_u32<R>(read: &mut R) -> iRes<u32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_f32<R>(read: &mut R) -> iRes<f32>
    where
        R: Read,
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
        Ok(f32::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_i64<R>(read: &mut R) -> iRes<i64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_u64<R>(read: &mut R) -> iRes<u64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    #[inline(always)]
    fn read_f64<R>(read: &mut R) -> iRes<f64>
    where
        R: Read,
    {
        let mut buffer = [0u8; 8];
        read.read_exact(&mut buffer)?;
        Ok(f64::from_be_bytes(buffer))
    }
}
