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
    ClusterTooBig,
    CantCalculateAngleIfZeroLength,
    TriFace3DNotSpanningVolume,
    PlyError(PlyError),
    StlError(StlError),
    PtxError(PtxError),
    XyError(XyError),
    XyzError(XyzError),
    ObjError(ObjError),
    OffError(OffError),
    PslError(PslError),
}

pub enum PlyError {
    LoadStartNotFound,
    LoadFormatNotFound,
    LoadWrongPropertyCount,
    LoadVertexIndexDefinitionNotFound,
    LoadHeaderInvalid,
    LoadVertexCountNotFound,
    LoadFaceCountNotFound,
    LoadVertexCountIncorrect,
    AccessFile,
    ColorArrayIncorrectLength,
    //@todo these all could name the affected line
    InvalidType(String),
    InvalidVertexType, //@todo would be better to name the issue
    InvalidFaceType,   //@todo would be better to name the issue
    InvalidMeshIndices(Option<usize>),
    LineParse(usize),
    InvalidProperty(usize),
    InvalidVertex(usize),
    PropertyLineLocation(usize),
    FaceStructure,
    InvalidVertexDimensionDefinition,
}

pub enum StlError {
    LoadFileEndReached,
    AccessFile,
    LineParse(usize),
}

pub enum PtxError {
    LoadFileEndReached,
    AccessFile,
    LineParse(usize),
    Columns(usize),
    Rows(usize),
    Matrix(usize),
    Point(usize),
}

pub enum XyError {
    EstimateDelimiter,
    AccessFile,
    LineParse(usize),
}

pub enum XyzError {
    EstimateDelimiter,
    AccessFile,
    LineParse(usize),
}

pub enum ObjError {
    AccessFile,
    InvalidMeshIndices(usize),
    LineParse(usize),
}

pub enum OffError {
    AccessFile,
    InvalidMeshIndices(usize),
    LineParse(usize),
}

pub enum PslError {
    AccessFile,
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MinMaxSwapped => write!(f, "Passed min/max values are swapped (min > max)"),
            Self::MinMaxEqual => write!(f, "Passed min/max values are equal"),
            Self::TooFewPoints => write!(f, "Container had too few points for the operation"),
            Self::BoundingBoxMissing => write!(f, "Bounding box is missing for the operation"),
            Self::NormalizeVecWithoutLength => write!(f, "Can't normalize a vector of length 0"),
            Self::IOError => write!(f, "Can't read or write a file"),
            Self::ParseError => write!(f, "Can't parse data"),
            Self::IndexOutOfBounds => write!(f, "Tried to access an out of bounds index"),
            Self::IncorrectFaceID => write!(f, "Used an incorrect face id"),
            Self::IncorrectVertexID => write!(f, "Used an incorrect vertex id"),
            Self::IncorrectEdgeID => write!(f, "Used an incorrect edge id"),
            Self::IncorrectVoxelID => write!(f, "Used an incorrect voxel id"),
            Self::IncorrectUnitID => write!(f, "Used an incorrect unit id"),
            Self::IncorrectSegmentID => write!(f, "Used an incorrect segment id"),
            Self::IncorrectDimension => write!(f, "Trying to access an incorrect dimension"),
            Self::DimensionsDontMatch => write!(f, "Trying to mix types with different dimensions"),
            Self::NumberConversionError => {
                write!(f, "Failed converting one number type to another")
            }
            Self::NumberInWrongRange => write!(f, "Passed number is within the wrong range"),
            Self::ComparisionFailed => write!(f, "Comparision between two values failed"),
            Self::CantCalculateAngleIfZeroLength => {
                write!(f, "Can't calculate the angle between 0 vectors")
            }
            Self::ClusterTooBig => write!(f, "Clustering size is too big for given mesh"),
            Self::TriFace3DNotSpanningVolume => write!(
                f,
                "TriFace3D must be constructed from points spanning a volume"
            ),
            Self::PlyError(x) => x.fmt(f),
            Self::StlError(x) => x.fmt(f),
            Self::PtxError(x) => x.fmt(f),
            Self::XyError(x) => x.fmt(f),
            Self::XyzError(x) => x.fmt(f),
            Self::ObjError(x) => x.fmt(f),
            Self::OffError(x) => x.fmt(f),
            Self::PslError(x) => x.fmt(f),
        }
    }
}

impl fmt::Debug for PlyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadStartNotFound => write!(f, "Start of .ply header not found"),
            Self::LoadFormatNotFound => write!(f, "Format of .ply missing or not supported"),
            Self::LoadWrongPropertyCount => {
                write!(f, "Property count of .ply missing or not supported")
            }
            Self::LoadVertexIndexDefinitionNotFound => {
                write!(f, "Index definition in .ply not found")
            }
            Self::LoadHeaderInvalid => write!(f, "Header of .ply seems to be invalid"),
            Self::LoadVertexCountNotFound => write!(f, "Vertex count of .ply not found"),
            Self::LoadFaceCountNotFound => write!(f, "Face count of .ply not found"),
            Self::LoadVertexCountIncorrect => write!(f, "Vertex count of .ply not found"),
            Self::ColorArrayIncorrectLength => {
                write!(f, "The provided color array has an incorrect length")
            }
            Self::InvalidType(x) => write!(f, "Invalid type in header '{}'", x),
            Self::InvalidVertexType => write!(f, "Invalid vertex type in header"),
            Self::InvalidFaceType => write!(f, "Invalid face type in header"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::InvalidMeshIndices(opt_x) => match opt_x {
                Some(x) => write!(f, "File contains invalid mesh indices on line {}", x),
                None => write!(f, "File contains invalid mesh indices"),
            },
            Self::InvalidProperty(x) => write!(f, "Invalid property on line {}", x),
            Self::InvalidVertex(x) => write!(f, "Invalid vertex definition on line {}", x),
            Self::InvalidVertexDimensionDefinition => {
                write!(f, "Invalid order / definition of vertex dimension order")
            }
            Self::PropertyLineLocation(x) => write!(
                f,
                "Found property line at unexpected location on line {}",
                x
            ),
            Self::FaceStructure => write!(
                f,
                "Invalid face structure, only supporting 3 vertices per face"
            ),
        }
    }
}

impl fmt::Debug for StlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadFileEndReached => write!(f, "Unexpected reach of .stl file end"),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
        }
    }
}

impl fmt::Debug for PtxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadFileEndReached => write!(f, "Unexpected reach of .ptx file end"),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::Columns(x) => write!(f, "Columns could not be parsed on line {}", x),
            Self::Rows(x) => write!(f, "Rows could not be parsed on line {}", x),
            Self::Matrix(x) => write!(f, "Transformation matrix could not be parsed on line {}", x),
            Self::Point(x) => write!(f, "Point could not be parsed on line {}", x),
        }
    }
}

impl fmt::Debug for XyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::EstimateDelimiter => write!(f, "Unable to estimate delimiter"),
        }
    }
}

impl fmt::Debug for XyzError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::EstimateDelimiter => write!(f, "Unable to estimate delimiter"),
        }
    }
}

impl fmt::Debug for ObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::InvalidMeshIndices(x) => {
                write!(f, "File contains invalid mesh indices on line {}", x)
            }
        }
    }
}

impl fmt::Debug for OffError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::InvalidMeshIndices(x) => {
                write!(f, "File contains invalid mesh indices on line {}", x)
            }
        }
    }
}

impl fmt::Debug for PslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
        }
    }
}

/// Result type used by rust-3d
pub type Result<T> = result::Result<T, ErrorKind>;

/// Result for PlyError
pub type PlyResult<T> = result::Result<T, PlyError>;

/// Result for StlError
pub type StlResult<T> = result::Result<T, StlError>;

/// Result for PtxError
pub type PtxResult<T> = result::Result<T, PtxError>;

/// Result for XyError
pub type XyResult<T> = result::Result<T, XyError>;

/// Result for XyzError
pub type XyzResult<T> = result::Result<T, XyzError>;

/// Result for ObjError
pub type ObjResult<T> = result::Result<T, ObjError>;

/// Result for OffError
pub type OffResult<T> = result::Result<T, OffError>;

/// Result for PslError
pub type PslResult<T> = result::Result<T, PslError>;

impl From<ParseFloatError> for ErrorKind {
    fn from(_error: ParseFloatError) -> Self {
        ErrorKind::ParseError
    }
}

impl From<ParseIntError> for ErrorKind {
    fn from(_error: ParseIntError) -> ErrorKind {
        ErrorKind::ParseError
    }
}

impl From<ioError> for ErrorKind {
    fn from(_error: ioError) -> Self {
        ErrorKind::IOError
    }
}

impl From<PlyError> for ErrorKind {
    fn from(error: PlyError) -> Self {
        Self::PlyError(error)
    }
}

impl From<StlError> for ErrorKind {
    fn from(error: StlError) -> Self {
        Self::StlError(error)
    }
}

impl From<XyError> for ErrorKind {
    fn from(error: XyError) -> Self {
        Self::XyError(error)
    }
}

impl From<XyzError> for ErrorKind {
    fn from(error: XyzError) -> Self {
        Self::XyzError(error)
    }
}

impl From<ObjError> for ErrorKind {
    fn from(error: ObjError) -> Self {
        Self::ObjError(error)
    }
}

impl From<OffError> for ErrorKind {
    fn from(error: OffError) -> Self {
        Self::OffError(error)
    }
}

impl From<ioError> for PlyError {
    fn from(_error: ioError) -> Self {
        PlyError::AccessFile
    }
}

impl From<ioError> for StlError {
    fn from(_error: ioError) -> Self {
        StlError::AccessFile
    }
}

impl From<ioError> for PtxError {
    fn from(_error: ioError) -> Self {
        PtxError::AccessFile
    }
}

impl From<ioError> for XyError {
    fn from(_error: ioError) -> Self {
        XyError::AccessFile
    }
}

impl From<ioError> for XyzError {
    fn from(_error: ioError) -> Self {
        XyzError::AccessFile
    }
}

impl From<ioError> for ObjError {
    fn from(_error: ioError) -> Self {
        ObjError::AccessFile
    }
}

impl From<ioError> for OffError {
    fn from(_error: ioError) -> Self {
        OffError::AccessFile
    }
}

impl From<ioError> for PslError {
    fn from(_error: ioError) -> Self {
        PslError::AccessFile
    }
}
