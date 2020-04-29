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

//! Module for IO operations of the .pts file format

use crate::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError},
};

use super::utils::*;

//------------------------------------------------------------------------------

/// Loads IsPushable<Is3D> from the .pts file format
pub fn load_pts<IP, P, R>(read: &mut R, ip: &mut IP) -> PtsResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = Vec::new();
    let mut i_line = 0;

    let mut n_vertices = None;
    let mut n_added = 0;

    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;

        if line.is_empty() {
            continue;
        }

        match n_vertices {
            None => {
                //@todo code duplication
                let mut words = to_words_skip_empty(line);
                n_vertices = Some(
                    words
                        .next()
                        .and_then(|word| from_ascii(word))
                        .ok_or(PtsError::LineParse(i_line))?,
                );
                ip.reserve(n_vertices.unwrap());
            }
            Some(n) => {
                if n_added < n {
                    let mut words = to_words_skip_empty(line);

                    let x = words
                        .next()
                        .and_then(|word| from_ascii(word))
                        .ok_or(PtsError::LineParse(i_line))?;

                    let y = words
                        .next()
                        .and_then(|word| from_ascii(word))
                        .ok_or(PtsError::LineParse(i_line))?;

                    let z = words
                        .next()
                        .and_then(|word| from_ascii(word))
                        .ok_or(PtsError::LineParse(i_line))?;

                    ip.push(P::new(x, y, z));
                    n_added += 1;
                } else {
                    // New block
                    //@todo code duplication
                    n_added = 0;
                    let mut words = to_words_skip_empty(line);
                    n_vertices = Some(
                        words
                            .next()
                            .and_then(|word| from_ascii(word))
                            .ok_or(PtsError::LineParse(i_line))?,
                    );
                    ip.reserve(n_vertices.unwrap());
                }
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

/// Error type for .pts file operations
pub enum PtsError {
    AccessFile,
    LineParse(usize),
}

/// Result type for .pts file operations
pub type PtsResult<T> = std::result::Result<T, PtsError>;

impl fmt::Debug for PtsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AccessFile => write!(f, "Unable to access file"),
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
        }
    }
}

impl From<ioError> for PtsError {
    fn from(_error: ioError) -> Self {
        PtsError::AccessFile
    }
}
