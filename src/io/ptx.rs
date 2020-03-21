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

//! Module for IO operations of the ptx file format

use crate::*;

use std::io::BufRead;

use core::str::FromStr;

/// Loads points from .ptx file into IsPushable<Is3D>
pub fn load_ptx<IP, P, R>(read: &mut R, ip: &mut IP) -> PtxResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + IsMatrix4Transformable,
    R: BufRead,
{
    let mut i_line = 0;
    let mut line_buffer = String::new();

    let mut line: &str;

    loop {
        let columns: usize;
        {
            let first_line = fetch_line(read, &mut line_buffer);
            if first_line.is_err() {
                break;
            }
            i_line += 1;

            columns =
                usize::from_str(first_line.unwrap()).map_err(|_| PtxError::Columns(i_line))?;
            // safe, since first_line being err causing break
        }

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        let rows = usize::from_str(line).map_err(|_| PtxError::Rows(i_line))?;

        // skip scanner position line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        // skip scanner x-axis line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        // skip scanner y-axis line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        // skip scanner z-axis line
        fetch_line(read, &mut line_buffer)?;
        i_line += 1;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m11, m12, m13, m14] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m21, m22, m23, m24] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m31, m32, m33, m34] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        line = fetch_line(read, &mut line_buffer)?;
        i_line += 1;
        let [m41, m42, m43, m44] = read_matrix_row(line).ok_or(PtxError::Matrix(i_line))?;

        let m = Matrix4 {
            data: [
                [m11, m12, m13, m14],
                [m21, m22, m23, m24],
                [m31, m32, m33, m34],
                [m41, m42, m43, m44],
            ],
        };

        let must_transform = m != Matrix4::identity();

        for _ in 0..columns * rows {
            line = fetch_line(read, &mut line_buffer)?;
            i_line += 1;

            let mut words = to_words(line);

            let x = f64::from_str(words.next().ok_or(PtxError::Point(i_line))?)
                .map_err(|_| PtxError::Point(i_line))?;
            let y = f64::from_str(words.next().ok_or(PtxError::Point(i_line))?)
                .map_err(|_| PtxError::Point(i_line))?;
            let z = f64::from_str(words.next().ok_or(PtxError::Point(i_line))?)
                .map_err(|_| PtxError::Point(i_line))?;

            let mut p = P::new(x, y, z);

            if must_transform {
                p.transform(&m)
            }
            ip.push(p)
        }
    }

    Ok(())
}

//@todo duplicate code
fn fetch_line<'a, R>(read: &mut R, line_buffer: &'a mut String) -> PtxResult<&'a str>
where
    R: BufRead,
{
    line_buffer.clear();
    let n_read = read.read_line(line_buffer)?;
    if n_read == 0 {
        return Err(PtxError::LoadFileEndReached);
    }
    Ok(line_buffer.trim_end())
}

fn read_matrix_row(line: &str) -> Option<[f64; 4]>
{
    let mut words = to_words(line);

    let a = f64::from_str(words.next()?).ok()?;
    let b = f64::from_str(words.next()?).ok()?;
    let c = f64::from_str(words.next()?).ok()?;
    let d = f64::from_str(words.next()?).ok()?;

    Some([a, b, c, d])
}
