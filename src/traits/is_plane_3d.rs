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

//! IsPlane3D trait used for planes within 3D space

use crate::prelude::*;

/// IsPlane3D is a trait used for planes within 3D space
pub trait IsPlane3D<P,N> : Sized where
    P: Is3D,
    N: IsNormalized3D {
    /// Should return a new plane with the given origin, u and v vectors
    fn new(origin: P, u: N, v: N) -> Self;
    /// Should return the origin of the plane
    fn origin(&self) -> P;
    /// Should return the u vector of the plane
    fn u(&self) -> N;
    /// Should return the v vector of the plane
    fn v(&self) -> N;
}
