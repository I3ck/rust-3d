extern crate num;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

pub struct CompressedPoint<T> where T: Unsigned + PrimInt  {
    pub unitsx: T,
    pub unitsy: T,
    pub unitsz: T
}