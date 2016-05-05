extern crate num;

use std::fmt;
use std::cmp;
use std::cmp::Ordering;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use point::{Point};
use pointCloud::{PointCloud};
use structs::{CompressedPoint, CompressedPointCloud};
use traits::{MoveAble};
use functions::{dist, sqr_dist, dimension_compare, dimension_dist, sort_and_limit};

//------------------------------------------------------------------------------

impl MoveAble for Point {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

//------------------------------------------------------------------------------

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//------------------------------------------------------------------------------



//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

impl MoveAble for PointCloud {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

//------------------------------------------------------------------------------

impl fmt::Display for PointCloud {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.data {
            match p.fmt(f) {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
            match f.write_str("\n") {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
        }
        return Ok(());
    }
}

//------------------------------------------------------------------------------



//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

impl<T> CompressedPointCloud<T> where T: Unsigned + PrimInt {
    pub fn compress(pc: &PointCloud) -> Option<CompressedPointCloud<T>> {
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
            let distx = p.x - pmin.x;
            let disty = p.y - pmin.y;
            let distz = p.z - pmin.z;

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

    pub fn decompress(&self) -> Option<PointCloud> {
        let mut pc = PointCloud::new();

        for p in &self.data {
            if let (Some(unitsxf), Some(unitsyf), Some(unitszf)) = (p.unitsx.to_f64(), p.unitsy.to_f64(), p.unitsz.to_f64()) {
                pc.push(Point{
                    x: self.start.x + (self.unitsizex * unitsxf),
                    y: self.start.y + (self.unitsizey * unitsyf),
                    z: self.start.z + (self.unitsizez * unitszf)
                });
            }
        }
        return Some(pc);
    }
}
