extern crate num;

use std::fmt;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use structs::{PointCloud, CompressedPoint, CompressedPointCloud};
use traits::{Point, MoveAble};

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

impl Point {
    pub fn new() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

impl<P> MoveAble for PointCloud<P> where P: Point  {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

//------------------------------------------------------------------------------

impl<P> fmt::Display for PointCloud<P> where P: Point {
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

impl<P> PointCloud<P> where P: Point {
    pub fn new() -> PointCloud<P> {
        PointCloud{data: Vec::new()}
    }

//------------------------------------------------------------------------------

    pub fn push(&mut self, p: Point) {
        self.data.push(p);
    }

//------------------------------------------------------------------------------

    pub fn len(&self) -> usize {
        self.data.len()
    }

//------------------------------------------------------------------------------

    pub fn center(&self) -> Option<Point> {
        let size = self.len();

        if size < 1 {
            return None;
        }

        let sizef = size as f64;

        let mut sumx: f64 = 0.0;
        let mut sumy: f64 = 0.0;
        let mut sumz: f64 = 0.0;

        for p in &self.data {
            sumx += p.x;
            sumy += p.y;
            sumz += p.z;
        }

        return Some(Point {
            x: (sumx / sizef),
            y: (sumy / sizef),
            z: (sumz / sizef)
        })
    }

//------------------------------------------------------------------------------

    pub fn bbox(&self) -> Option<(Point, Point)> {
        if self.len() < 2 {
            return None;
        }

        let mut minx = self.data[0].x;
        let mut miny = self.data[0].y;
        let mut minz = self.data[0].z;
        let mut maxx = self.data[0].x;
        let mut maxy = self.data[0].y;
        let mut maxz = self.data[0].z;

        for p in &self.data {
            if p.x < minx { minx = p.x; }
            if p.y < miny { miny = p.y; }
            if p.z < minz { minz = p.z; }
            if p.x > maxx { maxx = p.x; }
            if p.y > maxy { maxy = p.y; }
            if p.z > maxz { maxz = p.z; }
        }

        return Some((Point{x: minx, y: miny, z: minz}, Point{x: maxx, y: maxy, z: maxz}));
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
TODO remove Point struct for P trait
impl<T> CompressedPointCloud<T> where T: Unsigned + PrimInt {
    pub fn compress(pc: &PointCloud<P>) -> Option<CompressedPointCloud<T>> {
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
