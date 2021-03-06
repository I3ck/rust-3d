/*
Copyright 2016 Martin Buck

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

//! View, which defines a restricted / full view onto any T. E.g. used when filtering collections of points.

use std::collections::HashSet;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Clone)]
/// View, which defines a restricted / full view onto any T. E.g. used when filtering collections of points.
pub enum View {
    Full,
    Restricted(HashSet<usize>),
}

impl View {
    /// Merges two Views
    pub fn union(&mut self, other: View) {
        match other {
            Self::Full => *self = other,
            Self::Restricted(indices_other) => match self {
                Self::Full => {}
                Self::Restricted(indices_source) => {
                    indices_source.extend(&indices_other);
                }
            },
        }
    }
}
