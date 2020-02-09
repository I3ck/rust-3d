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

//! U32IndexVec is a u32 container with a usize interface (make sure your use case doesn't go out of bounds)

use crate::{IsIndexContainer, IsIndexContainerIterator};

//------------------------------------------------------------------------------

#[derive(Clone, Default)]
/// U32IndexVec is a u32 container with a usize interface (make sure your use case doesn't go out of bounds)
pub struct U32IndexVec {
    pub data: Vec<u32>,
}

impl IsIndexContainer for U32IndexVec {
    fn ensure_supported(&mut self, _x: usize) {
        //@todo nothing we can do here?!
    }

    fn get(&self, i: usize) -> usize {
        self.data[i] as usize
    }

    fn set(&mut self, i: usize, x: usize) {
        self.data[i] = x as u32
    }

    fn push(&mut self, x: usize) {
        self.data.push(x as u32)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn reserve(&mut self, n: usize) {
        self.data.reserve(n)
    }

    fn iter(&self) -> IsIndexContainerIterator<Self> {
        IsIndexContainerIterator::new(self)
    }
}
