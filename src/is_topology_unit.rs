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

//! IsTopologyUnit trait used for single units of a topology. E.g. size 1 for paths, size 3 for tri meshes, size 4 for quad meshes

use crate::*;

//------------------------------------------------------------------------------

/// IsTopologyUnit trait used for single units of a topology. E.g. size 1 for paths, size 3 for tri meshes, size 4 for quad meshes
pub trait IsTopologyUnit {
    /// Should return the number of indices a unit is defined with. (e.g. 3 for a tri mesh)
    fn n_vids() -> usize;
    /// Should return the vertex id of the nth element of this unit. Failure if index out of bounds
    fn vid(&self, index: usize) -> Result<VId>;

    /// Applies the provided function to all indices within this unit
    fn for_each_vid(&self, f: &mut dyn FnMut(VId)) {
        for i in 0..Self::n_vids() {
            f(self.vid(i).unwrap()) // safe as long as implementation isn't incorrect
        }
    }
}
