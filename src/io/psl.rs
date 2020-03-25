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

//! Module for IO operations of the psl file format

use crate::*;

use std::{
    fmt,
    io::{Error as ioError, Read},
};

fn read_i32_le<R>(read: &mut R) -> std::io::Result<i32>
where
    R: Read,
{
    let mut buffer = [0u8; 4];
    read.read_exact(&mut buffer)?;
    Ok(i32::from_le_bytes(buffer))
}

fn read_f32_le<R>(read: &mut R) -> std::io::Result<f32>
where
    R: Read,
{
    let mut buffer = [0u8; 4];
    read.read_exact(&mut buffer)?;
    Ok(f32::from_le_bytes(buffer))
}

//------------------------------------------------------------------------------

/// Loads a IsPushable<Is3D> as x y z coordinates from .psl files
pub fn load_psl<IP, P, R>(read: &mut R, ip: &mut IP) -> PslResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: Read,
{
    // header
    {
        //@todo ensure buffer reads "PSLF"?
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
    }

    let _version = read_i32_le(read)?;

    // comments
    {
        let mut buffer = [0u8; 128];
        read.read_exact(&mut buffer)?;
    }

    let n_passes = read_i32_le(read)?;

    let _digitizing_vector_flag = read_i32_le(read)?;

    // reserved 92*i32
    {
        let mut buffer = [0u8; 368];
        read.read_exact(&mut buffer)?;
    }

    for _ in 0..n_passes {
        let n_lines = read_i32_le(read)?;
        let _scanner_id = read_i32_le(read)?;

        // reserved 14*i32
        {
            let mut buffer = [0u8; 56];
            read.read_exact(&mut buffer)?;
        }

        for _ in 0..n_lines {
            let n_points = read_i32_le(read)?;

            // ijk 3*f32
            {
                let mut buffer = [0u8; 12];
                read.read_exact(&mut buffer)?;
            }

            // reserved 12*i32
            {
                let mut buffer = [0u8; 48];
                read.read_exact(&mut buffer)?;
            }

            for _ in 0..n_points {
                let x = read_f32_le(read)?;
                let y = read_f32_le(read)?;
                let z = read_f32_le(read)?;

                ip.push(P::new(x as f64, y as f64, z as f64));
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .psl file operations
pub enum PslError {
    AccessFile,
}

/// Result type for .psl file operations
pub type PslResult<T> = std::result::Result<T, PslError>;

impl fmt::Debug for PslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
        }
    }
}

impl From<ioError> for PslError {
    fn from(_error: ioError) -> Self {
        PslError::AccessFile
    }
}
