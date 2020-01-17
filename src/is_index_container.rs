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

//! IsIndexContainer trait for containers holding indices

/// IsIndexContainer trait for containers holding indices

pub trait IsIndexContainer: Clone + Default {
    /// Should reserve space for n more elements
    fn reserve(&mut self, n: usize);

    /// Should return the number of elements
    fn len(&self) -> usize;

    /// Should return the element at index
    fn get(&self, index: usize) -> usize;

    /// Should overwrite the element at index with value
    fn set(&mut self, index: usize, value: usize);

    /// Should push value to the end of the container
    fn push(&mut self, value: usize);

    /// Should return an iterator over the values
    fn iter(&self) -> IsIndexContainerIterator<Self>;
}

pub struct IsIndexContainerIterator<'a, IC>
where
    IC: IsIndexContainer,
{
    parent: &'a IC,
    max: usize,
    index: usize,
}

impl<'a, IC> IsIndexContainerIterator<'a, IC>
where
    IC: IsIndexContainer,
{
    pub fn new(parent: &'a IC) -> Self {
        Self {
            parent,
            max: parent.len(),
            index: 0,
        }
    }
}

impl<'a, IC> Iterator for IsIndexContainerIterator<'a, IC>
where
    IC: IsIndexContainer,
{
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = if self.index < self.max {
            Some(self.parent.get(self.index))
        } else {
            None
        };

        self.index += 1;

        result
    }
}
