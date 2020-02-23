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

use crate::*;

use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{BufRead, Read, Write};

use core::str::FromStr;

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

pub fn load_stl<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
where
    EM: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3>,
    P: IsBuildable3D + Clone,
    R: BufRead,
{
    let solid = "solid".as_bytes();

    let mut is_ascii = true;
    for i in 0..5 {
        if read.read_u8()? != solid[i] {
            is_ascii = false
        }
    }

    if is_ascii {
        load_stl_ascii(read, mesh)
    } else {
        load_stl_binary(read, mesh)
    }
}

/// Loads a Mesh from ASCII .stl files (assuming 'solid' already dropped from input)
fn load_stl_ascii<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
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

/// Loads a Mesh from binary .stl files (assuming 'solid' already dropped from input)
fn load_stl_binary<EM, P, R>(read: &mut R, mesh: &mut EM) -> StlResult<()>
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
