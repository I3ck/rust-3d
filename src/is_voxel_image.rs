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

//! IsVoxelImage trait used for any type of voxel image

//------------------------------------------------------------------------------

/// IsVoxelImage is a trait used for any type of voxel image
pub trait IsVoxelImage<T> {
    /// Should return the number of voxels in x-direction
    fn size_x(&self) -> usize;
    /// Should return the number of voxels in y-direction
    fn size_y(&self) -> usize;
    /// Should return the number of voxels in z-direction
    fn size_z(&self) -> usize;
    /// Should return the voxel at a given x y z coordinate
    fn voxel(&self, x: usize, y: usize, z: usize) -> Option<T>;
}
