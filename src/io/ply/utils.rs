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

//! Module for interal util functions for IO operations of the ply file format

use crate::*;

use super::types::*;

use byteorder::{ByteOrder, ReadBytesExt};

use std::io::Read;

use core::str::FromStr;

//------------------------------------------------------------------------------

pub fn collect_index_line(line: &str) -> Option<[usize; 3]> {
    let mut words = to_words(line);
    if words.next()? != "3" {
        return None;
    }

    let a = usize::from_str(words.next()?).ok()?;
    let b = usize::from_str(words.next()?).ok()?;
    let c = usize::from_str(words.next()?).ok()?;

    Some([a, b, c])
}

//------------------------------------------------------------------------------

pub fn read_face_type<BO, R>(read: &mut R, t: &FaceType) -> PlyResult<usize>
where
    BO: ByteOrder,
    R: Read,
{
    Ok(match t {
        FaceType::Char => read.read_i8()? as usize,
        FaceType::UChar => read.read_u8()? as usize,
        FaceType::Short => read.read_i16::<BO>()? as usize,
        FaceType::UShort => read.read_u16::<BO>()? as usize,
        FaceType::Int => read.read_i32::<BO>()? as usize,
        FaceType::UInt => read.read_u32::<BO>()? as usize,
    })
}

//------------------------------------------------------------------------------

pub fn read_vertex_type<BO, R>(read: &mut R, t: &VertexType) -> PlyResult<f64>
where
    BO: ByteOrder,
    R: Read,
{
    Ok(match t {
        VertexType::Float => read.read_f32::<BO>()? as f64,
        VertexType::Double => read.read_f64::<BO>()?,
    })
}

//------------------------------------------------------------------------------

pub fn skip_bytes<R>(read: &mut R, n_bytes: usize)
where
    R: Read,
{
    for _ in 0..n_bytes {
        let _ = read.read_u8();
    }
}

pub fn skip_n<I>(i: &mut I, n: usize)
where
    I: Iterator,
{
    for _ in 0..n {
        i.next();
    }
}
