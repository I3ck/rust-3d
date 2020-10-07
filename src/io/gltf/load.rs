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

//! Module for load operations of the glTF file formats

use crate::*;

use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::File,
    io::{Read, Seek, SeekFrom},
    iter::FusedIterator,
    marker::PhantomData,
    path::PathBuf,
};

use super::{
    super::{byte_reader::*, types::*, utils::*},
    types::*,
};

use serde_json;

//------------------------------------------------------------------------------

/// Loads an IsMesh3D from the glb file format
pub fn load_glb<EM, P, R>(read: R, folder_path: PathBuf, mesh: &mut EM) -> IOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + IsMatrix4Transformable + Clone,
    R: Read + Seek,
{
    let iterator = GltfIterator::<P, R>::new_glb(read, folder_path)?;

    for rd in iterator {
        match rd? {
            FaceDataReserve::Data(x) => {
                mesh.add_vertex(x);
            }
            FaceDataReserve::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .map_err(|_| IOError::InvalidMeshIndices)?;
            }
            FaceDataReserve::ReserveDataFaces(n_vertices, n_faces) => {
                mesh.reserve_vertices(n_vertices);
                mesh.reserve_faces(n_faces);
            }
            FaceDataReserve::ReserveDataFacesExact(n_vertices, n_faces) => {
                mesh.reserve_vertices_exact(n_vertices);
                mesh.reserve_faces_exact(n_faces);
            }
        }
    }

    Ok(())
}

/// Loads an IsMesh3D from the glTF file format
pub fn load_gltf<EM, P, R>(read: R, folder_path: PathBuf, mesh: &mut EM) -> IOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + IsMatrix4Transformable + Clone,
    R: Read + Seek,
{
    let iterator = GltfIterator::<P, R>::new_gltf(read, folder_path)?;

    for rd in iterator {
        match rd? {
            FaceDataReserve::Data(x) => {
                mesh.add_vertex(x);
            }
            FaceDataReserve::Face([a, b, c]) => {
                mesh.try_add_connection(VId(a), VId(b), VId(c))
                    .map_err(|_| IOError::InvalidMeshIndices)?;
            }
            FaceDataReserve::ReserveDataFaces(n_vertices, n_faces) => {
                mesh.reserve_vertices(n_vertices);
                mesh.reserve_faces(n_faces);
            }
            FaceDataReserve::ReserveDataFacesExact(n_vertices, n_faces) => {
                mesh.reserve_vertices_exact(n_vertices);
                mesh.reserve_faces_exact(n_faces);
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Iterator to incrementally load a .glTF or .glb file
pub struct GltfIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
    chunk_offset: u64,
    root: Root,
    is_done: bool,
    node_trace: Vec<usize>,
    current_primitive: usize,
    pf_iterator: PointFaceIterator<P, R>,
    phantom: PhantomData<P>,
}

impl<P, R> GltfIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
    /// Creates an iterator for reading a .glb file
    pub fn new_glb(mut read: R, folder_path: PathBuf) -> IOResult<Self> {
        let _header = read_file_header(&mut read)?;
        let pos_chunk_json = read.seek(SeekFrom::Current(0))?;
        let chunk_json =
            read_chunk(&mut read, pos_chunk_json).and_then(|x| JSONChunk::try_from(x))?;
        let pos_chunk_bin = read.seek(SeekFrom::Current(0))? + 8; // +8 since two u32 are part of the header
        let chunk_bin = read_chunk_header(&mut read, pos_chunk_bin)
            .and_then(|x| BinChunkHeader::try_from(x))?;

        let json = parse_json(&chunk_json)?;

        let root = Root::new(&json)?;

        let mut result = Self {
            root,
            chunk_offset: chunk_bin.pos,
            is_done: false,
            node_trace: vec![0],
            current_primitive: 0,
            pf_iterator: PointFaceIterator::new(read, folder_path),
            phantom: PhantomData,
        };

        result.node_trace = result.decended_left(result.node_trace.clone());
        result.fetch_data()?;

        Ok(result)
    }

    /// Creates an iterator for reading a .glTF file
    pub fn new_gltf(mut read: R, folder_path: PathBuf) -> IOResult<Self> {
        let json: serde_json::Value = serde_json::from_reader(&mut read)?;
        let root = Root::new(&json)?;

        let mut result = Self {
            root,
            chunk_offset: 0,
            is_done: false,
            node_trace: vec![0],
            current_primitive: 0,
            pf_iterator: PointFaceIterator::new(read, folder_path),
            phantom: PhantomData,
        };

        result.node_trace = result.decended_left(result.node_trace.clone());
        result.fetch_data()?;

        Ok(result)
    }

    #[inline(always)]
    fn current_node(&self) -> Option<&Node> {
        self.node_of_trace(&self.node_trace)
    }

    #[inline(always)]
    fn node_of_trace(&self, trace: &[usize]) -> Option<&Node> {
        if trace.is_empty() {
            None
        } else if trace.len() == 1 {
            self.root.root_nodes.get(self.node_trace[0])
        } else {
            Self::follow_trace(self.root.root_nodes.get(self.node_trace[0])?, &trace[1..])
        }
    }

    #[inline(always)]
    fn follow_trace<'a>(node: &'a Node, trace: &[usize]) -> Option<&'a Node> {
        if trace.is_empty() {
            Some(node)
        } else {
            match &node.mesh_or_children {
                MeshOrChildren::Mesh(_) => None,
                MeshOrChildren::Children(children) => {
                    Self::follow_trace(children.get(trace[0])?, &trace[1..])
                } //@todo recursion limit?
            }
        }
    }

    #[inline(always)]
    fn fetch_data(&mut self) -> IOResult<bool> {
        if let (Some(current_node), Some(mesh)) = (
            self.current_node(),
            self.current_node().and_then(|x| x.mesh_or_children.mesh()),
        ) {
            if mesh.primitives.is_empty() {
                return Ok(false);
            }
            let primitive = &mesh.primitives[self.current_primitive];

            let acc_pos = &primitive.positions;
            let bw_pos = &acc_pos.buffer_view;
            let (uri_pos, offset_pos) = match &bw_pos.buffer.uri {
                None => (None, self.chunk_offset),
                Some(x) => (Some(x.clone()), 0),
            };

            let p_settings = PointIterSettings {
                uri: uri_pos,
                seek_start: offset_pos + acc_pos.byte_offset + bw_pos.byte_offset,
                to_fetch: acc_pos.count as usize,
                bytes_to_skip: if let Some(stride) = bw_pos.byte_stride {
                    let size = 3 * 4;
                    if stride < size {
                        return Err(IOError::Gltf(GltfError::Stride));
                    }
                    (stride - size) as usize
                } else {
                    0
                },
                transformation: current_node.transformation.clone(),
            };

            let f_settings = if let Some(acc_id) = &primitive.indices {
                let bw_id = &acc_id.buffer_view;
                let ct = acc_id.component_type;

                let (uri_id, offset_id) = match &bw_id.buffer.uri {
                    None => (None, self.chunk_offset),
                    Some(x) => (Some(x.clone()), 0),
                };

                Some(FaceIterSettings {
                    uri: uri_id,
                    seek_start: offset_id + acc_id.byte_offset + bw_id.byte_offset,
                    to_fetch: acc_id.count as usize / 3,
                    bytes_to_skip: if let Some(stride) = bw_id.byte_stride {
                        let size = match ct {
                            IndexComponentType::U8 => 3 * 1,  // 3 * 1byte
                            IndexComponentType::U16 => 3 * 2, // 3 * 2bytes
                            IndexComponentType::U32 => 3 * 4, // 3 * 4bytes
                        };
                        if stride < size {
                            return Err(IOError::Gltf(GltfError::Stride));
                        }
                        (stride - size) as usize
                    } else {
                        0
                    },
                    component_type: ct,
                })
            } else {
                None
            };

            self.pf_iterator.update(p_settings, f_settings)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn ensure_data_available(&mut self) -> IOResult<bool> {
        if !self.pf_iterator.is_done() {
            return Ok(true);
        }
        if self.next_primitive()? {
            if self.fetch_data()? {
                Ok(true)
            } else {
                self.ensure_data_available() //@todo take care of to deep recursion
            }
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    fn next_primitive(&mut self) -> IOResult<bool> {
        if let Some(mesh) = self.current_node().and_then(|x| x.mesh_or_children.mesh()) {
            let n_primitives = mesh.primitives.len();
            if self.current_primitive + 1 < n_primitives {
                self.current_primitive += 1;
                Ok(true)
            } else {
                self.next_node()
            }
        } else {
            self.next_node()
        }
    }

    #[inline(always)]
    fn decended_left(&self, mut trace: Vec<usize>) -> Vec<usize> {
        if let Some(node) = self.node_of_trace(&trace) {
            if !node
                .mesh_or_children
                .children()
                .map(|x| x.is_empty())
                .unwrap_or(true)
            {
                trace.push(0);
                self.decended_left(trace)
            } else {
                trace
            }
        } else {
            trace
        }
    }

    #[inline(always)]
    fn trace_for_next_node(&self, mut trace: Vec<usize>) -> Option<Vec<usize>> {
        if trace.is_empty() {
            None
        } else {
            let i = trace.len() - 1;
            trace[i] += 1;
            if self.node_of_trace(&trace).is_some() {
                Some(self.decended_left(trace))
            } else {
                trace.pop();
                self.trace_for_next_node(trace)
            }
        }
    }

    #[inline(always)]
    fn next_node(&mut self) -> IOResult<bool> {
        self.current_primitive = 0;
        if let Some(trace) = self.trace_for_next_node(self.node_trace.clone()) {
            self.node_trace = trace;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<P, R> Iterator for GltfIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
    type Item = IOResult<FaceDataReserve<P>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        match self.ensure_data_available() {
            Ok(true) => self.pf_iterator.next(),
            Ok(false) => {
                self.is_done = true;
                None
            }
            Err(e) => return Some(Err(e)),
        }
    }
}

impl<P, R> FusedIterator for GltfIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
}

//------------------------------------------------------------------------------

#[derive(Default, Debug)]
struct PointIterSettings {
    pub uri: Option<String>, //@todo &str?
    pub seek_start: u64,
    pub to_fetch: usize,
    pub bytes_to_skip: usize,
    pub transformation: Option<Matrix4>,
}

#[derive(Default, Debug, Clone)]
struct FaceIterSettings {
    pub uri: Option<String>, //@todo &str?
    pub seek_start: u64,
    pub to_fetch: usize,
    pub bytes_to_skip: usize,
    pub component_type: IndexComponentType,
}

struct PointFaceIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
    root_read: R,
    folder_path: PathBuf,
    #[allow(dead_code)]
    uri_readers: HashMap<PathBuf, File>,
    p_settings: PointIterSettings,
    f_settings: Option<FaceIterSettings>,
    points_pushed: usize,
    index_offset: usize,
    data_faces_to_reserve: [usize; 2],
    phantom: PhantomData<P>,
}

impl<P, R> PointFaceIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
    pub fn new(root_read: R, folder_path: PathBuf) -> Self {
        Self {
            root_read,
            folder_path,
            uri_readers: HashMap::default(),
            p_settings: Default::default(),
            f_settings: Default::default(),
            points_pushed: 0,
            index_offset: 0,
            data_faces_to_reserve: [0, 0],
            phantom: PhantomData,
        }
    }

    pub fn update(&mut self, p: PointIterSettings, f: Option<FaceIterSettings>) -> IOResult<()> {
        self.p_settings = p;
        self.f_settings = f;
        self.index_offset = self.points_pushed;
        self.data_faces_to_reserve = [
            self.p_settings.to_fetch,
            self.f_settings.as_ref().map(|x| x.to_fetch).unwrap_or(0),
        ];

        if self.p_settings.to_fetch != 0 {
            self.seek_to_points()
        } else {
            self.seek_to_faces()
        }
    }

    pub fn is_done(&self) -> bool {
        self.p_settings.to_fetch == 0
            && self.f_settings.as_ref().map(|x| x.to_fetch).unwrap_or(0) == 0
    }

    //@todo inline all these

    fn fetch_point<R2>(
        read: &mut R2,
        p_settings: &PointIterSettings,
    ) -> IOResult<FaceDataReserve<P>>
    where
        R2: Read,
    {
        let x = LittleReader::read_f32(read)?;
        let y = LittleReader::read_f32(read)?;
        let z = LittleReader::read_f32(read)?;

        if p_settings.to_fetch != 0 && p_settings.bytes_to_skip != 0 {
            skip_bytes(read, p_settings.bytes_to_skip)?
        }

        let mut p = P::new(x as f64, y as f64, z as f64);
        if let Some(t) = &p_settings.transformation {
            p.transform(t)
        }

        Ok(FaceDataReserve::Data(p))
    }

    fn fetch_face<R2>(
        index_offset: usize,
        read: &mut R2,
        f_settings: &FaceIterSettings,
    ) -> IOResult<FaceDataReserve<P>>
    where
        R2: Read,
    {
        let o = index_offset;

        match f_settings.component_type {
            IndexComponentType::U8 => {
                let vid1 = LittleReader::read_u8(read)?;
                let vid2 = LittleReader::read_u8(read)?;
                let vid3 = LittleReader::read_u8(read)?;

                if f_settings.to_fetch != 0 && f_settings.bytes_to_skip != 0 {
                    skip_bytes(read, f_settings.bytes_to_skip)?
                }

                Ok(FaceDataReserve::Face([
                    vid1 as usize + o,
                    vid2 as usize + o,
                    vid3 as usize + o,
                ]))
            }
            IndexComponentType::U16 => {
                let vid1 = LittleReader::read_u16(read)?;
                let vid2 = LittleReader::read_u16(read)?;
                let vid3 = LittleReader::read_u16(read)?;

                if f_settings.to_fetch != 0 && f_settings.bytes_to_skip != 0 {
                    skip_bytes(read, f_settings.bytes_to_skip)?
                }

                Ok(FaceDataReserve::Face([
                    vid1 as usize + o,
                    vid2 as usize + o,
                    vid3 as usize + o,
                ]))
            }

            IndexComponentType::U32 => {
                let vid1 = LittleReader::read_u32(read)?;
                let vid2 = LittleReader::read_u32(read)?;
                let vid3 = LittleReader::read_u32(read)?;

                if f_settings.to_fetch != 0 && f_settings.bytes_to_skip != 0 {
                    skip_bytes(read, f_settings.bytes_to_skip)?
                }

                Ok(FaceDataReserve::Face([
                    vid1 as usize + o,
                    vid2 as usize + o,
                    vid3 as usize + o,
                ]))
            }
        }
    }

    fn seek_to_points(&mut self) -> IOResult<()> {
        match &self.p_settings.uri {
            None => {
                self.root_read
                    .seek(SeekFrom::Start(self.p_settings.seek_start))?;
            }
            Some(x) => {
                let path = self.folder_path.join(x);
                let mut read = if let Some(x) = self.uri_readers.get(&path) {
                    x
                } else {
                    let entry = File::open(path.clone())
                        .map_err(|_| IOError::Gltf(GltfError::BufferUriAccess))?;
                    self.uri_readers.insert(path.clone(), entry);
                    self.uri_readers.get(&path).as_ref().unwrap() //unwrap safe, since just inserted
                };
                read.seek(SeekFrom::Start(self.p_settings.seek_start))?;
            }
        }

        Ok(())
    }

    fn seek_to_faces(&mut self) -> IOResult<()> {
        if let Some(f_settings) = &self.f_settings {
            match &f_settings.uri {
                None => {
                    self.root_read
                        .seek(SeekFrom::Start(f_settings.seek_start))?;
                }
                Some(x) => {
                    let path = self.folder_path.join(x); //@todo unwrap
                    let mut read = if let Some(x) = self.uri_readers.get(&path) {
                        x
                    } else {
                        let entry = File::open(path.clone())
                            .map_err(|_| IOError::Gltf(GltfError::BufferUriAccess))?;
                        self.uri_readers.insert(path.clone(), entry);
                        self.uri_readers.get(&path).as_ref().unwrap() //unwrap safe, since just inserted
                    };
                    read.seek(SeekFrom::Start(f_settings.seek_start))?;
                }
            }
        }
        Ok(())
    }
}

impl<P, R> Iterator for PointFaceIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
    type Item = IOResult<FaceDataReserve<P>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data_faces_to_reserve != [0, 0] {
            self.data_faces_to_reserve = [0, 0];
            Some(Ok(FaceDataReserve::ReserveDataFaces(
                self.data_faces_to_reserve[0],
                self.data_faces_to_reserve[1],
            )))
        } else if self.p_settings.to_fetch != 0 {
            self.points_pushed += 1;
            self.p_settings.to_fetch -= 1;

            let result = match &self.p_settings.uri {
                None => Self::fetch_point(&mut self.root_read, &self.p_settings),
                Some(x) => {
                    let path = self.folder_path.join(x);
                    Self::fetch_point(&mut self.uri_readers.get(&path).unwrap(), &self.p_settings)
                } //unwrap safe, since inserting in seek
            };

            if self.p_settings.to_fetch == 0 {
                match self.seek_to_faces() {
                    Err(e) => Some(Err(e)),
                    Ok(_) => Some(result),
                }
            } else {
                Some(result)
            }
        } else if let Some(f_settings) = &mut self.f_settings {
            if f_settings.to_fetch != 0 {
                f_settings.to_fetch -= 1;

                Some(match &f_settings.uri {
                    None => Self::fetch_face(self.index_offset, &mut self.root_read, f_settings),
                    Some(x) => {
                        let path = self.folder_path.join(x);
                        Self::fetch_face(
                            self.index_offset,
                            &mut self.uri_readers.get(&path).unwrap(),
                            f_settings,
                        )
                    } //unwrap safe, since inserting in seek
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<P, R> FusedIterator for PointFaceIterator<P, R>
where
    P: IsBuildable3D + IsMatrix4Transformable,
    R: Read + Seek,
{
}

//------------------------------------------------------------------------------

fn read_file_header<R>(read: &mut R) -> IOResult<FileHeader>
where
    R: Read,
{
    let magic = LittleReader::read_u32(read)?;
    let version = LittleReader::read_u32(read)?;
    let length = LittleReader::read_u32(read)?;

    let raw = RawFileHeader {
        magic,
        version,
        length,
    };

    FileHeader::try_from(raw)
}

//------------------------------------------------------------------------------

fn read_chunk_header<R>(read: &mut R, pos: u64) -> IOResult<ChunkHeader>
where
    R: Read,
{
    let length = LittleReader::read_u32(read)?;
    let ctype = LittleReader::read_u32(read)?;

    Ok(ChunkHeader { pos, length, ctype })
}

//------------------------------------------------------------------------------

fn read_chunk<R>(read: &mut R, pos: u64) -> IOResult<Chunk>
where
    R: Read,
{
    let header = read_chunk_header(read, pos)?;
    let mut data = vec![0; header.length as usize];
    read.read_exact(&mut data)?;
    Ok(Chunk { header, data })
}

fn parse_json(chunk: &JSONChunk) -> IOResult<serde_json::Value> {
    let value = serde_json::from_slice(&chunk.data)?;
    Ok(value)
}
