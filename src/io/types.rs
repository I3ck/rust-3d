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

//! Module for types used for IO actions

use std::{
    fmt::{Display, Formatter},
    result::Result,
};

//------------------------------------------------------------------------------

/// Trait for adding line information to (error) types
pub trait LineInfoResult<T, E>: Sized {
    fn simple(self) -> Result<T, WithLineInfo<E>>;
    fn index(self, i: usize) -> Result<T, WithLineInfo<E>>;
    fn line(self, i: usize, line: &[u8]) -> Result<T, WithLineInfo<E>>;
}

//------------------------------------------------------------------------------

pub enum DataReserve<T> {
    Data(T),
    Reserve(usize),
}
//@todo implement From<T> and use

//------------------------------------------------------------------------------

pub enum FaceData<T> {
    Face([usize; 3]), //@todo VId or usize?
    Data(T),
}
//@todo implement From<T> and use

//------------------------------------------------------------------------------

pub enum FaceDataReserve<T> {
    Face([usize; 3]), //@todo VId or usize?
    Data(T),
    ReserveDataFaces(usize, usize),
}

impl<T> From<FaceData<T>> for FaceDataReserve<T> {
    fn from(x: FaceData<T>) -> Self {
        match x {
            FaceData::Data(x) => Self::Data(x),
            FaceData::Face(x) => Self::Face(x),
        }
    }
}

impl<T> From<DataReserve<T>> for FaceDataReserve<T> {
    fn from(x: DataReserve<T>) -> Self {
        match x {
            DataReserve::Data(x) => Self::Data(x),
            DataReserve::Reserve(n_d) => Self::ReserveDataFaces(n_d, 0),
        }
    }
}

//------------------------------------------------------------------------------

/// Wrapper type with additional line information
#[derive(Debug)]
pub enum WithLineInfo<T> {
    None(T),
    Index(usize, T),
    Line(usize, String, T),
}

//------------------------------------------------------------------------------

impl<T> Display for WithLineInfo<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::None(x) => write!(f, "{}", x),
            Self::Index(i, x) => write!(f, "Line #{}: '{}'", i, x),
            Self::Line(i, l, x) => write!(f, "Line #{}: '{}' '{}'", i, x, l),
        }
    }
}

//------------------------------------------------------------------------------

/// Result type for errors with additional line information
pub type IOResult<T, E> = Result<T, WithLineInfo<E>>;

impl<T, E> LineInfoResult<T, E> for Result<T, E> {
    fn simple(self) -> Result<T, WithLineInfo<E>> {
        self.map_err(|e| WithLineInfo::None(e))
    }
    fn index(self, i: usize) -> Result<T, WithLineInfo<E>> {
        self.map_err(|e| WithLineInfo::Index(i, e))
    }
    fn line(self, i: usize, line: &[u8]) -> Result<T, WithLineInfo<E>> {
        self.map_err(|e| WithLineInfo::Line(i, String::from_utf8_lossy(line).to_string(), e))
    }
}
