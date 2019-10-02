/*
Copyright 2017 Martin Buck

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

//! HasDistanceTo trait used for types which have a defined distance towards another T.
//! Implementing HasDistanceTo<T> for U also implements HasDistanceTo<U> for T

use crate::prelude::*;

/// HasDistanceTo trait used for types which have a defined distance towards another T.
/// Implementing HasDistanceTo<T> for U also implements HasDistanceTo<U> for T
pub trait HasDistanceTo<T> {
    /// Should return the sqr distance to other
    fn sqr_distance(&self, other: &T) -> NonNegative;
    /// The distance to other
    fn distance(&self, other: &T) -> NonNegative {
        self.sqr_distance(other).sqrt()
    }
}

/*
@todo currently breaks A.distance(A). Possible, once specialization hits stable
impl<T, U> HasDistanceTo<T> for U where
    T: HasDistanceTo<U> {

    fn sqr_distance(&self, other: &T) -> NonNegative {
        other.sqr_distance(self)
    }
}*/
