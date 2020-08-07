/*
Copyright 2017 Martin Buck

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

//! Module for IO operations of the stl file format

use crate::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError, Read, Write},
};

use fnv::FnvHashMap;

use super::{byte_reader::*, from_bytes::*, types::*, utils::*};

//------------------------------------------------------------------------------

//@todo can be resolved in a better way once https://github.com/rust-lang/rust/issues/48043 is on stable
//@todo work around in case the binary data is invalid
const MAX_TRIANGLES_BINARY: u32 = 1_000_000_000;

//------------------------------------------------------------------------------

/// Saves an IsMesh3D in the ASCII .stl file format
pub fn save_stl_ascii<M, P, W>(write: &mut W, mesh: &M) -> StlResult<()>
where
    M: IsMesh3D<P>,
    P: IsBuildable3D,
    W: Write,
{
    write.write_all(b"solid STL generated by rust-3d\n")?;

    for i in 0..mesh.num_faces() {
        let [v1, v2, v3] = mesh.face_vertices(FId { val: i }).unwrap(); // safe since iterating num_faces
        let n = mesh.face_normal(FId { val: i }).unwrap(); // safe since iterating num_faces
        let buffer = "facet normal ".to_string()
            + &str_exp(&n)
            + "\n"
            + "    outer loop\n"
            + "        vertex "
            + &str_exp(&v1)
            + "\n"
            + "        vertex "
            + &str_exp(&v2)
            + "\n"
            + "        vertex "
            + &str_exp(&v3)
            + "\n"
            + "    endloop\n"
            + "endfacet\n";
        write.write_all(buffer.as_bytes())?;
    }
    write.write_all(b"endsolid STL generated by rust-3d\n")?;
    Ok(())
}

//------------------------------------------------------------------------------

/// Loads a Mesh from .stl file with duplicate vertices
pub fn load_stl_mesh_duped<EM, P, R, IPN>(
    read: &mut R,
    format: StlFormat,
    mesh: &mut EM,
    face_normals: &mut IPN,
) -> StlIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
    IPN: IsPushable<P>,
{
    if is_ascii(read, format).simple()? {
        load_stl_mesh_duped_ascii(read, mesh, face_normals)
    } else {
        load_stl_mesh_duped_binary(read, mesh, face_normals).simple()
    }
}

//------------------------------------------------------------------------------

/// Loads a Mesh from .stl file with unique vertices, dropping invalid triangles
pub fn load_stl_mesh_unique<EM, P, R, IPN>(
    read: &mut R,
    format: StlFormat,
    mesh: &mut EM,
    face_normals: &mut IPN,
) -> StlIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
    IPN: IsPushable<P>,
{
    if is_ascii(read, format).simple()? {
        load_stl_mesh_unique_ascii(read, mesh, face_normals)
    } else {
        load_stl_mesh_unique_binary(read, mesh, face_normals).simple()
    }
}

//------------------------------------------------------------------------------

/// Loads points from .stl file as triplets into IsPushable<IsBuildable3D>
pub fn load_stl_triplets<IP, P, R, IPN>(
    read: &mut R,
    format: StlFormat,
    ip: &mut IP,
    face_normals: &mut IPN,
) -> StlIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
    IPN: IsPushable<P>,
{
    if is_ascii(read, format).simple()? {
        load_stl_triplets_ascii(read, ip, face_normals)
    } else {
        load_stl_triplets_binary(read, ip, face_normals).simple()
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

fn is_ascii<R>(read: &mut R, format: StlFormat) -> StlResult<bool>
where
    R: BufRead,
{
    let solid = "solid".as_bytes();
    let mut buffer = [0u8; 5];

    let mut result = true;
    read.read_exact(&mut buffer)?;

    for i in 0..5 {
        if buffer[i] != solid[i] {
            result = false
        }
    }

    // It is important to always consume the bytes above, even if format defines the result
    Ok(match format {
        StlFormat::Ascii => true,
        StlFormat::Binary => false,
        StlFormat::Auto => result,
    })
}

//------------------------------------------------------------------------------

fn load_stl_mesh_duped_ascii<EM, P, R, IPN>(
    read: &mut R,
    mesh: &mut EM,
    face_normals: &mut IPN,
) -> StlIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
    IPN: IsPushable<P>,
{
    let mut i_line = 0;
    let mut line_buffer = Vec::new();

    // skip first line
    read.read_until(b'\n', &mut line_buffer).index(i_line)?;
    i_line += 1;

    loop {
        match read_stl_facet(read, &mut line_buffer, &mut i_line) {
            Ok([a, b, c, n]) => {
                mesh.add_face(a, b, c);
                face_normals.push(n);
            }
            Err(WithLineInfo::None(StlError::LoadFileEndReached))
            | Err(WithLineInfo::Index(_, StlError::LoadFileEndReached))
            | Err(WithLineInfo::Line(_, _, StlError::LoadFileEndReached)) => break,
            Err(x) => return Err(x),
        }
    }

    Ok(())
}

struct StlTriangle {
    pub n: [f32; 3],
    pub x: [f32; 3],
    pub y: [f32; 3],
    pub z: [f32; 3],
}

#[inline(always)]
fn read_stl_triangle<R>(read: &mut R) -> StlResult<StlTriangle>
where
    R: Read,
{
    // size for StlTriangle + u16 garbage
    let mut buffer = [0u8; 50];
    read.read_exact(&mut buffer)?;

    Ok(StlTriangle {
        n: array_from_bytes_le!(f32, 3, &buffer[0..12])?,
        x: array_from_bytes_le!(f32, 3, &buffer[12..24])?,
        y: array_from_bytes_le!(f32, 3, &buffer[24..36])?,
        z: array_from_bytes_le!(f32, 3, &buffer[36..48])?,
    })
}

//------------------------------------------------------------------------------

fn load_stl_mesh_duped_binary<EM, P, R, IPN>(
    read: &mut R,
    mesh: &mut EM,
    face_normals: &mut IPN,
) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
    IPN: IsPushable<P>,
{
    // Drop header ('solid' is already dropped)
    {
        let mut buffer = [0u8; 75];
        read.read_exact(&mut buffer)?;
    }

    let n_triangles = LittleReader::read_u32(read)?;
    if n_triangles > MAX_TRIANGLES_BINARY {
        return Err(StlError::InvalidFaceCount);
    }

    mesh.reserve_vertices(3 * n_triangles as usize);
    mesh.reserve_faces(n_triangles as usize);

    face_normals.reserve(n_triangles as usize);

    for _ in 0..n_triangles {
        let t = read_stl_triangle(read)?;

        let n = P::new(t.n[0] as f64, t.n[1] as f64, t.n[2] as f64);
        let x = P::new(t.x[0] as f64, t.x[1] as f64, t.x[2] as f64);
        let y = P::new(t.y[0] as f64, t.y[1] as f64, t.y[2] as f64);
        let z = P::new(t.z[0] as f64, t.z[1] as f64, t.z[2] as f64);

        mesh.add_face(x, y, z);
        face_normals.push(n)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_stl_mesh_unique_ascii<EM, P, R, IPN>(
    read: &mut R,
    mesh: &mut EM,
    face_normals: &mut IPN,
) -> StlIOResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
    IPN: IsPushable<P>,
{
    let mut i_line = 0;
    let mut line_buffer = Vec::new();

    // skip first line
    read.read_until(b'\n', &mut line_buffer).index(i_line)?;
    i_line += 1;

    let mut map = FnvHashMap::default();

    loop {
        match read_stl_facet::<P, _>(read, &mut line_buffer, &mut i_line) {
            Ok([a, b, c, n]) => {
                let id_a = *map.entry(a.clone()).or_insert_with(|| {
                    let value = mesh.num_vertices();
                    mesh.add_vertex(a);
                    value
                });

                let id_b = *map.entry(b.clone()).or_insert_with(|| {
                    let value = mesh.num_vertices();
                    mesh.add_vertex(b);
                    value
                });

                let id_c = *map.entry(c.clone()).or_insert_with(|| {
                    let value = mesh.num_vertices();
                    mesh.add_vertex(c);
                    value
                });

                // Ignore this issues since this only fails if a triangle uses a vertex multiple times
                // Simply do not add this triangle and normal
                match mesh.try_add_connection(
                    VId { val: id_a },
                    VId { val: id_b },
                    VId { val: id_c },
                ) {
                    Ok(_) => {
                        face_normals.push(n);
                    }
                    Err(_) => (),
                }
            }
            Err(WithLineInfo::None(StlError::LoadFileEndReached))
            | Err(WithLineInfo::Index(_, StlError::LoadFileEndReached))
            | Err(WithLineInfo::Line(_, _, StlError::LoadFileEndReached)) => break,
            Err(x) => return Err(x),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_stl_mesh_unique_binary<EM, P, R, IPN>(
    read: &mut R,
    mesh: &mut EM,
    face_normals: &mut IPN,
) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
    IPN: IsPushable<P>,
{
    // Drop header ('solid' is already dropped)
    {
        let mut buffer = [0u8; 75];
        read.read_exact(&mut buffer)?;
    }

    let n_triangles = LittleReader::read_u32(read)?;
    if n_triangles > MAX_TRIANGLES_BINARY {
        return Err(StlError::InvalidFaceCount);
    }

    mesh.reserve_vertices((0.5 * n_triangles as f64) as usize);
    mesh.reserve_faces(n_triangles as usize);

    face_normals.reserve(n_triangles as usize);

    let mut map = FnvHashMap::default();

    for _ in 0..n_triangles {
        let t = read_stl_triangle(read)?;

        let n = P::new(t.n[0] as f64, t.n[1] as f64, t.n[2] as f64);
        let x = P::new(t.x[0] as f64, t.x[1] as f64, t.x[2] as f64);
        let y = P::new(t.y[0] as f64, t.y[1] as f64, t.y[2] as f64);
        let z = P::new(t.z[0] as f64, t.z[1] as f64, t.z[2] as f64);

        let id_x = *map.entry(x.clone()).or_insert_with(|| {
            let value = mesh.num_vertices();
            mesh.add_vertex(x);
            value
        });

        let id_y = *map.entry(y.clone()).or_insert_with(|| {
            let value = mesh.num_vertices();
            mesh.add_vertex(y);
            value
        });

        let id_z = *map.entry(z.clone()).or_insert_with(|| {
            let value = mesh.num_vertices();
            mesh.add_vertex(z);
            value
        });

        // Ignore this issues since this only fails if a triangle uses a vertex multiple times
        // Simply do not add this triangle
        match mesh.try_add_connection(VId { val: id_x }, VId { val: id_y }, VId { val: id_z }) {
            Ok(_) => {
                face_normals.push(n);
            }
            Err(_) => (),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_stl_triplets_ascii<IP, P, R, IPN>(
    read: &mut R,
    ip: &mut IP,
    face_normals: &mut IPN,
) -> StlIOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
    IPN: IsPushable<P>,
{
    let mut i_line = 0;
    let mut line_buffer = Vec::new();

    // skip first line
    read.read_until(b'\n', &mut line_buffer).index(i_line)?;
    i_line += 1;

    loop {
        match read_stl_facet(read, &mut line_buffer, &mut i_line) {
            Ok([a, b, c, n]) => {
                ip.push(a);
                ip.push(b);
                ip.push(c);
                face_normals.push(n);
            }
            Err(WithLineInfo::None(StlError::LoadFileEndReached))
            | Err(WithLineInfo::Index(_, StlError::LoadFileEndReached))
            | Err(WithLineInfo::Line(_, _, StlError::LoadFileEndReached)) => break,
            Err(x) => return Err(x),
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn load_stl_triplets_binary<IP, P, R, IPN>(
    read: &mut R,
    ip: &mut IP,
    face_normals: &mut IPN,
) -> StlResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: Read,
    IPN: IsPushable<P>,
{
    // Drop header ('solid' is already dropped)
    {
        let mut buffer = [0u8; 75];
        read.read_exact(&mut buffer)?;
    }

    let n_triangles = LittleReader::read_u32(read)?;
    if n_triangles > MAX_TRIANGLES_BINARY {
        return Err(StlError::InvalidFaceCount);
    }

    ip.reserve(3 * n_triangles as usize);
    face_normals.reserve(n_triangles as usize);

    for _ in 0..n_triangles {
        let t = read_stl_triangle(read)?;

        let n = P::new(t.n[0] as f64, t.n[1] as f64, t.n[2] as f64);
        let x = P::new(t.x[0] as f64, t.x[1] as f64, t.x[2] as f64);
        let y = P::new(t.y[0] as f64, t.y[1] as f64, t.y[2] as f64);
        let z = P::new(t.z[0] as f64, t.z[1] as f64, t.z[2] as f64);

        ip.push(x);
        ip.push(y);
        ip.push(z);

        face_normals.push(n)
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn read_stl_facet<P, R>(
    read: &mut R,
    line_buffer: &mut Vec<u8>,
    i_line: &mut usize,
) -> StlIOResult<[P; 4]>
where
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line: &[u8];

    line = trim_start(fetch_line(read, line_buffer).index(*i_line)?);
    *i_line += 1;

    if line.starts_with(b"endsolid") {
        return Err(StlError::LoadFileEndReached).line(*i_line, line);
    }

    if !line.starts_with(b"facet") {
        return Err(StlError::Facet).line(*i_line, line);
    }

    let n = read_stl_normal(&line).unwrap_or(P::new(0.0, 0.0, 1.0));

    line = trim_start(fetch_line(read, line_buffer).index(*i_line)?);
    *i_line += 1;

    if !line.starts_with(b"outer loop") {
        return Err(StlError::Loop).line(*i_line, line);
    }

    line = fetch_line(read, line_buffer).index(*i_line)?;
    *i_line += 1;

    let a = read_stl_vertex(&line)
        .ok_or(StlError::Vertex)
        .line(*i_line, line)?;

    line = fetch_line(read, line_buffer).index(*i_line)?;
    *i_line += 1;

    let b = read_stl_vertex(&line)
        .ok_or(StlError::Vertex)
        .line(*i_line, line)?;

    line = fetch_line(read, line_buffer).index(*i_line)?;
    *i_line += 1;

    let c = read_stl_vertex(&line)
        .ok_or(StlError::Vertex)
        .line(*i_line, line)?;

    line = trim_start(fetch_line(read, line_buffer).index(*i_line)?);
    *i_line += 1;

    if !line.starts_with(b"endloop") {
        return Err(StlError::EndLoop).line(*i_line, line);
    }

    line = trim_start(fetch_line(read, line_buffer).index(*i_line)?);
    *i_line += 1;

    if !line.starts_with(b"endfacet") {
        return Err(StlError::EndFacet).line(*i_line, line);
    }

    Ok([a, b, c, n])
}

//------------------------------------------------------------------------------

fn read_stl_vertex<P>(line: &[u8]) -> Option<P>
where
    P: IsBuildable3D,
{
    let mut words = to_words_skip_empty(line);

    // skip "vertex"
    words.next()?;

    let x = from_ascii(words.next()?)?;
    let y = from_ascii(words.next()?)?;
    let z = from_ascii(words.next()?)?;

    Some(P::new(x, y, z))
}

//------------------------------------------------------------------------------

fn read_stl_normal<P>(line: &[u8]) -> Option<P>
where
    P: IsBuildable3D,
{
    let mut words = to_words_skip_empty(line);

    // skip "facet"
    words.next()?;

    // skip "normal"
    words.next()?;

    let i = from_ascii(words.next()?)?;
    let j = from_ascii(words.next()?)?;
    let k = from_ascii(words.next()?)?;

    Some(P::new(i, j, k))
}

//------------------------------------------------------------------------------

fn str_exp<P>(p: &P) -> String
where
    P: Is3D,
{
    format!("{:e} {:e} {:e}", p.x(), p.y(), p.z()).to_string()
}

//------------------------------------------------------------------------------

/// Whether format shall be considered to be binary/ASCII or auto determined
#[derive(Copy, Clone)]
pub enum StlFormat {
    Ascii,
    Binary,
    Auto,
}

impl Default for StlFormat {
    fn default() -> Self {
        Self::Auto
    }
}

//------------------------------------------------------------------------------

/// Error type for .stl file operations
pub enum StlError {
    LoadFileEndReached,
    AccessFile,
    BinaryData,
    InvalidFaceCount,
    Facet,
    EndFacet,
    Vertex,
    Loop,
    EndLoop,
}

/// Result type for .stl file operations
pub type StlResult<T> = std::result::Result<T, StlError>;

/// Result type for .stl file operations
pub type StlIOResult<T> = IOResult<T, StlError>;

impl fmt::Debug for StlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LoadFileEndReached => write!(f, "Unexpected reach of .stl file end"),
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::BinaryData => write!(f, "Binary data seems to be invalid"),
            Self::InvalidFaceCount => write!(f, "Containing an invalid face count"),
            Self::Facet => write!(f, "Unable to parse facet"),
            Self::EndFacet => write!(f, "Unable to parse endfacet"),
            Self::Vertex => write!(f, "Unable to parse vertex"),
            Self::Loop => write!(f, "Unable to parse loop"),
            Self::EndLoop => write!(f, "Unable to parse endloop"),
        }
    }
}

impl From<ioError> for StlError {
    fn from(_error: ioError) -> Self {
        StlError::AccessFile
    }
}

impl From<WithLineInfo<ioError>> for WithLineInfo<StlError> {
    fn from(other: WithLineInfo<ioError>) -> Self {
        match other {
            WithLineInfo::<ioError>::None(x) => WithLineInfo::None(StlError::from(x)),
            WithLineInfo::<ioError>::Index(i, x) => WithLineInfo::Index(i, StlError::from(x)),
            WithLineInfo::<ioError>::Line(i, l, x) => WithLineInfo::Line(i, l, StlError::from(x)),
        }
    }
}

impl From<WithLineInfo<FetchLineError>> for WithLineInfo<StlError> {
    fn from(other: WithLineInfo<FetchLineError>) -> Self {
        match other {
            WithLineInfo::<FetchLineError>::None(x) => WithLineInfo::None(StlError::from(x)),
            WithLineInfo::<FetchLineError>::Index(i, x) => {
                WithLineInfo::Index(i, StlError::from(x))
            }
            WithLineInfo::<FetchLineError>::Line(i, l, x) => {
                WithLineInfo::Line(i, l, StlError::from(x))
            }
        }
    }
}

impl From<std::array::TryFromSliceError> for StlError {
    fn from(_error: std::array::TryFromSliceError) -> Self {
        StlError::BinaryData
    }
}
impl From<FromBytesError> for StlError {
    fn from(_error: FromBytesError) -> Self {
        StlError::BinaryData
    }
}

impl From<FetchLineError> for StlError {
    fn from(_error: FetchLineError) -> Self {
        StlError::LoadFileEndReached
    }
}
