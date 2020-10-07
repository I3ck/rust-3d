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

//! Module for types for the .glTF file formats

//------------------------------------------------------------------------------

use std::{collections::HashSet, convert::TryFrom};

use super::super::types::*;

use crate::*;

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Root {
    pub root_nodes: Vec<Node>,
}

impl Root {
    pub fn new(val: &serde_json::Value) -> IOResult<Self> {
        let nodes = val
            .get("nodes")
            .and_then(|x| x.as_array())
            .ok_or(IOError::Gltf(GltfError::JSONNodes))?;

        let meshes = val
            .get("meshes")
            .and_then(|x| x.as_array())
            .ok_or(IOError::Gltf(GltfError::JSONAccessors))?;

        let accessors = val
            .get("accessors")
            .and_then(|x| x.as_array())
            .ok_or(IOError::Gltf(GltfError::JSONAccessors))?;

        let buffer_views = val
            .get("bufferViews")
            .and_then(|x| x.as_array())
            .ok_or(IOError::Gltf(GltfError::JSONBufferViews))?;

        let buffers = val
            .get("buffers")
            .and_then(|x| x.as_array())
            .ok_or(IOError::Gltf(GltfError::JSONBuffers))?;

        let arrays = JSONArrays {
            nodes,
            meshes,
            accessors,
            buffer_views,
            buffers,
        };

        let mut child_nodes = HashSet::new();
        for node in nodes {
            if let Some(children) = node.get("children").and_then(|x| x.as_array()) {
                for child in children {
                    if let Some(id) = child.as_u64() {
                        child_nodes.insert(id);
                    }
                }
            }
        }

        let mut root_nodes = Vec::new();
        for (id, node) in nodes.iter().enumerate() {
            if !child_nodes.contains(&(id as u64)) {
                if let Some(n) = Node::new(&arrays, node, &None) {
                    root_nodes.push(n)
                }
            }
        }

        Ok(Self { root_nodes })
    }
}

//------------------------------------------------------------------------------

pub struct JSONArrays<'a> {
    pub nodes: &'a Vec<serde_json::Value>,
    pub meshes: &'a Vec<serde_json::Value>,
    pub accessors: &'a Vec<serde_json::Value>,
    pub buffer_views: &'a Vec<serde_json::Value>,
    pub buffers: &'a Vec<serde_json::Value>,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
struct Transformations {
    pub translation: Option<[f64; 3]>,
    pub scale: Option<[f64; 3]>,
    pub rotation: Option<[f64; 4]>,
    pub matrix: Option<[f64; 16]>,
}

impl Transformations {
    pub fn new(val: &serde_json::Value) -> Self {
        let translation = Self::read_translation(val);
        let scale = Self::read_scale(val);
        let rotation = Self::read_rotation(val);
        let matrix = Self::read_matrix(val);

        Self {
            translation,
            scale,
            rotation,
            matrix,
        }
    }

    pub fn transformation(&self) -> Option<Matrix4> {
        if self.matrix.is_none()
            && self.scale.is_none()
            && self.rotation.is_none()
            && self.translation.is_none()
            && self.matrix.is_none()
        {
            None
        } else if let Some(matrix) = self.matrix {
            Some(Matrix4::from(matrix).transposed())
        } else {
            let scale = if let Some(s) = self.scale {
                Matrix4::scale(s[0], s[1], s[2])
            } else {
                Matrix4::identity()
            };
            let rotation = if let Some(r) = self.rotation {
                Matrix4::from_unit_quaternion(&r)
            } else {
                Matrix4::identity()
            };
            let translation = if let Some(t) = self.translation {
                Matrix4::translation(t[0], t[1], t[2])
            } else {
                Matrix4::identity()
            };

            Some(translation * rotation * scale)
        }
    }

    fn read_translation(val: &serde_json::Value) -> Option<[f64; 3]> {
        let arr = val.get("translation").and_then(|x| x.as_array())?;
        if arr.len() != 3 {
            return None;
        }
        Some([arr[0].as_f64()?, arr[1].as_f64()?, arr[2].as_f64()?])
    }

    fn read_scale(val: &serde_json::Value) -> Option<[f64; 3]> {
        let arr = val.get("scale").and_then(|x| x.as_array())?;
        if arr.len() != 3 {
            return None;
        }
        Some([arr[0].as_f64()?, arr[1].as_f64()?, arr[2].as_f64()?])
    }

    fn read_rotation(val: &serde_json::Value) -> Option<[f64; 4]> {
        let arr = val.get("rotation").and_then(|x| x.as_array())?;
        if arr.len() != 4 {
            return None;
        }
        Some([
            arr[0].as_f64()?,
            arr[1].as_f64()?,
            arr[2].as_f64()?,
            arr[3].as_f64()?,
        ])
    }

    fn read_matrix(val: &serde_json::Value) -> Option<[f64; 16]> {
        let arr = val.get("matrix").and_then(|x| x.as_array())?;
        if arr.len() != 16 {
            return None;
        }
        Some([
            arr[0].as_f64()?,
            arr[1].as_f64()?,
            arr[2].as_f64()?,
            arr[3].as_f64()?,
            arr[4].as_f64()?,
            arr[5].as_f64()?,
            arr[6].as_f64()?,
            arr[7].as_f64()?,
            arr[8].as_f64()?,
            arr[9].as_f64()?,
            arr[10].as_f64()?,
            arr[11].as_f64()?,
            arr[12].as_f64()?,
            arr[13].as_f64()?,
            arr[14].as_f64()?,
            arr[15].as_f64()?,
        ])
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub enum MeshOrChildren {
    Mesh(Mesh),
    Children(Vec<Node>),
}

impl MeshOrChildren {
    pub fn mesh(&self) -> Option<&Mesh> {
        match self {
            MeshOrChildren::Mesh(x) => Some(&x),
            MeshOrChildren::Children(_) => None,
        }
    }

    pub fn children(&self) -> Option<&Vec<Node>> {
        match self {
            MeshOrChildren::Mesh(_) => None,
            MeshOrChildren::Children(x) => Some(&x),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Node {
    pub transformation: Option<Matrix4>,
    pub mesh_or_children: MeshOrChildren,
}

impl Node {
    pub fn new(
        arrays: &JSONArrays,
        val: &serde_json::Value,
        parent_transformation: &Option<Matrix4>,
    ) -> Option<Self> {
        let transformations = Transformations::new(val);

        let transformation = match (parent_transformation, transformations.transformation()) {
            (None, None) => None,
            (None, Some(x)) => Some(x),
            (Some(x), None) => Some(x.clone()),
            (Some(parent), Some(ref this)) => Some(parent * this),
        };

        let mut children = Self::read_children(arrays, val, &transformation);
        let mesh = Self::read_mesh(arrays, val);

        // Simplification, just treat the Mesh as another child node
        match (mesh, children.is_empty()) {
            (None, true) => None,
            (None, false) => Some(Self {
                transformation,
                mesh_or_children: MeshOrChildren::Children(children),
            }),
            (Some(m), true) => Some(Self {
                transformation,
                mesh_or_children: MeshOrChildren::Mesh(m),
            }),
            (Some(m), false) => {
                children.push(Self::new_from_mesh(m, transformation.clone()));
                Some(Self {
                    transformation,
                    mesh_or_children: MeshOrChildren::Children(children),
                })
            }
        }
    }

    fn new_from_mesh(mesh: Mesh, transformation: Option<Matrix4>) -> Self {
        Self {
            transformation,
            mesh_or_children: MeshOrChildren::Mesh(mesh),
        }
    }

    //@todo this could cause endless recursion in case the file is malformed
    fn read_children(
        arrays: &JSONArrays,
        val: &serde_json::Value,
        parent_transformation: &Option<Matrix4>,
    ) -> Vec<Node> {
        let mut result = Vec::new();
        if let Some(children) = val.get("children").and_then(|x| x.as_array()) {
            for child in children {
                if let Some(id) = child.as_u64() {
                    if let Some(n) = arrays
                        .nodes
                        .get(id as usize)
                        .and_then(|node_val| Node::new(arrays, node_val, parent_transformation))
                    {
                        result.push(n)
                    }
                }
            }
        }

        result
    }

    fn read_mesh(arrays: &JSONArrays, val: &serde_json::Value) -> Option<Mesh> {
        if let Some(mesh_id) = val.get("mesh").and_then(|x| x.as_u64()) {
            arrays
                .meshes
                .get(mesh_id as usize)
                .and_then(|x| Mesh::new(arrays, x).ok())
        } else {
            None
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn new(arrays: &JSONArrays, val: &serde_json::Value) -> IOResult<Self> {
        let primitives_array = val
            .get("primitives")
            .and_then(|x| x.as_array())
            .ok_or(IOError::Gltf(GltfError::JSONPrimitives))?;

        let mut primitives = Vec::new();
        for primitive_val in primitives_array.iter() {
            // Ignoring invalid primitives
            match Primitive::new(arrays, primitive_val) {
                Ok(x) => primitives.push(x),
                Err(_) => (),
            };
        }

        Ok(Self { primitives })
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Primitive {
    //@todo consider storing normal id as well
    pub positions: PosAccessor,
    pub indices: Option<IndexAccessor>,
}

impl Primitive {
    pub fn new(arrays: &JSONArrays, val: &serde_json::Value) -> IOResult<Self> {
        let mode = val.get("mode").and_then(|x| x.as_u64()).unwrap_or(4);
        if mode == 4 {
            // TRIANGLES
            let attributes = val
                .get("attributes")
                .ok_or(IOError::Gltf(GltfError::JSONAttributes))?;
            let positions_id = attributes
                .get("POSITION")
                .and_then(|x| x.as_u64())
                .ok_or(IOError::Gltf(GltfError::JSONPosition))?;
            let indices_id = val.get("indices").and_then(|x| x.as_u64());
            let positions = Accessor::new(arrays, &arrays.accessors[positions_id as usize])
                .and_then(|x| PosAccessor::new(x))?;
            let indices = indices_id
                .and_then(|x| Accessor::new(arrays, &arrays.accessors[x as usize]).ok())
                .and_then(|x| IndexAccessor::new(x).ok());

            Ok(Self { positions, indices })
        } else {
            Err(IOError::Gltf(GltfError::PrimitiveMode4Only))
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IndexComponentType {
    U8,
    U16,
    U32,
}

impl IndexComponentType {
    pub fn new(ct: ComponentType) -> IOResult<Self> {
        match ct {
            ComponentType::U8 => Ok(Self::U8),
            ComponentType::U16 => Ok(Self::U16),
            ComponentType::U32 => Ok(Self::U32),
            ComponentType::I8 | ComponentType::I16 | ComponentType::F32 => {
                Err(IOError::Gltf(GltfError::IndexComponentType))
            }
        }
    }
}

impl Default for IndexComponentType {
    fn default() -> Self {
        Self::U8
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComponentType {
    I8,
    U8,
    I16,
    U16,
    U32,
    F32,
}

impl ComponentType {
    pub fn new(val: u64) -> IOResult<Self> {
        match val {
            5120 => Ok(Self::I8),
            5121 => Ok(Self::U8),
            5122 => Ok(Self::I16),
            5123 => Ok(Self::U16),
            5125 => Ok(Self::U32),
            5126 => Ok(Self::F32),
            _ => Err(IOError::Gltf(GltfError::ComponentType)),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

impl AccessorType {
    pub fn new(val: &str) -> IOResult<Self> {
        match val {
            "SCALAR" => Ok(Self::Scalar),
            "VEC2" => Ok(Self::Vec2),
            "VEC3" => Ok(Self::Vec3),
            "VEC4" => Ok(Self::Vec4),
            "MAT2" => Ok(Self::Mat2),
            "MAT3" => Ok(Self::Mat3),
            "MAT4" => Ok(Self::Mat4),
            _ => Err(IOError::Gltf(GltfError::AccessorType)),
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Accessor {
    pub buffer_view: BufferView,
    pub byte_offset: u64,
    pub component_type: ComponentType,
    pub count: u64,
    pub accessor_type: AccessorType,
}

impl Accessor {
    pub fn new(arrays: &JSONArrays, val: &serde_json::Value) -> IOResult<Self> {
        let buffer_view_id = val
            .get("bufferView")
            .and_then(|x| x.as_u64())
            .ok_or(IOError::Gltf(GltfError::JSONBufferView))?;
        let byte_offset = val.get("byteOffset").and_then(|x| x.as_u64()).unwrap_or(0);
        let component_type = val
            .get("componentType")
            .and_then(|x| x.as_u64())
            .ok_or(IOError::Gltf(GltfError::JSONComponentType))
            .and_then(|x| ComponentType::new(x))?;
        let count = val
            .get("count")
            .and_then(|x| x.as_u64())
            .ok_or(IOError::Gltf(GltfError::JSONCount))?;
        let accessor_type = val
            .get("type")
            .and_then(|x| x.as_str())
            .ok_or(IOError::Gltf(GltfError::JSONAccessorType))
            .and_then(|x| AccessorType::new(x))?;

        let buffer_view = BufferView::new(arrays, &arrays.buffer_views[buffer_view_id as usize])?;

        Ok(Self {
            buffer_view,
            byte_offset,
            component_type,
            count,
            accessor_type,
        })
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct IndexAccessor {
    pub buffer_view: BufferView,
    pub byte_offset: u64,
    pub component_type: IndexComponentType,
    pub count: u64,
}

impl IndexAccessor {
    pub fn new(accessor: Accessor) -> IOResult<Self> {
        if accessor.accessor_type != AccessorType::Scalar {
            return Err(IOError::Gltf(GltfError::IndexAccessorType));
        }
        let component_type = IndexComponentType::new(accessor.component_type)?;

        Ok(Self {
            buffer_view: accessor.buffer_view,
            byte_offset: accessor.byte_offset,
            component_type,
            count: accessor.count,
        })
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PosAccessor {
    pub buffer_view: BufferView,
    pub byte_offset: u64,
    pub count: u64,
}

impl PosAccessor {
    pub fn new(accessor: Accessor) -> IOResult<Self> {
        if accessor.accessor_type != AccessorType::Vec3 {
            return Err(IOError::Gltf(GltfError::PosAccessorType));
        }
        if accessor.component_type != ComponentType::F32 {
            return Err(IOError::Gltf(GltfError::PosComponentType));
        }

        Ok(Self {
            buffer_view: accessor.buffer_view,
            byte_offset: accessor.byte_offset,
            count: accessor.count,
        })
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BufferView {
    pub buffer: Buffer,
    pub byte_length: u64,
    pub byte_offset: u64,
    pub byte_stride: Option<u64>,
}

impl BufferView {
    pub fn new(arrays: &JSONArrays, val: &serde_json::Value) -> IOResult<Self> {
        let buffer_id = val
            .get("buffer")
            .and_then(|x| x.as_u64())
            .ok_or(IOError::Gltf(GltfError::JSONBuffer))?;
        let byte_length = val
            .get("byteLength")
            .and_then(|x| x.as_u64())
            .ok_or(IOError::Gltf(GltfError::JSONByteLength))?;
        let byte_offset = val.get("byteOffset").and_then(|x| x.as_u64()).unwrap_or(0);
        let byte_stride = val.get("byteStride").and_then(|x| x.as_u64());
        let buffer = Buffer::new(&arrays.buffers[buffer_id as usize])?;

        Ok(Self {
            buffer,
            byte_length,
            byte_offset,
            byte_stride,
        })
    }
}

//------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Buffer {
    pub byte_length: u64,
    pub uri: Option<String>,
}

impl Buffer {
    pub fn new(val: &serde_json::Value) -> IOResult<Self> {
        let byte_length = val
            .get("byteLength")
            .and_then(|x| x.as_u64())
            .ok_or(IOError::Gltf(GltfError::JSONBufferLength))?;
        let uri = val
            .get("uri")
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());
        Ok(Self { byte_length, uri })
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct RawFileHeader {
    pub magic: u32,
    pub version: u32,
    pub length: u32,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct FileHeader {
    pub length: u32,
}

impl TryFrom<RawFileHeader> for FileHeader {
    type Error = IOError;
    fn try_from(x: RawFileHeader) -> IOResult<Self> {
        if x.magic != VALID_MAGIC {
            Err(IOError::Gltf(GltfError::Header))
        } else if x.version != VALID_VERSION {
            Err(IOError::Gltf(GltfError::Version))
        } else {
            Ok(Self { length: x.length })
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ChunkHeader {
    pub length: u32,
    pub ctype: u32,
    pub pos: u64,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Chunk {
    pub header: ChunkHeader,
    pub data: Vec<u8>,
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct JSONChunk {
    pub length: u32,
    pub data: Vec<u8>,
}

impl TryFrom<Chunk> for JSONChunk {
    type Error = IOError;
    fn try_from(x: Chunk) -> IOResult<Self> {
        if x.header.ctype != TYPE_JSON {
            Err(IOError::Gltf(GltfError::JSONChunk))
        } else {
            Ok(Self {
                length: x.header.length,
                data: x.data,
            })
        }
    }
}

//------------------------------------------------------------------------------

#[derive(Debug)]
pub struct BinChunkHeader {
    pub pos: u64,
    pub length: u32,
}

impl TryFrom<ChunkHeader> for BinChunkHeader {
    type Error = IOError;
    fn try_from(x: ChunkHeader) -> IOResult<Self> {
        if x.ctype != TYPE_BIN {
            Err(IOError::Gltf(GltfError::BinChunk))
        } else {
            Ok(Self {
                pos: x.pos,
                length: x.length,
            })
        }
    }
}

//------------------------------------------------------------------------------

const VALID_MAGIC: u32 = 0x46546C67; //"glTF"
const VALID_VERSION: u32 = 2;
const TYPE_JSON: u32 = 0x4E4F534A; // "JSON"
const TYPE_BIN: u32 = 0x004E4942; // "BIN"

//------------------------------------------------------------------------------
