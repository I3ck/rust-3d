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

//@todo better error handling instead of yielding partial data
//@todo _mesh / _triplet code duplication

use crate::*;

use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{BufRead, Read, Write};

use fnv::FnvHashMap;

use core::str::FromStr;

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

/// Loads a Mesh from .stl file with duplicate vertices
pub fn load_stl_mesh_duped<EM, P, R>(
    read: &mut R,
    format: StlFormat,
    mesh: &mut EM,
) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    if is_ascii(read, format)? {
        load_stl_mesh_duped_ascii(read, mesh)
    } else {
        load_stl_mesh_duped_binary(read, mesh)
    }
}

/// Loads a Mesh from .stl file with unique vertices
pub fn load_stl_mesh_unique<EM, P, R>(
    read: &mut R,
    format: StlFormat,
    mesh: &mut EM,
) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    if is_ascii(read, format)? {
        load_stl_mesh_unique_ascii(read, mesh)
    } else {
        load_stl_mesh_unique_binary(read, mesh)
    }
}

/// Loads points from .stl file as triplets into IsPushable<Is3D>
pub fn load_stl_triplets<IP, P, R>(read: &mut R, format: StlFormat, ip: &mut IP) -> StlResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    if is_ascii(read, format)? {
        load_stl_triplets_ascii(read, ip)
    } else {
        load_stl_triplets_binary(read, ip)
    }
}

fn is_ascii<R>(read: &mut R, format: StlFormat) -> StlResult<bool>
where
    R: BufRead,
{
    let solid = "solid".as_bytes();

    let mut result = true;
    for i in 0..5 {
        if read.read_u8()? != solid[i] {
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

fn load_stl_mesh_duped_ascii<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut i_line = 0;
    let mut line_buffer = String::new();

    // skip first line
    read.read_line(&mut line_buffer)?;
    i_line += 1;

    loop {
        match read_stl_facet(read, &mut line_buffer, &mut i_line) {
            Ok([a, b, c]) => {
                mesh.add_face(a, b, c);
                ()
            }
            Err(StlError::LoadFileEndReached) => break,
            Err(x) => return Err(x),
        }
    }

    Ok(())
}

fn load_stl_mesh_duped_binary<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
{
    // Drop header ('solid' is already dropped)
    {
        let mut buffer = [0u8; 75];
        read.read_exact(&mut buffer)?;
    }

    let n_triangles = read.read_u32::<LittleEndian>()?;
    mesh.reserve_vertices(3 * n_triangles as usize);
    mesh.reserve_faces(n_triangles as usize);

    let mut buffer = [0f32; 3];

    for _ in 0..n_triangles {
        // Drop normal
        read.read_f32_into::<LittleEndian>(&mut buffer)?;

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let a = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let b = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let c = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_u16::<LittleEndian>()?;

        mesh.add_face(a, b, c);
    }

    Ok(())
}

fn load_stl_mesh_unique_ascii<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let mut i_line = 0;
    let mut line_buffer = String::new();

    // skip first line
    read.read_line(&mut line_buffer)?;
    i_line += 1;

    let mut map = FnvHashMap::default();

    loop {
        match read_stl_facet::<P, _>(read, &mut line_buffer, &mut i_line) {
            Ok([a, b, c]) => {
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

                mesh.try_add_connection(VId { val: id_a }, VId { val: id_b }, VId { val: id_c })
                    .unwrap(); // safe since added above
                ()
            }
            Err(StlError::LoadFileEndReached) => break,
            Err(x) => return Err(x),
        }
    }

    Ok(())
}

fn load_stl_mesh_unique_binary<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: Read,
{
    // Drop header ('solid' is already dropped)
    {
        let mut buffer = [0u8; 75];
        read.read_exact(&mut buffer)?;
    }

    let n_triangles = read.read_u32::<LittleEndian>()?;
    mesh.reserve_vertices((0.5 * n_triangles as f64) as usize);
    mesh.reserve_faces(n_triangles as usize);

    let mut buffer = [0f32; 3];

    //@todo FnvHashMap?
    let mut map = FnvHashMap::default();

    for _ in 0..n_triangles {
        // Drop normal
        read.read_f32_into::<LittleEndian>(&mut buffer)?;

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let a = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let b = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let c = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_u16::<LittleEndian>()?;

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

        mesh.try_add_connection(VId { val: id_a }, VId { val: id_b }, VId { val: id_c })
            .unwrap(); // safe since added above
    }

    Ok(())
}

pub fn load_stl_triplets_ascii<IP, P, R>(read: &mut R, ip: &mut IP) -> StlResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut i_line = 0;
    let mut line_buffer = String::new();

    // skip first line
    read.read_line(&mut line_buffer)?;
    i_line += 1;

    loop {
        match read_stl_facet(read, &mut line_buffer, &mut i_line) {
            Ok([a, b, c]) => {
                ip.push(a);
                ip.push(b);
                ip.push(c);
                ()
            }
            Err(StlError::LoadFileEndReached) => break,
            Err(x) => return Err(x),
        }
    }

    Ok(())
}

pub fn load_stl_triplets_binary<IP, P, R>(read: &mut R, ip: &mut IP) -> StlResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: Read,
{
    // Drop header ('solid' is already dropped)
    {
        let mut buffer = [0u8; 75];
        read.read_exact(&mut buffer)?;
    }

    let n_triangles = read.read_u32::<LittleEndian>()?;
    let mut buffer = [0f32; 3];

    for _ in 0..n_triangles {
        // Drop normal
        read.read_f32_into::<LittleEndian>(&mut buffer)?;

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let a = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let b = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_f32_into::<LittleEndian>(&mut buffer)?;
        let c = P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64);

        read.read_u16::<LittleEndian>()?;

        ip.push(a);
        ip.push(b);
        ip.push(c);
    }

    Ok(())
}

fn fetch_line<'a, R>(read: &mut R, line_buffer: &'a mut String) -> StlResult<&'a str>
where
    R: BufRead,
{
    line_buffer.clear();
    let n_read = read.read_line(line_buffer)?;
    if n_read == 0 {
        return Err(StlError::LoadFileEndReached);
    }
    Ok(line_buffer.trim_end())
}

fn read_stl_facet<P, R>(
    read: &mut R,
    line_buffer: &mut String,
    i_line: &mut usize,
) -> StlResult<[P; 3]>
where
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line: &str;

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    if line.contains("endsolid") {
        return Err(StlError::LoadFileEndReached);
    }

    if !line.contains("facet") {
        return Err(StlError::LineParse(*i_line));
    }

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    if !line.contains("outer loop") {
        return Err(StlError::LineParse(*i_line));
    }

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    let a = read_stl_vertex(&line).ok_or(StlError::LineParse(*i_line))?;

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    let b = read_stl_vertex(&line).ok_or(StlError::LineParse(*i_line))?;

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    let c = read_stl_vertex(&line).ok_or(StlError::LineParse(*i_line))?;

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    if !line.contains("endloop") {
        return Err(StlError::LineParse(*i_line));
    }

    line = fetch_line(read, line_buffer)?;
    *i_line += 1;

    if !line.contains("endfacet") {
        return Err(StlError::LineParse(*i_line));
    }

    Ok([a, b, c])
}

fn read_stl_vertex<P>(line: &str) -> Option<P>
where
    P: IsBuildable3D,
{
    let mut words = to_words(line);

    words.next()?;

    let x = f64::from_str(words.next()?).ok()?;
    let y = f64::from_str(words.next()?).ok()?;
    let z = f64::from_str(words.next()?).ok()?;

    Some(P::new(x, y, z))
}

fn str_exp<P>(p: &P) -> String
where
    P: Is3D,
{
    format!("{:e} {:e} {:e}", p.x(), p.y(), p.z()).to_string()
}
