extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use traits::{HasPosition3D};
use point::{Point3D};
use pointCloud3D::{Point3DCloud3D};
use compressedPoint3D::{CompressedPoint3D};

pub struct CompressedPoint3DCloud3D<T> where T: Unsigned + PrimInt {
    pub start: Point3D,
    pub unitsizex: f64,
    pub unitsizey: f64,
    pub unitsizez: f64,
    pub data: Vec<CompressedPoint3D<T>>
}


impl<T> CompressedPoint3DCloud3D<T> where T: Unsigned + PrimInt {
    pub fn compress<P>(pc: &Point3DCloud3D<P>) -> Option<CompressedPoint3DCloud3D<T>> where P: HasPosition3D {
        let (pmin, pmax) = match pc.bbox() {
            None        => return None,
            Some(res)   => res,
        };

        let rangex = (pmax.x - pmin.x).abs();
        let rangey = (pmax.y - pmin.y).abs();
        let rangez = (pmax.z - pmin.z).abs();

        let maxval = match T::max_value().to_f64() {
            None        => return None,
            Some(res)   => res,
        };

        let unitsizex = rangex / maxval;
        let unitsizey = rangey / maxval;
        let unitsizez = rangez / maxval;

        let mut data = Vec::new();

        for p in &pc.data {
            let distx = p.x() - pmin.x;
            let disty = p.y() - pmin.y;
            let distz = p.z() - pmin.z;

            let unitsx = match T::from(distx / unitsizex) {
                None        => return None,
                Some(res)   => res
            };

            let unitsy = match T::from(disty / unitsizey) {
                None        => return None,
                Some(res)   => res
            };

            let unitsz = match T::from(distz / unitsizez) {
                None        => return None,
                Some(res)   => res
            };

            data.push(CompressedPoint3D{
                unitsx: unitsx,
                unitsy: unitsy,
                unitsz: unitsz
            })
        }
        return Some(CompressedPoint3DCloud3D::<T>{start: pmin, unitsizex: unitsizex, unitsizey: unitsizey, unitsizez: unitsizez, data: data});
    }

//------------------------------------------------------------------------------

    pub fn decompress<P>(&self) -> Option<Point3DCloud3D<P>> where P: HasPosition3D {
        let mut pc = Point3DCloud3D::new();

        for p in &self.data {
            if let (Some(unitsxf), Some(unitsyf), Some(unitszf)) = (p.unitsx.to_f64(), p.unitsy.to_f64(), p.unitsz.to_f64()) {
                pc.push(*P::build(
                    self.start.x + (self.unitsizex * unitsxf),
                    self.start.y + (self.unitsizey * unitsyf),
                    self.start.z + (self.unitsizez * unitszf)
                ));
            }
        }
        return Some(pc);
    }
}
