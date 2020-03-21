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

use core::str::FromStr;
use std::{
    fmt,
    io::{BufRead, Error as ioError},
};

//------------------------------------------------------------------------------

//@todo this could be extended to consider way more commands besides G1
//@todo code duplication

/// Loads a IsPushable<Is3D> as x y z coordinates from gcode
pub fn load_gcode_points<IP, P, R>(read: &mut R, ip: &mut IP) -> GcodeResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead,
{
    let mut line_buffer = String::new();

    let mut i_line = 0;

    let mut start_pushed = false;
    let mut ra = RelativeAbsolute::Absolute;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(&mut line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        i_line += 1;

        if line.starts_with("G1 ") {
            let mut any_changed = false;
            //@todo more specific errors
            let [opt_x, opt_y, opt_z] = g1_command(line).ok_or(GcodeError::LineParse(i_line))?;

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
        } else if line.starts_with("G90 ") {
            ra = RelativeAbsolute::Absolute;
        } else if line.starts_with("G91 ") {
            ra = RelativeAbsolute::Relative;
        } else if line.starts_with("G92 ") {
            let mut any_changed = false;
            //@todo more specific error
            let [opt_x, opt_y, opt_z] = g1_command(line).ok_or(GcodeError::LineParse(i_line))?;

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

    Ok(())
}

//------------------------------------------------------------------------------

fn g1_command(line: &str) -> Option<[Option<f64>; 3]> {
    let mut x = None;
    let mut y = None;
    let mut z = None;
    let mut words = to_words(line);

    words.next().unwrap(); // safe since starts_with above ensures at least one word present

    for word in words {
        //@todo faster way to access first char?
        match word.chars().nth(0) {
            Some(';') => break,
            Some('X') => x = Some(f64::from_str(skip_first_char(word)).ok()?),
            Some('Y') => y = Some(f64::from_str(skip_first_char(word)).ok()?),
            Some('Z') => z = Some(f64::from_str(skip_first_char(word)).ok()?),

            Some(_) | None => (),
        }
    }

    Some([x, y, z])
}

//@todo more efficient way?
fn skip_first_char(text: &str) -> &str {
    let mut cs = text.chars();
    cs.next();
    cs.as_str()
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
