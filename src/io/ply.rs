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

//! Module for IO operations of the ply file format

use crate::*;

//@todo float32 usage wrong here?
//@todo consider removal Ply prefix from private types

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use core::str::FromStr;

use std::io::{BufRead, Read, Write};

/*
char -> signed 1 byte
uchar -> unsigned 1 byte
short -> signed 2 bytes
ushort -> unsigned 2 bytes
int -> signed 4 bytes
uint -> unsigned 4 bytes
float -> 4 bytes
double -> 8 bytes
*/

enum PlyType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Float,
    Double,
}

impl PlyType {
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "char" => Some(Self::Char),
            "uchar" => Some(Self::UChar),
            "short" => Some(Self::Short),
            "ushort" => Some(Self::UShort),
            "int" => Some(Self::Int),
            "uint" => Some(Self::UInt),
            "float" | "float32" => Some(Self::Float),
            "double" | "float64" => Some(Self::Double),
            _ => None,
        }
    }

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

#[derive(Default, Debug)]
struct BytesWords {
    pub bytes: usize,
    pub words: usize,
}

#[derive(Debug)]
enum PlyVertexType {
    Float,
    Double,
}

impl PlyVertexType {
    //@todo TryFrom
    pub fn from_ply_type(x: PlyType) -> Option<Self> {
        match x {
            PlyType::Float => Some(Self::Float),
            PlyType::Double => Some(Self::Double),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum PlyFaceType {
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
}

impl PlyFaceType {
    //@todo TryFrom
    pub fn from_ply_type(x: PlyType) -> Option<Self> {
        match x {
            PlyType::Char => Some(Self::Char),
            PlyType::UChar => Some(Self::UChar),
            PlyType::Short => Some(Self::Short),
            PlyType::UShort => Some(Self::UShort),
            PlyType::Int => Some(Self::Int),
            PlyType::UInt => Some(Self::UInt),
            _ => None,
        }
    }
}

fn read_face_type<BO, R>(read: &mut R, t: &PlyFaceType) -> PlyResult<usize>
where
    BO: ByteOrder,
    R: Read,
{
    Ok(match t {
        PlyFaceType::Char => read.read_i8()? as usize,
        PlyFaceType::UChar => read.read_u8()? as usize,
        PlyFaceType::Short => read.read_i16::<BO>()? as usize,
        PlyFaceType::UShort => read.read_u16::<BO>()? as usize,
        PlyFaceType::Int => read.read_i32::<BO>()? as usize,
        PlyFaceType::UInt => read.read_u32::<BO>()? as usize,
    })
}

//@todo property list must also be considered
//@todo must consider case where properties / to skip are defined per face and not per vertex
//@todo settings this must track its scope (if after element vertex or element face)
#[derive(Debug)]
struct PlyVertexFormat {
    pub x: PlyVertexType,
    pub y: PlyVertexType,
    pub z: PlyVertexType,
    pub before: BytesWords,
    pub between_x_y: BytesWords,
    pub between_y_z: BytesWords,
    pub after: BytesWords,
}

//@todo must also check structure itself, not just padding
#[derive(Debug)]
struct PlyFaceFormat {
    pub count: PlyFaceType,
    pub index: PlyFaceType,
    pub before: BytesWords,
    pub after: BytesWords,
}

#[derive(Debug)]
enum PlyFormat {
    Ascii,
    LittleEndian,
    BigEndian,
}

enum PlyHeaderReadState {
    Meta,
    Vertex,
    Face,
}

#[derive(Debug)]
struct PlyHeader {
    pub format: PlyFormat,
    pub n_vertices: usize,
    pub n_faces: usize,
    pub vertex_format: PlyVertexFormat,
    pub face_format: PlyFaceFormat,
}

/// Saves an IsMesh3D in the ASCII .ply file format
pub fn save_ply_ascii<M, P, W>(write: &mut W, mesh: &M) -> PlyResult<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let header = "ply\n".to_string()
        + "format ascii 1.0\n"
        + "comment Created by rust-3d\n"
        + "element vertex "
        + &mesh.num_vertices().to_string()
        + "\n"
        + "property float x\n"
        + "property float y\n"
        + "property float z\n"
        + "element face "
        + &mesh.num_faces().to_string()
        + "\n"
        + "property list uchar uint vertex_indices\n"
        + "end_header\n";
    write.write_all(header.as_bytes())?;

    for i in 0..mesh.num_vertices() {
        let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating num_vertices
        write.write_all((vertex.to_str() + "\n").as_bytes())?;
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating num_faces
        write.write_all(
            ("3 ".to_string()
                + &face.a.to_string()
                + " "
                + &face.b.to_string()
                + " "
                + &face.c.to_string()
                + "\n")
                .as_bytes(),
        )?;
    }
    Ok(())
}

/// Saves an IsMesh3D in the ASCII .ply file format with additional colors
pub fn save_ply_ascii_colored<M, P, W>(write: &mut W, mesh: &M, colors: &Vec<Rgb>) -> PlyResult<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let n_vertices = mesh.num_vertices();
    let n_faces = mesh.num_faces();

    if n_vertices != colors.len() {
        return Err(PlyError::ColorArrayIncorrectLength);
    }

    let header = "ply\n".to_string()
        + "format ascii 1.0\n"
        + "comment Created by rust-3d\n"
        + "element vertex "
        + &n_vertices.to_string()
        + "\n"
        + "property float x\n"
        + "property float y\n"
        + "property float z\n"
        + "property uchar red\n"
        + "property uchar green\n"
        + "property uchar blue\n"
        + "element face "
        + &n_faces.to_string()
        + "\n"
        + "property list uchar uint vertex_indices\n"
        + "end_header\n";
    write.write_all(header.as_bytes())?;

    for i in 0..n_vertices {
        let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating n_vertices
        let color = &colors[i];
        write.write_all(
            format!(
                "{} {} {} {} {} {}\n",
                vertex.x(),
                vertex.y(),
                vertex.z(),
                color.r,
                color.g,
                color.b
            )
            .as_bytes(),
        )?;
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating n_faces
        write.write_all(
            ("3 ".to_string()
                + &face.a.to_string()
                + " "
                + &face.b.to_string()
                + " "
                + &face.c.to_string()
                + "\n")
                .as_bytes(),
        )?;
    }
    Ok(())
}

/// Saves an IsMesh3D in the binary .ply file format
pub fn save_ply_binary<M, P, W>(write: &mut W, mesh: &M, precision: &Precision) -> PlyResult<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &mesh.num_vertices().to_string()
                + "\n"
                + "property float32 x\n"
                + "property float32 y\n"
                + "property float32 z\n"
                + "element face "
                + &mesh.num_faces().to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
        Precision::P64 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &mesh.num_vertices().to_string()
                + "\n"
                + "property float64 x\n"
                + "property float64 y\n"
                + "property float64 z\n"
                + "element face "
                + &mesh.num_faces().to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    write.write_all(header.as_bytes())?;

    match precision {
        Precision::P32 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating num_vertices
                write.write_f32::<BigEndian>(vertex.x() as f32)?;
                write.write_f32::<BigEndian>(vertex.y() as f32)?;
                write.write_f32::<BigEndian>(vertex.z() as f32)?;
            }
        }

        Precision::P64 => {
            for i in 0..mesh.num_vertices() {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating num_vertices
                write.write_f64::<BigEndian>(vertex.x())?;
                write.write_f64::<BigEndian>(vertex.y())?;
                write.write_f64::<BigEndian>(vertex.z())?;
            }
        }
    }

    for i in 0..mesh.num_faces() {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating num_faces
        write.write_u8(3)?;
        write.write_u32::<BigEndian>(face.a.val as u32)?;
        write.write_u32::<BigEndian>(face.b.val as u32)?;
        write.write_u32::<BigEndian>(face.c.val as u32)?;
    }

    Ok(())
}

/// Saves an IsMesh3D in the binary .ply file format with additional colors
pub fn save_ply_binary_colored<M, P, W>(
    write: &mut W,
    mesh: &M,
    precision: &Precision,
    colors: &Vec<Rgb>,
) -> PlyResult<()>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D,
    W: Write,
{
    let n_vertices = mesh.num_vertices();
    let n_faces = mesh.num_faces();

    if n_vertices != colors.len() {
        return Err(PlyError::ColorArrayIncorrectLength);
    }

    let header = match precision {
        Precision::P32 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &n_vertices.to_string()
                + "\n"
                + "property float32 x\n"
                + "property float32 y\n"
                + "property float32 z\n"
                + "property uchar red\n"
                + "property uchar green\n"
                + "property uchar blue\n"
                + "element face "
                + &n_faces.to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
        Precision::P64 => {
            "ply\n".to_string()
                + "format binary_big_endian 1.0\n"
                + "comment Created by rust-3d\n"
                + "element vertex "
                + &n_vertices.to_string()
                + "\n"
                + "property float64 x\n"
                + "property float64 y\n"
                + "property float64 z\n"
                + "property uchar red\n"
                + "property uchar green\n"
                + "property uchar blue\n"
                + "element face "
                + &n_faces.to_string()
                + "\n"
                + "property list uint8 uint32 vertex_indices\n"
                + "end_header\n"
        }
    };

    write.write_all(header.as_bytes())?;

    match precision {
        Precision::P32 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating n_vertices
                let color = &colors[i];
                write.write_f32::<BigEndian>(vertex.x() as f32)?;
                write.write_f32::<BigEndian>(vertex.y() as f32)?;
                write.write_f32::<BigEndian>(vertex.z() as f32)?;
                write.write_u8(color.r)?;
                write.write_u8(color.g)?;
                write.write_u8(color.b)?;
            }
        }

        Precision::P64 => {
            for i in 0..n_vertices {
                let vertex = mesh.vertex(VId { val: i }).unwrap(); // safe since iterating n_vertices
                let color = &colors[i];
                write.write_f64::<BigEndian>(vertex.x())?;
                write.write_f64::<BigEndian>(vertex.y())?;
                write.write_f64::<BigEndian>(vertex.z())?;
                write.write_u8(color.r)?;
                write.write_u8(color.g)?;
                write.write_u8(color.b)?;
            }
        }
    }

    for i in 0..n_faces {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since iterating n_faces
        write.write_u8(3)?;
        write.write_u32::<BigEndian>(face.a.val as u32)?;
        write.write_u32::<BigEndian>(face.b.val as u32)?;
        write.write_u32::<BigEndian>(face.c.val as u32)?;
    }

    Ok(())
}

/// Loads an IsMesh3D from the .ply file format
pub fn load_ply<EM, P, R>(read: &mut R, mesh: &mut EM) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut line_buffer = String::new();
    let mut i_line = 0;

    let header = load_ply_header(read, &mut line_buffer, &mut i_line)?;

    //println!("{:?}", header);

    mesh.reserve_vertices(header.n_vertices);
    mesh.reserve_faces(header.n_faces);

    match header.format {
        PlyFormat::Ascii => load_ply_ascii(read, mesh, &header, &mut line_buffer, &mut i_line),
        PlyFormat::LittleEndian => load_ply_binary::<LittleEndian, _, _, _>(read, mesh, &header),
        PlyFormat::BigEndian => load_ply_binary::<BigEndian, _, _, _>(read, mesh, &header),
    }
}

fn load_ply_header<R>(
    read: &mut R,
    line_buffer: &mut String,
    i_line: &mut usize,
) -> PlyResult<PlyHeader>
where
    R: BufRead,
{
    //@todo foo_found vs found_foo naming convention
    let mut found_ply = false;
    let mut read_state = PlyHeaderReadState::Meta;
    let mut format = None;
    let mut n_vertices: Option<usize> = None;
    let mut n_faces: Option<usize> = None;

    let mut x_type = None;
    let mut y_type = None;
    let mut z_type = None;
    let mut n_types_found = 0;
    //@todo rename these, since now misleading whether vertex or face
    let mut before = BytesWords::default();
    let mut between_x_y = BytesWords::default();
    let mut between_y_z = BytesWords::default();
    let mut after = BytesWords::default();

    let mut face_count_type = None;
    let mut face_index_type = None;
    let mut face_before = BytesWords::default();
    let mut face_after = BytesWords::default();
    let mut face_structure_found = false;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        *i_line += 1;

        if line.starts_with("comment") {
            continue;
        }

        if line.starts_with("obj_info") {
            continue;
        }

        if !found_ply {
            if line == "ply" {
                found_ply = true;
                continue;
            }
            return Err(PlyError::LoadStartNotFound);
        }

        if format.is_none() {
            format = Some(match line {
                "format ascii 1.0" => PlyFormat::Ascii,
                "format binary_little_endian 1.0" => PlyFormat::LittleEndian,
                "format binary_big_endian 1.0" => PlyFormat::BigEndian,
                _ => return Err(PlyError::LoadFormatNotFound),
            });
            continue;
        }

        match n_vertices {
            None => {
                if line.starts_with("element vertex") {
                    read_state = PlyHeaderReadState::Vertex;
                    let mut words = to_words(&line);
                    match words.clone().count() {
                        3 => {
                            n_vertices = Some(
                                usize::from_str(words.nth(2).unwrap())
                                    .map_err(|_| PlyError::LineParse(*i_line))?,
                            );
                            continue;
                        }
                        _ => return Err(PlyError::LineParse(*i_line)),
                    }
                }
            }
            Some(_) => {}
        }

        match n_faces {
            None => {
                if line.starts_with("element face") {
                    read_state = PlyHeaderReadState::Face;
                    let mut words = to_words(&line);
                    match words.clone().count() {
                        3 => {
                            n_faces = Some(
                                usize::from_str(words.nth(2).unwrap())
                                    .map_err(|_| PlyError::LineParse(*i_line))?,
                            );
                            continue;
                        }
                        _ => return Err(PlyError::LineParse(*i_line)),
                    }
                }
            }
            Some(_) => {}
        }

        if line.starts_with("property") {
            match read_state {
                PlyHeaderReadState::Vertex => {
                    let mut words = to_words(line);
                    words.next(); // skip "property"

                    let t = PlyType::from_str(words.next().unwrap()).unwrap(); //@todo error handling, invalid property line
                    let id = words.next().unwrap(); //@todo see above
                    if n_types_found == 0 {
                        if id == "x" {
                            x_type = Some(PlyVertexType::from_ply_type(t).unwrap()); //@todo see above
                            n_types_found += 1;
                        } else {
                            before.bytes += t.size_bytes();
                            before.words += 1;
                        }
                    } else if n_types_found == 1 {
                        if id == "y" {
                            y_type = Some(PlyVertexType::from_ply_type(t).unwrap()); //@todo see above
                            n_types_found += 1;
                        } else {
                            between_x_y.bytes += t.size_bytes();
                            between_x_y.words += 1;
                        }
                    } else if n_types_found == 2 {
                        if id == "z" {
                            z_type = Some(PlyVertexType::from_ply_type(t).unwrap()); //@todo see above
                            n_types_found += 1;
                        } else {
                            between_y_z.bytes += t.size_bytes();
                            between_y_z.words += 1;
                        }
                    } else {
                        after.bytes += t.size_bytes();
                        after.words += 1;
                    }
                }
                PlyHeaderReadState::Face => {
                    if line.starts_with("property list") {
                        if line.contains("vertex_indices") {
                            //@todo is this properly defined?
                            let mut words = to_words(line);
                            words.next(); // skip "property"
                            words.next(); // skip "list"
                            let t_count = PlyFaceType::from_ply_type(
                                PlyType::from_str(words.next().unwrap()).unwrap(),
                            )
                            .unwrap(); //@todo error handling, invalid property line
                            let t_index = PlyFaceType::from_ply_type(
                                PlyType::from_str(words.next().unwrap()).unwrap(),
                            )
                            .unwrap(); //@todo error handling, invalid property line

                            face_count_type = Some(t_count);
                            face_index_type = Some(t_index);

                            //@todo remove bool
                            //@todo later parse the real structure here
                            face_structure_found = true
                        } else {
                            //@todo better error
                            //@todo currently can't handle multiple property list lines (also unlikely)
                            return Err(PlyError::LineParse(*i_line));
                        }
                    } else {
                        let mut words = to_words(line);
                        words.next(); // skip "property"
                        let t = PlyType::from_str(words.next().unwrap()).unwrap(); //@todo error handling, invalid property line
                        if face_structure_found {
                            face_after.bytes += t.size_bytes();
                            face_after.words += 1;
                        } else {
                            face_before.bytes += t.size_bytes();
                            face_before.words += 1;
                        }
                    }
                    //@todo
                }
                _ => return Err(PlyError::LineParse(*i_line)), //@todo better error
            }

            continue;
        }

        //@todo use if let
        if line == "end_header"
            && found_ply
            && format.is_some()
            && n_vertices.is_some()
            && n_faces.is_some()
            && x_type.is_some()
            && y_type.is_some()
            && z_type.is_some()
            && face_count_type.is_some()
            && face_index_type.is_some()
        {
            //@todo nicer way to write this
            // safe due to if above
            return Ok(PlyHeader {
                format: format.unwrap(),
                n_vertices: n_vertices.unwrap(),
                n_faces: n_faces.unwrap(),
                vertex_format: PlyVertexFormat {
                    x: x_type.unwrap(),
                    y: y_type.unwrap(),
                    z: z_type.unwrap(),
                    before,
                    between_x_y,
                    between_y_z,
                    after,
                },
                face_format: PlyFaceFormat {
                    before: face_before,
                    after: face_after,
                    count: face_count_type.unwrap(),
                    index: face_index_type.unwrap(),
                },
            });
        }

        //@todo better error (header could not be parsed / incorrect)
        return Err(PlyError::LoadHeaderEndNotFound);
    }

    Err(PlyError::LoadHeaderEndNotFound)
}

fn load_ply_binary<BO, EM, P, R>(read: &mut R, mesh: &mut EM, header: &PlyHeader) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
    BO: ByteOrder,
{
    for _ in 0..header.n_vertices {
        for _ in 0..header.vertex_format.before.bytes {
            let _ = read.read_u8();
        }
        let x = match header.vertex_format.x {
            PlyVertexType::Float => read.read_f32::<BO>()? as f64,
            PlyVertexType::Double => read.read_f64::<BO>()?,
        };
        for _ in 0..header.vertex_format.between_x_y.bytes {
            let _ = read.read_u8();
        }
        let y = match header.vertex_format.x {
            PlyVertexType::Float => read.read_f32::<BO>()? as f64,
            PlyVertexType::Double => read.read_f64::<BO>()?,
        };
        for _ in 0..header.vertex_format.between_y_z.bytes {
            let _ = read.read_u8();
        }
        let z = match header.vertex_format.x {
            PlyVertexType::Float => read.read_f32::<BO>()? as f64,
            PlyVertexType::Double => read.read_f64::<BO>()?,
        };
        for _ in 0..header.vertex_format.after.bytes {
            let _ = read.read_u8();
        }

        mesh.add_vertex(P::new(x, y, z));
    }

    for _ in 0..header.n_faces {
        for _ in 0..header.face_format.before.bytes {
            let _ = read.read_u8();
        }
        let element_count = read_face_type::<BO, _>(read, &header.face_format.count)?;
        if element_count != 3 {
            return Err(PlyError::LineParse(0)); //@todo incorrect face structure
        }
        let a = read_face_type::<BO, _>(read, &header.face_format.index)?;
        let b = read_face_type::<BO, _>(read, &header.face_format.index)?;
        let c = read_face_type::<BO, _>(read, &header.face_format.index)?;

        for _ in 0..header.face_format.after.bytes {
            let _ = read.read_u8();
        }

        //@todo new error without line information!?
        mesh.try_add_connection(
            VId { val: a as usize },
            VId { val: b as usize },
            VId { val: c as usize },
        )
        .map_err(|_| PlyError::InvalidMeshIndices(0))?;
    }

    Ok(())
}

fn load_ply_ascii<EM, P, R>(
    read: &mut R,
    mesh: &mut EM,
    header: &PlyHeader,
    line_buffer: &mut String,
    i_line: &mut usize,
) -> PlyResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    loop {
        line_buffer.clear();
        let n_read = read.read_line(line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        *i_line += 1;

        if header.n_vertices > mesh.num_vertices() {
            let mut words = line.split(" ").skip_empty_string();
            for _ in 0..header.vertex_format.before.words {
                words.next();
            }
            let x = f64::from_str(words.next().unwrap()).unwrap(); //@todo unwrap
            for _ in 0..header.vertex_format.between_x_y.words {
                words.next();
            }
            let y = f64::from_str(words.next().unwrap()).unwrap(); //@todo unwrap
            for _ in 0..header.vertex_format.between_y_z.words {
                words.next();
            }
            let z = f64::from_str(words.next().unwrap()).unwrap(); //@todo unwrap
                                                                   // no need to skip 'after' since we're done with this line anyway

            mesh.add_vertex(P::new(x, y, z));
            continue;
        }

        if header.n_faces > mesh.num_faces() {
            let [a, b, c] = collect_index_line(&line).ok_or(PlyError::LineParse(*i_line))?;
            mesh.try_add_connection(VId { val: a }, VId { val: b }, VId { val: c })
                .map_err(|_| PlyError::InvalidMeshIndices(*i_line))?;
            continue;
        }
    }

    if header.n_vertices != mesh.num_vertices() {
        return Err(PlyError::LoadVertexCountIncorrect);
    }

    Ok(())
}

fn collect_index_line(line: &str) -> Option<[usize; 3]> {
    let mut words = to_words(line);
    if words.next()? != "3" {
        return None;
    }

    let a = usize::from_str(words.next()?).ok()?;
    let b = usize::from_str(words.next()?).ok()?;
    let c = usize::from_str(words.next()?).ok()?;

    Some([a, b, c])
}
