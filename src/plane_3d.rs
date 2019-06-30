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

//! Plane3D, a plane within 3D space

use prelude::*;

/// Plane3D, a plane within 3D space
#[derive (Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
pub struct Plane3D<P, N> where
    P: Is3D,
    N: IsNormalized3D {

    pub origin: P,
    pub u: N,
    pub v: N
}

impl<P, N> Default for Plane3D<P, N> where
    P: Is3D + Default,
    N: IsNormalized3D {

    fn default() -> Self {
        Plane3D {
            origin: P::default(),
            u: N::norm_x(),
            v: N::norm_y()
        }
    }
}

impl<P, N> IsPlane3D<P,N> for Plane3D<P,N> where
    P: IsBuildable3D + Clone,
    N: IsNormalized3D + Clone {

    fn new(origin: P, u: N, v: N) -> Self {
        Plane3D { origin, u, v }
    }

    fn origin(&self) -> P {
        self.origin.clone()
    }

    fn u(&self) -> N {
        self.u.clone()
    }

    fn v(&self) -> N {
        self.v.clone()
    }
}
