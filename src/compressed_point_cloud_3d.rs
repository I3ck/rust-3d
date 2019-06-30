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

//! CompressedPointCloud3D

extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use prelude::*;
use compressed_point_3d::*;

#[derive (Debug, Clone, PartialEq, PartialOrd)]
/// CompressedPointCloud3D
pub struct CompressedPointCloud3D<T> where
    T: Unsigned + PrimInt {

    pub start: Point3D,
    pub unitsizex: f64,
    pub unitsizey: f64,
    pub unitsizez: f64,
    pub data: Vec<CompressedPoint3D<T>>
}


impl<T> CompressedPointCloud3D<T> where
    T: Unsigned + PrimInt {
    /// Creates a new CompressedPointCloud3D from a normal point cloud
    pub fn compress<P>(pc: &PointCloud3D<P>) -> Result<CompressedPointCloud3D<T>> where
        P: Is3D {

        let bb = pc.bounding_box()?;

        let rangex = (bb.max_p().x - bb.min_p().x).abs();
        let rangey = (bb.max_p().y - bb.min_p().y).abs();
        let rangez = (bb.max_p().z - bb.min_p().z).abs();

        let maxval = T::max_value().to_f64().ok_or(ErrorKind::NumberConversionError)?;
        let unitsizex = rangex / maxval;
        let unitsizey = rangey / maxval;
        let unitsizez = rangez / maxval;

        let mut data = Vec::new();

        for p in &pc.data {
            let distx = p.x() - bb.min_p().x;
            let disty = p.y() - bb.min_p().y;
            let distz = p.z() - bb.min_p().z;

            let unitsx = T::from(distx / unitsizex).ok_or(ErrorKind::NumberConversionError)?;
            let unitsy = T::from(disty / unitsizey).ok_or(ErrorKind::NumberConversionError)?;
            let unitsz = T::from(distz / unitsizez).ok_or(ErrorKind::NumberConversionError)?;

            data.push(CompressedPoint3D{
                unitsx,
                unitsy,
                unitsz
            })
        }
        Ok(CompressedPointCloud3D::<T>{start: bb.min_p(), unitsizex, unitsizey, unitsizez, data})
    }

    /// Creates a new point cloud from this
    pub fn decompress<P>(&self) -> PointCloud3D<P> where
        P: Is3D + IsBuildable3D {

        let mut pc = PointCloud3D::new();

        for p in &self.data {
            if let (Some(unitsxf), Some(unitsyf), Some(unitszf)) = (p.unitsx.to_f64(), p.unitsy.to_f64(), p.unitsz.to_f64()) {
                pc.push(P::new(
                    self.start.x + (self.unitsizex * unitsxf),
                    self.start.y + (self.unitsizey * unitsyf),
                    self.start.z + (self.unitsizez * unitszf)
                ));
            }
        }
        pc
    }
}
