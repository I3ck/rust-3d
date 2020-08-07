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

use std::result::Result;

//------------------------------------------------------------------------------

/// Trait for adding line information to (error) types
pub trait LineInfoResult<T, E>: Sized {
    fn simple(self) -> Result<T, WithLineInfo<E>>;
    fn index(self, i: usize) -> Result<T, WithLineInfo<E>>;
    fn line(self, i: usize, line: &[u8]) -> Result<T, WithLineInfo<E>>;
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
