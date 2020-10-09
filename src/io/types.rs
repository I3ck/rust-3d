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
    ReserveExact(usize),
}

//------------------------------------------------------------------------------

pub enum FaceDataReserve<T> {
    Face([usize; 3]), //@todo VId or usize?
    Data(T),
    ReserveDataFaces(usize, usize),
    ReserveDataFacesExact(usize, usize),
}

impl<T> From<DataReserve<T>> for FaceDataReserve<T> {
    fn from(x: DataReserve<T>) -> Self {
        match x {
            DataReserve::Data(x) => Self::Data(x),
            DataReserve::Reserve(n_d) => Self::ReserveDataFaces(n_d, 0),
            DataReserve::ReserveExact(n_d) => Self::ReserveDataFacesExact(n_d, 0),
        }
    }
}

//------------------------------------------------------------------------------

//@todo consider split into load/save
#[derive(Clone)]
pub enum IOError {
    AccessFile,
    Header,
    UnsupportedVersion,
    UnknownPointFormat,
    BinaryData,
    Threading,
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
    InvalidJSON,
    EstimateDelimiter,
    Gltf(GltfError),
}

#[derive(Clone)]
pub enum GltfError {
    Header,
    Version,
    JSONChunk,
    BinChunk,
    JSONPrimitives,
    JSONAttributes,
    JSONPosition,
    JSONIndices,
    PrimitiveMode4Only,
    ComponentType,
    IndexComponentType,
    PosComponentType,
    AccessorType,
    IndexAccessorType,
    PosAccessorType,
    JSONBufferView,
    JSONComponentType,
    JSONAccessorType,
    JSONCount,
    JSONBuffer,
    JSONBufferLength,
    JSONByteLength,
    JSONNodes,
    JSONAccessors,
    JSONBufferViews,
    JSONBuffers,
    JSONMeshes,
    JSONMesh,
    Stride,
    BufferUriNotSupported,
    BufferUriAccess,
    Base64Decode,
}

pub type IOResult<T> = Result<T, IOError>; //@todo rename

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
            Self::InvalidJSON => write!(f, "Unable to parse JSON format"),
            Self::Threading => write!(f, "Error during multi threading"),
            Self::Gltf(x) => write!(f, "{:?}", x),
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

impl From<serde_json::error::Error> for IOError {
    fn from(_error: serde_json::error::Error) -> Self {
        //@todo use more information
        IOError::InvalidJSON
    }
}

//------------------------------------------------------------------------------

impl std::fmt::Debug for GltfError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Header => write!(f, "Invalid header of .glTF/.glb file"),
            Self::Version => write!(f, "Version of .glTF/.glb file not supported"),
            Self::JSONChunk => write!(f, "JSON chunk of .glTF/.glb file is invalid"),
            Self::BinChunk => write!(f, "Binary chunk of .glTF/.glb file is invalid"),
            Self::JSONPrimitives => {
                write!(f, "JSON primitives of .glTF/.glb file could not be parsed")
            }
            Self::JSONAttributes => {
                write!(f, "JSON attributes of .glTF/.glb file could not be parsed")
            }
            Self::JSONPosition => {
                write!(f, "JSON positions of .glTF/.glb file could not be parsed")
            }
            Self::JSONIndices => write!(f, "JSON indices of .glTF/.glb file could not be parsed"),
            Self::PrimitiveMode4Only => write!(
                f,
                "Only supporting primitive mode 4 of shapes (triangles) in .glTF/.glb"
            ),
            Self::ComponentType => write!(f, "Invalid component type in .glTF/.glb"),
            Self::IndexComponentType => write!(f, "Invalid index component type in .glTF/.glb"),
            Self::PosComponentType => write!(f, "Invalid position component type in .glTF/.glb"),
            Self::AccessorType => write!(f, "Invalid accessor type in .glTF/.glb"),
            Self::IndexAccessorType => write!(f, "Invalid index accessor type in .glTF/.glb"),
            Self::PosAccessorType => write!(f, "Invalid position accessor type in .glTF/.glb"),
            Self::JSONBufferView => {
                write!(f, "JSON bufferView of .glTF/.glb file could not be parsed")
            }
            Self::JSONComponentType => write!(
                f,
                "JSON componentType of .glTF/.glb file could not be parsed"
            ),
            Self::JSONAccessorType => write!(
                f,
                "JSON accessor type of .glTF/.glb file could not be parsed"
            ),
            Self::JSONCount => write!(f, "JSON count of .glTF/.glb file could not be parsed"),
            Self::JSONBuffer => write!(f, "JSON buffer of .glTF/.glb file could not be parsed"),
            Self::JSONByteLength => {
                write!(f, "JSON byteLength of .glTF/.glb file could not be parsed")
            }
            Self::JSONBufferLength => write!(
                f,
                "JSON length of buffer of .glTF/.glb file could not be parsed"
            ),
            Self::JSONNodes => write!(f, "JSON nodes of .glTF/.glb file could not be parsed"),
            Self::JSONAccessors => {
                write!(f, "JSON accessors of .glTF/.glb file could not be parsed")
            }
            Self::JSONBufferViews => {
                write!(f, "JSON bufferViews of .glTF/.glb file could not be parsed")
            }
            Self::JSONBuffers => write!(f, "JSON buffers of .glTF/.glb file could not be parsed"),
            Self::JSONMeshes => write!(f, "JSON meshes of .glTF/.glb file could not be parsed"),
            Self::JSONMesh => write!(f, "JSON mesh of .glTF/.glb file could not be parsed"),
            Self::Stride => write!(f, "Invalid stride value in .glTF/.glb file"),
            Self::BufferUriNotSupported => {
                write!(f, "Buffers with uri value aren't currently supported")
            }
            Self::BufferUriAccess => write!(f, "Could not access buffer file defined via uri"),
            Self::Base64Decode => write!(f, "Unable to decode base64 data in .glTF/.glb file"),
        }
    }
}

impl std::fmt::Display for GltfError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
