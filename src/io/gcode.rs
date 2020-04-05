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

//! Module for IO of the gcode file format

use crate::*;

use std::{
    fmt,
    io::{BufRead, Error as ioError},
};

use super::utils::*;

//------------------------------------------------------------------------------

/// Loads a IsPushable<Is3D> as x y z coordinates from gcode
pub fn load_gcode_points<IP, P, R>(read: &mut R, ip: &mut IP) -> GcodeResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = Vec::new();

    let mut i_line = 0;

    let mut start_pushed = false;
    let mut ra = RelativeAbsolute::Absolute;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    while let Ok(line) = fetch_line(read, &mut line_buffer) {
        i_line += 1;

        if line.len() >= 4 && line[0] == b'G' {
            if line[2] == b' ' && (line[1] == b'1' || line[1] == b'2' || line[1] == b'3') {
                // Move according to absolute/relative
                let mut any_changed = false;
                //@todo more specific errors
                let [opt_x, opt_y, opt_z] =
                    command(&line[3..]).ok_or(GcodeError::LineParse(i_line))?;

                if let Some(new_x) = opt_x {
                    any_changed = true;
                    match ra {
                        RelativeAbsolute::Absolute => x = new_x,
                        RelativeAbsolute::Relative => x += new_x,
                    }
                }

                if let Some(new_y) = opt_y {
                    any_changed = true;
                    match ra {
                        RelativeAbsolute::Absolute => y = new_y,
                        RelativeAbsolute::Relative => y += new_y,
                    }
                }

                if let Some(new_z) = opt_z {
                    any_changed = true;
                    match ra {
                        RelativeAbsolute::Absolute => z = new_z,
                        RelativeAbsolute::Relative => z += new_z,
                    }
                }

                if any_changed {
                    if !start_pushed {
                        ip.push(P::new(0.0, 0.0, 0.0));
                        start_pushed = true
                    }
                    ip.push(P::new(x, y, z));
                }
            } else if line[1] == b'9' && line[3] == b' ' {
                // G9x
                if line[2] == b'0' {
                    // G90
                    ra = RelativeAbsolute::Absolute;
                } else if line[2] == b'1' {
                    // G91
                    ra = RelativeAbsolute::Relative;
                } else if line[2] == b'2' {
                    // G92
                    // Move according absolute
                    let mut any_changed = false;
                    //@todo more specific error
                    let [opt_x, opt_y, opt_z] =
                        command(&line[4..]).ok_or(GcodeError::LineParse(i_line))?;

                    if let Some(new_x) = opt_x {
                        any_changed = true;
                        x = new_x
                    }

                    if let Some(new_y) = opt_y {
                        any_changed = true;
                        y = new_y
                    }

                    if let Some(new_z) = opt_z {
                        any_changed = true;
                        z = new_z
                    }

                    if any_changed {
                        if !start_pushed {
                            ip.push(P::new(0.0, 0.0, 0.0));
                            start_pushed = true
                        }
                        ip.push(P::new(x, y, z));
                    }
                }
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

fn command(line: &[u8]) -> Option<[Option<f64>; 3]> {
    let mut n_found = 0;
    let mut x = None;
    let mut y = None;
    let mut z = None;
    let words = line.split(|x| *x == b' ');

    for word in words {
        if n_found == 3 {
            break;
        }

        let n = word.len();

        if n == 0 {
            continue;
        }

        if word[0] == b';' {
            break;
        }

        if n < 2 {
            continue;
        }

        match word[0] {
            b'X' => {
                x = Some(from_ascii(&word[1..])?);
                n_found += 1
            }
            b'Y' => {
                y = Some(from_ascii(&word[1..])?);
                n_found += 1
            }
            b'Z' => {
                z = Some(from_ascii(&word[1..])?);
                n_found += 1
            }
            _ => (),
        }
    }

    Some([x, y, z])
}

enum RelativeAbsolute {
    Relative,
    Absolute,
}

/// Error type for .gcode file operations
pub enum GcodeError {
    AccessFile,
    LineParse(usize),
}

/// Result type for .gcode file operations
pub type GcodeResult<T> = std::result::Result<T, GcodeError>;

impl fmt::Debug for GcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LineParse(x) => write!(f, "Unable to parse line {}", x),
            Self::AccessFile => write!(f, "Unable to access file"),
        }
    }
}

impl From<ioError> for GcodeError {
    fn from(_error: ioError) -> Self {
        GcodeError::AccessFile
    }
}
