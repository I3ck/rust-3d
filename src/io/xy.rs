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

//! Module for IO of the xy file format

use crate::*;

use core::str::FromStr;
use std::io::{BufRead, Write};

/// Saves an IsRandomAccessible<Is2D> as x y coordinates with a specified delimiter between coordinates and positions. E.g. used to create the .xy file format or .csv files
pub fn save_xy<RA, P, W>(write: &mut W, ra: &RA, delim_coord: &str, delim_pos: &str) -> XyResult<()>
where
    RA: IsRandomAccessible<P>,
    P: Is2D,
    W: Write,
{
    let n = ra.len();
    for i in 0..n {
        let ref p = ra[i];
        let buffer = p.x().to_string() + delim_coord + &p.y().to_string() + delim_pos;
        write.write_all(buffer.as_bytes())?;
    }
    Ok(())
}

/// Loads a IsPushable<Is2D> as x y coordinates. E.g. used to load the .xy file format or .csv files
pub fn load_xy<IP, P, R>(read: &mut R, ip: &mut IP) -> XyResult<()>
where
    IP: IsPushable<P>,
    P: Is2D + IsBuildable2D,
    R: BufRead,
{
    let mut delim_determined = false;
    let mut delim: String = "".to_string();
    let mut line_buffer = String::new();

    let mut i_line = 0;

    loop {
        line_buffer.clear();
        let n_read = read.read_line(&mut line_buffer)?;
        if n_read == 0 {
            break;
        }
        let line = line_buffer.trim_end();
        i_line += 1;

        if !delim_determined {
            delim = estimate_delimiter(1, &line)
                .ok_or(XyError::LineParse(i_line))?
                .to_string();
            delim_determined = true;
        }

        let mut words = line.split(&delim);

        let x = f64::from_str(words.next().ok_or(XyError::LineParse(i_line))?)
            .map_err(|_| XyError::LineParse(i_line))?;

        let y = f64::from_str(words.next().ok_or(XyError::LineParse(i_line))?)
            .map_err(|_| XyError::LineParse(i_line))?;

        ip.push(P::new(x, y));
    }

    Ok(())
}
