extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use traits::{HasPosition};
use point::{Point};
use pointCloud::{PointCloud};
use compressedPoint::{CompressedPoint};

pub struct CompressedPointCloud<T> where T: Unsigned + PrimInt {
    pub start: Point,
    pub unitsizex: f64,
    pub unitsizey: f64,
    pub unitsizez: f64,
    pub data: Vec<CompressedPoint<T>>
}


impl<T> CompressedPointCloud<T> where T: Unsigned + PrimInt {
    pub fn compress<P>(pc: &PointCloud<P>) -> Option<CompressedPointCloud<T>> where P: HasPosition {
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

            data.push(CompressedPoint{
                unitsx: unitsx,
                unitsy: unitsy,
                unitsz: unitsz
            })
        }
        return Some(CompressedPointCloud::<T>{start: pmin, unitsizex: unitsizex, unitsizey: unitsizey, unitsizez: unitsizez, data: data});
    }

//------------------------------------------------------------------------------

    pub fn decompress<P>(&self) -> Option<PointCloud<P>> where P: HasPosition {
        let mut pc = PointCloud::new();

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
