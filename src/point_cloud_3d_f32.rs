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

//@todo implement as many traits as possible (see point_cloud_3d for reference)

//! PointCloud3Df32, a collection of positions within 3D space stored lossy as f32 vector for easier usage during rendering

use crate::*;

use std::marker::PhantomData;

//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
/// PointCloud3Df32, a collection of positions within 3D space stored lossy as f32 vector for easier usage during rendering
pub struct PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    pub data: Vec<f32>,
    _phantom: PhantomData<P>,
}

impl<P> PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    /// Creates a new, empty point cloud
    pub fn new() -> PointCloud3Df32<P> {
        PointCloud3Df32 {
            data: Vec::new(),
            _phantom: PhantomData::default(),
        }
    }
    /// Creates a new, empty point cloud with capacity for n points
    pub fn with_capacity(n: usize) -> PointCloud3Df32<P> {
        PointCloud3Df32 {
            data: Vec::with_capacity(3 * n),
            _phantom: PhantomData::default(),
        }
    }
}

//------------------------------------------------------------------------------

impl<P> IsPushable<P> for PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    fn push(&mut self, p: P) {
        self.data.push(p.x() as f32);
        self.data.push(p.y() as f32);
        self.data.push(p.z() as f32);
    }
}

impl<P> IsDataContainer<P> for PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    fn reserve_d(&mut self, n: usize) {
        self.data.reserve(3 * n);
    }

    fn len_d(&self) -> usize {
        self.data.len() / 3
    }

    fn push_d(&mut self, p: P) {
        self.data.push(p.x() as f32);
        self.data.push(p.y() as f32);
        self.data.push(p.z() as f32);
    }

    fn get_d(&self, index: usize) -> P {
        P::new(
            self.data[3 * index + 0] as f64,
            self.data[3 * index + 1] as f64,
            self.data[3 * index + 2] as f64,
        )
    }

    fn set_d(&mut self, index: usize, p: P) {
        self.data[3 * index + 0] = p.x() as f32;
        self.data[3 * index + 1] = p.y() as f32;
        self.data[3 * index + 2] = p.z() as f32;
    }
}

impl<P> IsMovable3D for PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for index in 0..self.data.len() / 3 {
            self.data[3 * index + 0] += x as f32;
            self.data[3 * index + 1] += y as f32;
            self.data[3 * index + 2] += z as f32;
        }
    }
}

impl<P> HasBoundingBox3DMaybe for PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        BoundingBox3D::from_into_iterator(
            self.data
                .chunks_exact(3)
                .map(|chunk| P::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64)),
        )
    }
}

impl<P> Into<Vec<f32>> for PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    fn into(self) -> Vec<f32> {
        self.data
    }
}

impl<P> From<Vec<f32>> for PointCloud3Df32<P>
where
    P: IsBuildable3D,
{
    fn from(data: Vec<f32>) -> Self {
        Self {
            data,
            _phantom: PhantomData::default(),
        }
    }
}
