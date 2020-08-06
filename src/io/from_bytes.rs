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

//! Module containing the FromBytes trait

use std::convert::TryInto;

//------------------------------------------------------------------------------

#[macro_export]
macro_rules! array_from_bytes_le {
    ($t:ty , $n:expr , $bytes:expr) => {{
        let size = std::mem::size_of::<$t>();
        if $bytes.len() != size * $n {
            return Err(FromBytesError::SizeMismatch)?;
        }

        let mut arr: [$t; $n] = [<$t>::default(); $n];
        for i in 0..$n {
            arr[i] = <$t>::from_le_slice(&$bytes[i * size..(i + 1) * size])?
        }
        let result: FromBytesResult<[$t; $n]> = Ok(arr);
        result
    }};
}

#[macro_export]
macro_rules! array_from_bytes_be {
    ($t:ty , $n:expr , $bytes:expr) => {{
        let size = std::mem::size_of::<$t>();
        if $bytes.len() != size * $n {
            return Err(FromBytesError::SizeMismatch)?;
        }

        let mut arr: [$t; $n] = [<$t>::default(); $n];
        for i in 0..$n {
            arr[i] = <$t>::from_be_slice(&$bytes[i * size..(i + 1) * size])?
        }
        let result: FromBytesResult<[$t; $n]> = Ok(arr);
        result
    }};
}

//------------------------------------------------------------------------------

#[inline(always)]
#[allow(unused)]
pub fn slice_from_bytes_le<FB>(bytes: &[u8], target: &mut [FB]) -> FromBytesResult<()>
where
    FB: FromBytes,
{
    let size = std::mem::size_of::<FB>();

    if bytes.len() != size * target.len() {
        return Err(FromBytesError::SizeMismatch);
    }

    for i in 0..target.len() {
        target[i] = FB::from_le_slice(&bytes[i * size..(i + 1) * size])?
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
#[allow(unused)]
pub fn slice_from_bytes_be<FB>(bytes: &[u8], target: &mut [FB]) -> FromBytesResult<()>
where
    FB: FromBytes,
{
    let size = std::mem::size_of::<FB>();

    if size * bytes.len() != target.len() {
        return Err(FromBytesError::SizeMismatch);
    }

    for i in 0..target.len() {
        target[i] = FB::from_be_slice(&bytes[i * size..(i + 1) * size])?
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Trait for the conversion from byte slices (size is checked at run time)
pub trait FromBytes: Sized {
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self>;
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self>;
}

//------------------------------------------------------------------------------

impl FromBytes for i8 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i8::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i8::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for u8 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u8::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u8::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for i16 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i16::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i16::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for u16 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u16::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u16::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for i32 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i32::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i32::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for u32 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u32::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u32::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for f32 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(f32::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(f32::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for i64 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i64::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(i64::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for u64 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u64::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(u64::from_be_bytes(bytes.try_into()?))
    }
}

impl FromBytes for f64 {
    #[inline(always)]
    fn from_le_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(f64::from_le_bytes(bytes.try_into()?))
    }
    #[inline(always)]
    fn from_be_slice(bytes: &[u8]) -> FromBytesResult<Self> {
        Ok(f64::from_be_bytes(bytes.try_into()?))
    }
}

//------------------------------------------------------------------------------

/// Error type for the FromBytes conversion
pub enum FromBytesError {
    BinaryData,
    SizeMismatch,
}

/// Result type for the FromBytes conversion
pub type FromBytesResult<T> = std::result::Result<T, FromBytesError>;

impl From<std::array::TryFromSliceError> for FromBytesError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        FromBytesError::BinaryData
    }
}

//------------------------------------------------------------------------------
