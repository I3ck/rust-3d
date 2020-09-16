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

//! Module for interal types for IO operations of the ply file format

use core::convert::TryFrom;

use std::fmt;

use super::super::types::*;

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}

impl Type {
    #[inline(always)]
    pub fn size_bytes(&self) -> usize {
        match self {
            Self::Char => 1,
            Self::UChar => 1,
            Self::Short => 2,
            Self::UShort => 2,
            Self::Int => 4,
            Self::UInt => 4,
            Self::Float => 4,
            Self::Double => 8,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&[u8]> for Type {
    type Error = String;

    fn try_from(x: &[u8]) -> std::result::Result<Self, Self::Error> {
        match x {
            b"char" | b"int8" => Ok(Self::Char),
            b"uchar" | b"uint8" => Ok(Self::UChar),
            b"short" | b"int16" => Ok(Self::Short),
            b"ushort" | b"uint16" => Ok(Self::UShort),
            b"int" | b"int32" => Ok(Self::Int),
            b"uint" | b"uint32" => Ok(Self::UInt),
            b"float" | b"float32" => Ok(Self::Float),
            b"double" | b"float64" => Ok(Self::Double),
            _ => Err(std::str::from_utf8(x).unwrap_or("").to_string()),
        }
    }
}

//------------------------------------------------------------------------------

pub enum MeshOrPoints {
    Mesh,
    Points,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum Xyz {
    X,
    Y,
    Z,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum VertexOrder {
    Xyz,
    Xzy,
    Yxz,
    Yzx,
    Zxy,
    Zyx,
}

impl TryFrom<[Xyz; 3]> for VertexOrder {
    type Error = IOError;

    fn try_from(x: [Xyz; 3]) -> IOResult<Self> {
        match x {
            [Xyz::X, Xyz::Y, Xyz::Z] => Ok(Self::Xyz),
            [Xyz::X, Xyz::Z, Xyz::Y] => Ok(Self::Xzy),
            [Xyz::Y, Xyz::X, Xyz::Z] => Ok(Self::Yxz),
            [Xyz::Y, Xyz::Z, Xyz::X] => Ok(Self::Yzx),
            [Xyz::Z, Xyz::X, Xyz::Y] => Ok(Self::Zxy),
            [Xyz::Z, Xyz::Y, Xyz::X] => Ok(Self::Zyx),
            _ => Err(IOError::InvalidPlyVertexDimensionDefinition),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Default, Clone, Debug)]
pub struct BytesWords {
    pub bytes: usize,
    pub words: usize,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum VertexType {
    Float,
    Double,
}

impl TryFrom<Type> for VertexType {
    type Error = Type;

    fn try_from(x: Type) -> std::result::Result<Self, Self::Error> {
        match x {
            Type::Float => Ok(Self::Float),
            Type::Double => Ok(Self::Double),
            t => Err(t),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum FaceType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
}

impl TryFrom<Type> for FaceType {
    type Error = Type;

    fn try_from(x: Type) -> std::result::Result<Self, Self::Error> {
        match x {
            Type::Char => Ok(Self::Char),
            Type::UChar => Ok(Self::UChar),
            Type::Short => Ok(Self::Short),
            Type::UShort => Ok(Self::UShort),
            Type::Int => Ok(Self::Int),
            Type::UInt => Ok(Self::UInt),
            t => Err(t),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VertexFormat {
    pub order: VertexOrder,
    pub first: VertexType,
    pub snd: VertexType,
    pub third: VertexType,
    pub before: BytesWords,
    pub between_first_snd: BytesWords,
    pub between_snd_third: BytesWords,
    pub after: BytesWords,
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FaceFormat {
    pub count: FaceType,
    pub index: FaceType,
    pub before: BytesWords,
    pub after: BytesWords,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum Format {
    Ascii,
    LittleEndian,
    BigEndian,
}

//------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum HeaderReadState {
    Meta,
    Vertex,
    Face,
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VertexData {
    pub count: usize,
    pub format: VertexFormat,
}

#[derive(Debug, Clone)]
pub struct FaceData {
    pub count: usize,
    pub format: FaceFormat,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Header {
    Full(FullHeader),
    Partial(PartialHeader),
}

#[derive(Debug, Clone)]
pub struct FullHeader {
    pub format: Format,
    pub vertex: VertexData,
    pub face: FaceData,
}

#[derive(Debug, Clone)]
pub struct PartialHeader {
    pub format: Format,
    pub vertex: VertexData,
}

impl Into<PartialHeader> for FullHeader {
    fn into(self) -> PartialHeader {
        PartialHeader {
            format: self.format,
            vertex: self.vertex,
        }
    }
}

impl Into<PartialHeader> for Header {
    fn into(self) -> PartialHeader {
        match self {
            Self::Full(x) => x.into(),
            Self::Partial(x) => x,
        }
    }
}
