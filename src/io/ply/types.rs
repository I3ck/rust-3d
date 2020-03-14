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

use crate::*;

use core::convert::TryFrom;

//------------------------------------------------------------------------------

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

impl TryFrom<&str> for Type {
    type Error = PlyError;

    fn try_from(x: &str) -> PlyResult<Self> {
        match x {
            "char" => Ok(Self::Char),
            "uchar" => Ok(Self::UChar),
            "short" => Ok(Self::Short),
            "ushort" => Ok(Self::UShort),
            "int" => Ok(Self::Int),
            "uint" => Ok(Self::UInt),
            "float" | "float32" => Ok(Self::Float),
            "double" | "float64" => Ok(Self::Double),
            _ => Err(PlyError::InvalidType(x.to_string())),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Xyz {
    X,
    Y,
    Z,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum VertexOrder {
    Xyz,
    Xzy,
    Yxz,
    Yzx,
    Zxy,
    Zyx,
}

impl TryFrom<[Xyz; 3]> for VertexOrder {
    type Error = PlyError;

    fn try_from(x: [Xyz; 3]) -> PlyResult<Self> {
        match x {
            [Xyz::X, Xyz::Y, Xyz::Z] => Ok(Self::Xyz),
            [Xyz::X, Xyz::Z, Xyz::Y] => Ok(Self::Xzy),
            [Xyz::Y, Xyz::X, Xyz::Z] => Ok(Self::Yxz),
            [Xyz::Y, Xyz::Z, Xyz::X] => Ok(Self::Yzx),
            [Xyz::Z, Xyz::X, Xyz::Y] => Ok(Self::Zxy),
            [Xyz::Z, Xyz::Y, Xyz::X] => Ok(Self::Zyx),
            _ => Err(PlyError::InvalidVertexDimensionDefinition),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Default, Debug)]
pub struct BytesWords {
    pub bytes: usize,
    pub words: usize,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum VertexType {
    Float,
    Double,
}

impl TryFrom<Type> for VertexType {
    type Error = PlyError;

    fn try_from(x: Type) -> PlyResult<Self> {
        match x {
            Type::Float => Ok(Self::Float),
            Type::Double => Ok(Self::Double),
            _ => Err(PlyError::InvalidVertexType),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum FaceType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
}

impl TryFrom<Type> for FaceType {
    type Error = PlyError;

    fn try_from(x: Type) -> PlyResult<Self> {
        match x {
            Type::Char => Ok(Self::Char),
            Type::UChar => Ok(Self::UChar),
            Type::Short => Ok(Self::Short),
            Type::UShort => Ok(Self::UShort),
            Type::Int => Ok(Self::Int),
            Type::UInt => Ok(Self::UInt),
            _ => Err(PlyError::InvalidFaceType),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FaceFormat {
    pub count: FaceType,
    pub index: FaceType,
    pub before: BytesWords,
    pub after: BytesWords,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum Format {
    Ascii,
    LittleEndian,
    BigEndian,
}

//------------------------------------------------------------------------------

pub enum HeaderReadState {
    Meta,
    Vertex,
    Face,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Header {
    pub format: Format,
    pub n_vertices: usize,
    pub n_faces: usize,
    pub vertex_format: VertexFormat,
    pub face_format: FaceFormat,
}
