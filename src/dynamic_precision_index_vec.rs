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

//! DynamicPrecisionIndexVec is a memory efficient container with a usize interface. It uses u8 -> u16 -> u32 -> usize for storage depending on the largest index's size

use crate::{IsIndexContainer, IsIndexContainerIterator};

use std::{u16, u32, u8, usize};

//------------------------------------------------------------------------------

#[derive(Clone, Default)]
/// DynamicPrecisionIndexVec is a memory efficient container with a usize interface. It uses u8 -> u16 -> u32 -> usize for storage depending on the largest index's size
pub struct DynamicPrecisionIndexVec {
    mode: Mode,
}

//@todo implement as much of the Vec interface as possible
//@todo have constructors such as with_capacity that additionally initialize with the fitting mode
impl DynamicPrecisionIndexVec {
    /// Creates a new DynamicPrecisionIndexVec
    pub fn new() -> Self {
        Self {
            mode: Mode::U8(Vec::new()),
        }
    }
    /// If storage is currently done with u8 precision, returns a reference to the inner Vec
    pub fn get_u8(&self) -> Option<&Vec<u8>> {
        match self.mode {
            Mode::U8(ref vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with u8 precision, returns a mutable reference to the inner Vec
    pub fn get_u8_mut(&mut self) -> Option<&mut Vec<u8>> {
        match self.mode {
            Mode::U8(ref mut vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with u16 precision, returns a reference to the inner Vec
    pub fn get_u16(&self) -> Option<&Vec<u16>> {
        match self.mode {
            Mode::U16(ref vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with u16 precision, returns a mutable reference to the inner Vec
    pub fn get_u16_mut(&mut self) -> Option<&mut Vec<u16>> {
        match self.mode {
            Mode::U16(ref mut vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with u32 precision, returns a reference to the inner Vec
    pub fn get_u32(&self) -> Option<&Vec<u32>> {
        match self.mode {
            Mode::U32(ref vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with u32 precision, returns a mutable reference to the inner Vec
    pub fn get_u32_mut(&mut self) -> Option<&mut Vec<u32>> {
        match self.mode {
            Mode::U32(ref mut vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with usize precision, returns a reference to the inner Vec
    pub fn get_usize(&mut self) -> Option<&Vec<usize>> {
        match self.mode {
            Mode::Usize(ref vec) => Some(vec),
            _ => None,
        }
    }
    /// If storage is currently done with usize precision, returns a mutable reference to the inner Vec
    pub fn get_usize_mut(&mut self) -> Option<&mut Vec<usize>> {
        match self.mode {
            Mode::Usize(ref mut vec) => Some(vec),
            _ => None,
        }
    }
    /// First promotes the precision to usize, then returns a reference to the Vec<usize>. Calling this might unnecessarily increase memory usage
    pub fn get_usize_upgraded(&mut self) -> &Vec<usize> {
        self.upgrade_to_usize();
        match self.mode {
            Mode::Usize(ref vec) => vec,
            _ => panic!("Logic error in get_usize_upgraded of DynamicPrecisionIndexVec"),
        }
    }
    /// First promotes the precision to usize, then returns a mutable reference to the Vec<usize>. Calling this might unnecessarily increase memory usage
    pub fn get_usize_upgraded_mut(&mut self) -> &mut Vec<usize> {
        self.upgrade_to_usize();
        match self.mode {
            Mode::Usize(ref mut vec) => vec,
            _ => panic!("Logic error in get_usize_upgraded_mut of DynamicPrecisionIndexVec"),
        }
    }

    fn ensure_upgraded(&mut self, x: usize) {
        if x <= self.allowed_max() {
            return;
        }

        if x > u32::MAX as usize {
            self.upgrade_to_usize();
        } else if x > u16::MAX as usize {
            self.upgrade_to_u32();
        } else if x > u8::MAX as usize {
            self.upgrade_to_u16();
        }
    }

    fn upgrade_to_usize(&mut self) {
        if let Some(new_mode) = match self.mode {
            Mode::U8(ref vec) => {
                let new = vec.iter().map(|x| *x as usize).collect();
                Some(Mode::Usize(new))
            }
            Mode::U16(ref vec) => {
                let new = vec.iter().map(|x| *x as usize).collect();
                Some(Mode::Usize(new))
            }
            Mode::U32(ref vec) => {
                let new = vec.iter().map(|x| *x as usize).collect();
                Some(Mode::Usize(new))
            }
            Mode::Usize(_) => None,
        } {
            self.mode = new_mode;
        }
    }

    fn upgrade_to_u32(&mut self) {
        if let Some(new_mode) = match self.mode {
            Mode::U8(ref vec) => {
                let new = vec.iter().map(|x| *x as u32).collect();
                Some(Mode::U32(new))
            }
            Mode::U16(ref vec) => {
                let new = vec.iter().map(|x| *x as u32).collect();
                Some(Mode::U32(new))
            }
            Mode::U32(_) => None,
            Mode::Usize(_) => None,
        } {
            self.mode = new_mode;
        }
    }

    fn upgrade_to_u16(&mut self) {
        if let Some(new_mode) = match self.mode {
            Mode::U8(ref vec) => {
                let new = vec.iter().map(|x| *x as u16).collect();
                Some(Mode::U16(new))
            }
            Mode::U16(_) => None,
            Mode::U32(_) => None,
            Mode::Usize(_) => None,
        } {
            self.mode = new_mode;
        }
    }

    fn allowed_max(&self) -> usize {
        match self.mode {
            Mode::U8(_) => u8::MAX as usize,
            Mode::U16(_) => u16::MAX as usize,
            Mode::U32(_) => u32::MAX as usize,
            Mode::Usize(_) => usize::MAX,
        }
    }
}

impl IsIndexContainer for DynamicPrecisionIndexVec {
    fn get(&self, i: usize) -> usize {
        match self.mode {
            Mode::U8(ref vec) => vec[i] as usize,
            Mode::U16(ref vec) => vec[i] as usize,
            Mode::U32(ref vec) => vec[i] as usize,
            Mode::Usize(ref vec) => vec[i] as usize,
        }
    }

    fn set(&mut self, i: usize, x: usize) {
        self.ensure_upgraded(x);

        match self.mode {
            Mode::U8(ref mut vec) => vec[i] = x as u8,
            Mode::U16(ref mut vec) => vec[i] = x as u16,
            Mode::U32(ref mut vec) => vec[i] = x as u32,
            Mode::Usize(ref mut vec) => vec[i] = x,
        }
    }

    fn push(&mut self, x: usize) {
        self.ensure_upgraded(x);

        match self.mode {
            Mode::U8(ref mut vec) => vec.push(x as u8),
            Mode::U16(ref mut vec) => vec.push(x as u16),
            Mode::U32(ref mut vec) => vec.push(x as u32),
            Mode::Usize(ref mut vec) => vec.push(x),
        }
    }

    fn len(&self) -> usize {
        match self.mode {
            Mode::U8(ref vec) => vec.len(),
            Mode::U16(ref vec) => vec.len(),
            Mode::U32(ref vec) => vec.len(),
            Mode::Usize(ref vec) => vec.len(),
        }
    }

    fn reserve(&mut self, n: usize) {
        match self.mode {
            Mode::U8(ref mut vec) => vec.reserve(n),
            Mode::U16(ref mut vec) => vec.reserve(n),
            Mode::U32(ref mut vec) => vec.reserve(n),
            Mode::Usize(ref mut vec) => vec.reserve(n),
        }
    }

    fn iter(&self) -> IsIndexContainerIterator<Self> {
        IsIndexContainerIterator::new(self)
    }
}

impl From<&Vec<usize>> for DynamicPrecisionIndexVec {
    //@todo also implement for u8 u16 u32
    fn from(vec: &Vec<usize>) -> Self {
        let mut result = Self::new(); //@todo reserver here and more optimizations
        result.reserve(vec.len());
        for x in vec.iter() {
            result.push(x) //@todo consider iterating first to find the maximum and set mode according to it
        }

        result
    }
}

impl From<Vec<usize>> for DynamicPrecisionIndexVec {
    //@todo also implement for u8 u16 u32
    fn from(vec: Vec<usize>) -> Self {
        Self::from(&vec)
    }
}

//------------------------------------------------------------------------------

#[derive(Clone)]
enum Mode {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    Usize(Vec<usize>),
}

impl Default for Mode {
    fn default() -> Self {
        Self::U8(Vec::new())
    }
}
