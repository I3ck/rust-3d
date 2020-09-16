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

use std::result::Result;

use super::from_bytes::FromBytesError;

use super::ply::Type;

//------------------------------------------------------------------------------

pub enum DataReserve<T> {
    Data(T),
    Reserve(usize),
}

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

//@todo consider split into load/save
pub enum IOError {
    AccessFile,
    Header,
    UnsupportedVersion,
    UnknownPointFormat,
    BinaryData,
    VertexCount(Option<usize>),
    FaceCount(Option<usize>),
    FaceVertexCount,
    InvalidMeshIndices,
    ColorArrayLength,
    InvalidPlyType(String, usize),
    InvalidPlyVertexType(Type, usize),
    InvalidPlyFaceType(Type, usize),
    InvalidPlyVertexDimensionDefinition,
    Vertex(Option<usize>),
    Face(Option<usize>),
    Property(usize),
    MissingStart(usize),
    LineParse(usize),
    InvalidProperty(usize),
    UnkownFormat(usize),
    EndReached,
    Columns(usize),
    Rows(usize),
    Matrix(usize),
    Loop(usize),
    EndLoop(usize),
    EstimateDelimiter,
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
            Self::VertexCount(None) => write!(f, "Vertex count does not match"),
            Self::VertexCount(Some(x)) => write!(f, "Vertex count does not match on line {}", x),
            Self::ColorArrayLength => write!(f, "Length of color array does not match others"),
            Self::InvalidPlyType(s, x) => write!(f, "Invalid type '{}' in header '{}'", s, x),
            Self::InvalidPlyVertexType(t, x) => {
                write!(f, "Invalid vertex type '{}' in header {}", t, x)
            }
            Self::InvalidPlyFaceType(t, x) => {
                write!(f, "Invalid face type '{}' in header {}", t, x)
            }
            Self::InvalidMeshIndices => write!(f, "File contains invalid mesh indices"),
            Self::InvalidPlyVertexDimensionDefinition => {
                write!(f, "Invalid order / definition of vertex dimension order")
            }
            Self::FaceCount(None) => write!(f, "Unable to parse face count"),
            Self::FaceCount(Some(x)) => write!(f, "Unable to parse face count on line {}", x),
            Self::FaceVertexCount => write!(f, "Unable to parse vertex count of face"),
            Self::EndReached => write!(f, "Unexpected reach of file end"),
            Self::Columns(x) => write!(f, "Columns could not be parsed on line {}", x),
            Self::Rows(x) => write!(f, "Rows could not be parsed on line {}", x),
            Self::Matrix(x) => write!(f, "Transformation matrix could not be parsed on line {}", x),
            Self::Loop(x) => write!(f, "Unable to parse loop on line {}", x),
            Self::EndLoop(x) => write!(f, "Unable to parse endloop on line {}", x),
            Self::EstimateDelimiter => write!(f, "Unable to estimate delimiter"),
        }
    }
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<super::utils::FetchLineError> for IOError {
    fn from(_error: super::utils::FetchLineError) -> Self {
        IOError::EndReached
    }
}
