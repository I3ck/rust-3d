extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub struct PointCloud {
    pub data: Vec<Point>
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

pub struct KdTree {
    pub root: KdNode
}

pub struct KdNode {
    pub left: Option<Box<KdNode>>,
    pub right: Option<Box<KdNode>>,
    pub dimension: i8
}
