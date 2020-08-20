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

//! Iterator that skips empty elements

/// Iterator that skips empty elements
#[derive(Clone)]
pub struct SkipEmpty<'a, T, I>
where
    T: 'a,
    I: Iterator<Item = &'a [T]>,
{
    iterator: I,
}

impl<'a, T, I> SkipEmpty<'a, T, I>
where
    T: 'a,
    I: Iterator<Item = &'a [T]>,
{
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

impl<'a, T, I> Iterator for SkipEmpty<'a, T, I>
where
    T: 'a,
    I: Iterator<Item = &'a [T]>,
{
    type Item = &'a [T];

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iterator.next() {
                None => return None,
                Some([]) => continue,
                x @ Some(_) => return x,
            }
        }
    }
}

//------------------------------------------------------------------------------

/// Utility trait to easily spawn the SkipEmpty iterator
pub trait IsSkipEmptyProducer<'a, T>: Iterator<Item = &'a [T]>
where
    T: 'a,
    Self: Sized,
{
    fn skip_empty(self) -> SkipEmpty<'a, T, Self>;
}

impl<'a, T, F> IsSkipEmptyProducer<'a, T> for std::slice::Split<'a, T, F>
where
    F: FnMut(&T) -> bool,
{
    fn skip_empty(self) -> SkipEmpty<'a, T, Self> {
        SkipEmpty::new(self)
    }
}
