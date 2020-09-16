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

//! Module for types used for IO actions

use std::{
    fmt::{Display, Formatter},
    result::Result,
};

use super::from_bytes::FromBytesError;

use super::ply::Type;

//------------------------------------------------------------------------------

/// Trait for adding line information to (error) types
pub trait LineInfoResult<T, E>: Sized {
    fn simple(self) -> Result<T, WithLineInfo<E>>;
    fn index(self, i: usize) -> Result<T, WithLineInfo<E>>;
    fn line(self, i: usize, line: &[u8]) -> Result<T, WithLineInfo<E>>;
}

//------------------------------------------------------------------------------

pub enum DataReserve<T> {
    Data(T),
    Reserve(usize),
}
//@todo implement From<T> and use

//------------------------------------------------------------------------------

pub enum FaceDataReserve<T> {
    Face([usize; 3]), //@todo VId or usize?
    Data(T),
    ReserveDataFaces(usize, usize),
}

impl<T> From<DataReserve<T>> for FaceDataReserve<T> {
    fn from(x: DataReserve<T>) -> Self {
        match x {
            DataReserve::Data(x) => Self::Data(x),
            DataReserve::Reserve(n_d) => Self::ReserveDataFaces(n_d, 0),
        }
    }
}

//------------------------------------------------------------------------------

/// Wrapper type with additional line information
#[derive(Debug)]
pub enum WithLineInfo<T> {
    None(T),
    Index(usize, T),
    Line(usize, String, T),
}

//------------------------------------------------------------------------------

impl<T> Display for WithLineInfo<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::None(x) => write!(f, "{}", x),
            Self::Index(i, x) => write!(f, "Line #{}: '{}'", i, x),
            Self::Line(i, l, x) => write!(f, "Line #{}: '{}' '{}'", i, x, l),
        }
    }
}

//------------------------------------------------------------------------------

//@todo consider split into load/save
pub enum IOError {
    AccessFile,
    Header,
    UnsupportedVersion,
    UnknownPointFormat,
    BinaryData,
    VertexCount,
    FaceCount,
    FaceVertexCount,
    InvalidMeshIndices,
    ColorArrayLength,
    InvalidPlyType(String),
    InvalidPlyVertexType(Type),
    InvalidPlyFaceType(Type),
    InvalidPlyVertexDimensionDefinition,
    Vertex(Option<usize>),
    Face(Option<usize>),
    Property(usize),
    MissingStart(usize),
    LineParse(usize),
    InvalidProperty(usize),
    UnkownFormat(usize),
}

pub type IOResult2<T> = Result<T, IOError>; //@todo rename

impl From<std::io::Error> for IOError {
    fn from(_error: std::io::Error) -> Self {
        IOError::AccessFile
    }
}

impl From<std::array::TryFromSliceError> for IOError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        Self::BinaryData
    }
}

impl From<FromBytesError> for IOError {
    fn from(_error: FromBytesError) -> Self {
        Self::BinaryData
    }
}

impl std::fmt::Debug for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::BinaryData => write!(f, "Unable to parse binary data"),
            Self::UnknownPointFormat => write!(f, "Unknown point format"),
            Self::UnsupportedVersion => write!(f, "Unsupported version"),
            Self::Header => write!(f, "Could not parse header"),
            Self::InvalidProperty(x) => write!(f, "Invalid property on line {}", x),
            Self::MissingStart(x) => write!(f, "Start not found on line {}", x),
            Self::UnkownFormat(x) => write!(f, "Unknown format on line {}", x),
            Self::Vertex(Some(x)) => write!(f, "Unable to parse vertex on line {}", x),
            Self::Vertex(None) => write!(f, "Unable to parse vertex"),
            Self::Face(Some(x)) => write!(f, "Unable to parse face on line {}", x),
            Self::Face(None) => write!(f, "Unable to parse face"),
            Self::Property(x) => write!(f, "Unable to parse property on line {}", x),
            Self::VertexCount => write!(f, "Vertex count does not match"),
            Self::ColorArrayLength => write!(f, "Length of color array does not match others"),
            Self::InvalidPlyType(x) => write!(f, "Invalid type in header '{}'", x),
            Self::InvalidPlyVertexType(x) => write!(f, "Invalid vertex type in header {}", x),
            Self::InvalidPlyFaceType(x) => write!(f, "Invalid face type in header {}", x),
            Self::InvalidMeshIndices => write!(f, "File contains invalid mesh indices"),
            Self::InvalidPlyVertexDimensionDefinition => {
                write!(f, "Invalid order / definition of vertex dimension order")
            }
            Self::FaceCount => write!(f, "Unable to parse face count"),
            Self::FaceVertexCount => write!(f, "Unable to parse vertex count of face"),
        }
    }
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

//------------------------------------------------------------------------------

/// Result type for errors with additional line information
pub type IOResult<T, E> = Result<T, WithLineInfo<E>>;

impl<T, E> LineInfoResult<T, E> for Result<T, E> {
    #[inline(always)]
    fn simple(self) -> Result<T, WithLineInfo<E>> {
        self.map_err(|e| WithLineInfo::None(e))
    }
    #[inline(always)]
    fn index(self, i: usize) -> Result<T, WithLineInfo<E>> {
        self.map_err(|e| WithLineInfo::Index(i, e))
    }
    #[inline(always)]
    fn line(self, i: usize, line: &[u8]) -> Result<T, WithLineInfo<E>> {
        self.map_err(|e| WithLineInfo::Line(i, String::from_utf8_lossy(line).to_string(), e))
    }
}
