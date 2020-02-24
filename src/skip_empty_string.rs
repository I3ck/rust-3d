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

//! Iterator that skips empty strings

/// Iterator that skips empty strings
pub struct SkipEmptyString<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    iterator: I,
}

impl<'a, I> SkipEmptyString<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

impl<'a, I> Iterator for SkipEmptyString<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iterator.next() {
                None => return None,
                Some("") => return None,
                Some(x) => return Some(x),
            }
        }
    }
}

//------------------------------------------------------------------------------

/// Utility trait to easily spawn the SkipEmptyString iterator
pub trait IsSkipEmptyStringProducer<'a>: Iterator<Item = &'a str>
where
    Self: Sized,
{
    fn skip_empty_string(self) -> SkipEmptyString<'a, Self>;
}

impl<'a> IsSkipEmptyStringProducer<'a> for std::str::Split<'a, &str> {
    fn skip_empty_string(self) -> SkipEmptyString<'a, Self> {
        SkipEmptyString::new(self)
    }
}
