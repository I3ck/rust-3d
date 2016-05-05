extern crate num;

use std::fmt;
use std::cmp;
use std::cmp::Ordering;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use point::{Point};
use pointCloud::{PointCloud};
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
