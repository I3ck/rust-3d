extern crate num;

use traits::{Point};

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;



pub struct PointCloud<P> where P: Point {
    pub data: Vec<P>
}

pub struct CompressedPoint<T> where T: Unsigned + PrimInt  {
    pub unitsx: T,
    pub unitsy: T,
    pub unitsz: T
}

pub struct CompressedPointCloud<T> where T: Unsigned + PrimInt {
    pub start: Point,
    pub unitsizex: f64,
    pub unitsizey: f64,
    pub unitsizez: f64,
    pub data: Vec<CompressedPoint<T>>
}
