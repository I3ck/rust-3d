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

//! IsBuildableND trait used for types which are positioned in n-dimensional space and can be constructed

use crate::*;

//------------------------------------------------------------------------------

/// IsBuildableND is a trait used for types which are positioned in n-dimensional space and can be constructed
pub trait IsBuildableND: Sized + IsND {
    /// Should build an object from the correct number of coordinates
    fn new_nd(coords: &[f64]) -> Result<Self>;
    /// Should use the coordinates of another as its own
    fn from_nd<P>(&mut self, other: P) -> Result<()>
    where
        P: IsBuildableND;

    /// Returns a new object with 0 for all coordinates
    fn zero_nd() -> Result<Self> {
        Self::new_nd(&vec![0.0; Self::n_dimensions()])
    }
    /// Returns the center between this and other
    fn center_nd<P>(&self, other: &P, buffer: &mut Vec<f64>) -> Result<Self>
    where
        P: IsND,
    {
        let n = Self::n_dimensions();

        if n != P::n_dimensions() {
            return Err(ErrorKind::IncorrectDimension);
        }

        buffer.clear();
        for i in 0..n {
            buffer.push(0.5 * (self.position_nd(i)? + other.position_nd(i)?));
        }

        Self::new_nd(&buffer)
    }
}
