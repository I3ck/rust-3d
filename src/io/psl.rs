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

use byteorder::{LittleEndian, ReadBytesExt};

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

    //@todo ensure version is '1'?
    let _version = read.read_i32::<LittleEndian>()?;

    // comments
    {
        let mut buffer = [0u8; 128];
        read.read_exact(&mut buffer)?;
    }

    let n_passes = read.read_i32::<LittleEndian>()?;

    let _digitizing_vector_flag = read.read_i32::<LittleEndian>()?;

    // reserved
    {
        let mut buffer = [0i32; 92];
        read.read_i32_into::<LittleEndian>(&mut buffer)?;
    }

    for _ in 0..n_passes {
        let n_lines = read.read_i32::<LittleEndian>()?;
        let _scanner_id = read.read_i32::<LittleEndian>()?;

        // reserved
        {
            let mut buffer = [0i32; 14];
            read.read_i32_into::<LittleEndian>(&mut buffer)?;
        }

        for _ in 0..n_lines {
            let n_points = read.read_i32::<LittleEndian>()?;

            // ijk
            {
                let mut buffer = [0f32; 3];
                read.read_f32_into::<LittleEndian>(&mut buffer)?;
            }

            // reserved
            {
                let mut buffer = [0i32; 12];
                read.read_i32_into::<LittleEndian>(&mut buffer)?;
            }

            for _ in 0..n_points {
                let mut buffer = [0f32; 3];
                read.read_f32_into::<LittleEndian>(&mut buffer)?;

                ip.push(P::new(buffer[0] as f64, buffer[1] as f64, buffer[2] as f64));
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
