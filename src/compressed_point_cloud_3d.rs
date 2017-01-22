/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use result::*;
use traits::is_3d::*;
use traits::is_buildable_3d::*;
use traits::has_bounding_box_3d::*;
use point_3d::*;
use point_cloud_3d::*;
use compressed_point_3d::*;

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

    pub fn compress<P>(pc: &PointCloud3D<P>) -> Result<CompressedPointCloud3D<T>> where
        P: Is3D {

        let (pmin, pmax) = try!(pc.bounding_box());

        let rangex = (pmax.x - pmin.x).abs();
        let rangey = (pmax.y - pmin.y).abs();
        let rangez = (pmax.z - pmin.z).abs();

        let maxval = try!(T::max_value().to_f64().ok_or(ErrorKind::NumberConversionError));
        let unitsizex = rangex / maxval;
        let unitsizey = rangey / maxval;
        let unitsizez = rangez / maxval;

        let mut data = Vec::new();

        for p in &pc.data {
            let distx = p.x() - pmin.x;
            let disty = p.y() - pmin.y;
            let distz = p.z() - pmin.z;

            let unitsx = try!(T::from(distx / unitsizex).ok_or(ErrorKind::NumberConversionError));
            let unitsy = try!(T::from(disty / unitsizey).ok_or(ErrorKind::NumberConversionError));
            let unitsz = try!(T::from(distz / unitsizez).ok_or(ErrorKind::NumberConversionError));

            data.push(CompressedPoint3D{
                unitsx: unitsx,
                unitsy: unitsy,
                unitsz: unitsz
            })
        }
        Ok(CompressedPointCloud3D::<T>{start: pmin, unitsizex: unitsizex, unitsizey: unitsizey, unitsizez: unitsizez, data: data})
    }

//------------------------------------------------------------------------------

    pub fn decompress<P>(&self) -> PointCloud3D<P> where
        P: Is3D + IsBuildable3D {

        let mut pc = PointCloud3D::new();

        for p in &self.data {
            if let (Some(unitsxf), Some(unitsyf), Some(unitszf)) = (p.unitsx.to_f64(), p.unitsy.to_f64(), p.unitsz.to_f64()) {
                pc.push(*P::build(
                    self.start.x + (self.unitsizex * unitsxf),
                    self.start.y + (self.unitsizey * unitsyf),
                    self.start.z + (self.unitsizez * unitszf)
                ));
            }
        }
        pc
    }
}
