extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use point::{Point};

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

pub struct OctTree {
    pub root: OctNode,
    pub min: Point,
    pub max: Point
}

pub enum OctNode {
    Leaf(Point),
    Internal {
        tfl: Option<Box<OctNode>>, //top front left
        tfr: Option<Box<OctNode>>, //top front right
        tbl: Option<Box<OctNode>>, //top back left
        tbr: Option<Box<OctNode>>, //top back right
        bfl: Option<Box<OctNode>>, //bottom front left
        bfr: Option<Box<OctNode>>, //bottom front right
        bbl: Option<Box<OctNode>>, //bottom back left
        bbr: Option<Box<OctNode>>  //bottom back right
    }
}
