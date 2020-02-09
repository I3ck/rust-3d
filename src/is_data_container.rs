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

//! IsDataContainer trait used for data containers allowing for different storage types than using the abstracted type

/// IsDataContainer trait used for data containers allowing for different storage types than using the abstracted type
pub trait IsDataContainer<T> {
    /// Should reserve space for n elements of type T
    fn reserve_d(&mut self, n: usize);
    /// Should return the number of T elements in the container
    fn len_d(&self) -> usize;
    /// Should push a T to the end of the container
    fn push_d(&mut self, x: T);
    /// Should return the element at index
    fn get_d(&self, index: usize) -> T;
    /// Should set the element at index
    fn set_d(&mut self, index: usize, x: T);
}
