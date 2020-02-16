/*
Copyright 2016 Martin Buck

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

//! Result, the result type used within rust-3d. Also defining the error enum and several transformation methods between error types.

use std::{
    fmt,
    io::Error as ioError,
    num::{ParseFloatError, ParseIntError},
    result,
};

/// The Error Enum used by rust-3d
pub enum ErrorKind {
    MinMaxSwapped,
    MinMaxEqual,
    TooFewPoints,
    BoundingBoxMissing,
    NormalizeVecWithoutLength,
    IOError,
    ParseError,
    IndexOutOfBounds,
    IncorrectFaceID,
    IncorrectVertexID,
    IncorrectEdgeID,
    IncorrectVoxelID,
    IncorrectUnitID,
    IncorrectSegmentID,
    IncorrectDimension,
    DimensionsDontMatch,
    NumberConversionError,
    NumberInWrongRange,
    ComparisionFailed,
    ColorArrayIncorrectLength,
    ClusterTooBig,
    CantCalculateAngleIfZeroLength,
    TriFace3DNotSpanningVolume,
    PlyError(PlyError),
    StlError(StlError),
    XyError(XyError),
    XyzError(XyzError),
}

pub enum PlyError {
    LoadError,
    LoadStartNotFound,
    LoadFormatNotFound,
    LoadWrongPropertyCount,
    LoadVertexIndexDefinitionNotFound,
    LoadHeaderEndNotFound,
    LoadVertexCountNotFound,
    LoadFaceCountNotFound,
    LoadVertexCountIncorrect,
    LoadVerticesIncorrect,
    IncorrectFaceData,
}

pub enum StlError {
    LoadFileEndReached,
    LoadFileInvalid, //@todo specify better
}

pub enum XyError {
    LoadFileInvalid, //@todo specify better
}

pub enum XyzError {
    LoadFileInvalid, //@todo specify better
}

impl ErrorKind {
    /// Returns readable text for the ErrorKind
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MinMaxSwapped => "Passed min/max values are swapped (min > max)",
            Self::MinMaxEqual => "Passed min/max values are equal",
            Self::TooFewPoints => "Container had too few points for the operation",
            Self::BoundingBoxMissing => "Bounding box is missing for the operation",
            Self::NormalizeVecWithoutLength => "Can't normalize a vector of length 0",
            Self::IOError => "Can't read or write a file",
            Self::ParseError => "Can't parse data",
            Self::IndexOutOfBounds => "Tried to access an out of bounds index",
            Self::IncorrectFaceID => "Used an incorrect face id",
            Self::IncorrectVertexID => "Used an incorrect vertex id",
            Self::IncorrectEdgeID => "Used an incorrect edge id",
            Self::IncorrectVoxelID => "Used an incorrect voxel id",
            Self::IncorrectUnitID => "Used an incorrect unit id",
            Self::IncorrectSegmentID => "Used an incorrect segment id",
            Self::IncorrectDimension => "Trying to access an incorrect dimension",
            Self::DimensionsDontMatch => "Trying to mix types with different dimensions",
            Self::NumberConversionError => "Failed converting one number type to another",
            Self::NumberInWrongRange => "Passed number is within the wrong range",
            Self::ComparisionFailed => "Comparision between two values failed",
            Self::ColorArrayIncorrectLength => "The provided color array has an incorrect length",
            Self::CantCalculateAngleIfZeroLength => "Can't calculate the angle between 0 vectors",
            Self::ClusterTooBig => "Clustering size is too big for given mesh",
            Self::TriFace3DNotSpanningVolume => {
                "TriFace3D must be constructed from points spanning a volume"
            }
            Self::PlyError(x) => x.as_str(),
            Self::StlError(x) => x.as_str(),
            Self::XyError(x) => x.as_str(),
            Self::XyzError(x) => x.as_str(),
        }
    }
}

impl PlyError {
    /// Returns readable text for the PlyError
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoadError => "Error while loading .ply",
            Self::LoadStartNotFound => "Start of .ply header not found",
            Self::LoadFormatNotFound => "Format of .ply missing or not supported",
            Self::LoadWrongPropertyCount => "Property count of .ply missing or not supported",
            Self::LoadVertexIndexDefinitionNotFound => "Index definition in .ply not found",
            Self::LoadHeaderEndNotFound => "End of header definition of .ply not found",
            Self::LoadVertexCountNotFound => "Vertex count of .ply not found",
            Self::LoadFaceCountNotFound => "Face count of .ply not found",
            Self::LoadVertexCountIncorrect => "Vertex count of .ply not found",
            Self::LoadVerticesIncorrect => "Vertices in .ply incorrect",
            Self::IncorrectFaceData => "Face definition is incorrect / can not be parsed",
        }
    }
}

impl StlError {
    /// Returns readable text for the StlError
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoadFileEndReached => "Unexpected reach of .stl file end",
            Self::LoadFileInvalid => "Invalid .stl file",
        }
    }
}

impl XyError {
    /// Returns readable text for the XyError
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoadFileInvalid => "Invalid .xy file",
        }
    }
}

impl XyzError {
    /// Returns readable text for the XyzError
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LoadFileInvalid => "Invalid .xyz file",
        }
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Result type used by rust-3d
pub type Result<T> = result::Result<T, ErrorKind>;

/// Trait used to convert other Errors to ErrorKind
pub trait ToErrorKind {
    /// Creates an ErrorKind from this
    fn to_error_kind(&self) -> ErrorKind;
}

impl ToErrorKind for ParseFloatError {
    fn to_error_kind(&self) -> ErrorKind {
        ErrorKind::ParseError
    }
}

impl ToErrorKind for ParseIntError {
    fn to_error_kind(&self) -> ErrorKind {
        ErrorKind::ParseError
    }
}

impl ToErrorKind for ioError {
    fn to_error_kind(&self) -> ErrorKind {
        ErrorKind::IOError
    }
}

impl From<ioError> for ErrorKind {
    fn from(_error: ioError) -> Self {
        ErrorKind::IOError
    }
}
