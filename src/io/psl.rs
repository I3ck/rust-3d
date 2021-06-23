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

use std::{io::Read, iter::FusedIterator, marker::PhantomData};

use super::{byte_reader::*, types::*};

//------------------------------------------------------------------------------

pub struct PslIterator<P, R, const CHUNK_SIZE: usize>
where
    P: IsBuildable3D + Default,
    R: Read,
{
    read: R,
    is_done: bool,
    is_initialized: bool,
    n_passes_left: i32,
    n_lines_left: i32,
    n_point_left: i32,
    phantom: PhantomData<P>,
}

impl<P, R, const CHUNK_SIZE: usize> PslIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
{
    pub fn new(read: R) -> Self {
        Self {
            read,
            is_done: false,
            is_initialized: false,
            n_passes_left: 0,
            n_lines_left: 0,
            n_point_left: 0, //@todo pointS
            phantom: PhantomData,
        }
    }
    #[inline(always)]
    fn initialize(&mut self) -> IOResult<()> {
        self.n_passes_left = fetch_header_return_n_passes(&mut self.read)?;
        Ok(())
    }
    #[inline(always)]
    fn fetch_counts(&mut self) -> IOResult<()> {
        self.n_lines_left = if self.n_lines_left == 0 && self.n_passes_left > 0 {
            fetch_pass_header_return_n_lines(&mut self.read)?
        } else {
            0
        };
        self.n_point_left = if self.n_point_left == 0 && self.n_lines_left > 0 {
            fetch_line_header_return_n_points(&mut self.read)?
        } else {
            0
        };
        Ok(())
    }
    #[inline(always)]
    fn reduce_count(&mut self) {
        if self.n_point_left > 0 {
            self.n_point_left -= 1
        } else {
            if self.n_lines_left > 0 {
                self.n_lines_left -= 1
            } else {
                if self.n_passes_left > 0 {
                    self.n_passes_left -= 1
                }
            }
        }
    }
}

impl<P, R, const CHUNK_SIZE: usize> Iterator for PslIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
{
    type Item = IOResult<StackVec<DataReserve<P>, CHUNK_SIZE>>;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        if !self.is_initialized {
            self.is_initialized = true;
            if let Err(e) = self.initialize() {
                self.is_done = true;
                return Some(Err(e));
            }
        }

        let mut chunk = StackVec::default();

        loop {
            if chunk.is_full() {
                return Some(Ok(chunk));
            } else if let Err(e) = self.fetch_counts() {
                self.is_done = true;
                return Some(Err(e));
            } else if self.n_point_left == 0 {
                self.is_done = true;
                if chunk.has_data() {
                    return Some(Ok(chunk));
                }
                return None;
            }

            self.reduce_count();

            match fetch_point(&mut self.read) {
                Err(e) => {
                    self.is_done = true;
                    return Some(Err(e));
                }
                Ok(x) => chunk.push(DataReserve::Data(x)).unwrap(), // unwrap safe since we only call this if chunk.has_space()
            }
        }
    }
}

impl<P, R, const CHUNK_SIZE: usize> FusedIterator for PslIterator<P, R, CHUNK_SIZE>
where
    P: IsBuildable3D + Default,
    R: Read,
{
}

//------------------------------------------------------------------------------

/// Loads a IsPushable<Is3D> as x y z coordinates from .psl files
pub fn load_psl<IP, P, R, const CHUNK_SIZE: usize>(read: R, ip: &mut IP) -> IOResult<()>
where
    IP: IsPushable<P>,
    P: IsBuildable3D + Default,
    R: Read,
{
    let iterator = PslIterator::<_, _, CHUNK_SIZE>::new(read);

    for chunk in iterator {
        for x in chunk? {
            match x {
                DataReserve::Data(x) => ip.push(x),
                DataReserve::Reserve(x) => ip.reserve(x),
                DataReserve::ReserveExact(x) => ip.reserve_exact(x),
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_header_return_n_passes<R>(read: &mut R) -> IOResult<i32>
where
    R: Read,
{
    // header
    {
        let mut buffer = [0u8; 4];
        read.read_exact(&mut buffer)?;
    }

    let _version = LittleReader::read_i32(read)?;

    // comments
    {
        let mut buffer = [0u8; 128];
        read.read_exact(&mut buffer)?;
    }

    let n_passes = LittleReader::read_i32(read)?;

    let _digitizing_vector_flag = LittleReader::read_i32(read)?;

    // reserved 92*i32
    {
        let mut buffer = [0u8; 368];
        read.read_exact(&mut buffer)?;
    }

    Ok(n_passes)
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_pass_header_return_n_lines<R>(read: &mut R) -> IOResult<i32>
where
    R: Read,
{
    let n_lines = LittleReader::read_i32(read)?;
    let _scanner_id = LittleReader::read_i32(read)?;

    // reserved 14*i32
    {
        let mut buffer = [0u8; 56];
        read.read_exact(&mut buffer)?;
    }

    Ok(n_lines)
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_line_header_return_n_points<R>(read: &mut R) -> IOResult<i32>
where
    R: Read,
{
    let n_points = LittleReader::read_i32(read)?;

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

    Ok(n_points)
}

//------------------------------------------------------------------------------

#[inline(always)]
fn fetch_point<R, P>(read: &mut R) -> IOResult<P>
where
    R: Read,
    P: IsBuildable3D,
{
    let x = LittleReader::read_f32(read)?;
    let y = LittleReader::read_f32(read)?;
    let z = LittleReader::read_f32(read)?;

    Ok(P::new(x as f64, y as f64, z as f64))
}
