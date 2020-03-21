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

//! rust-3d trait implementations for the standard Box

use crate::*;

//------------------------------------------------------------------------------

impl<P> IsND for Box<P>
where
    P: IsND,
{
    fn n_dimensions() -> usize {
        P::n_dimensions()
    }

    fn position_nd(&self, dimension: usize) -> Result<f64> {
        self.as_ref().position_nd(dimension)
    }
}

impl<P> Is2D for Box<P>
where
    P: IsND + Is2D,
{
    fn x(&self) -> f64 {
        self.as_ref().x()
    }

    fn y(&self) -> f64 {
        self.as_ref().y()
    }
}

impl<P> Is3D for Box<P>
where
    P: IsND + Is3D,
{
    fn x(&self) -> f64 {
        self.as_ref().x()
    }

    fn y(&self) -> f64 {
        self.as_ref().y()
    }

    fn z(&self) -> f64 {
        self.as_ref().z()
    }
}
