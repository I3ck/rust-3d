/*
Copyright 2017 Martin Buck
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

//! Distances between objects in ND space

use prelude::*;

/// Returns the distance between two IsND in case their number of dimensions match
pub fn dist_nd<P,U>(p1: &P, p2: &U) -> Result<f64> where
    P: IsND,
    U: IsND {

    sqr_dist_nd(p1,p2).map(|x| x.sqrt())
}

/// Returns the squared distance between two IsND in case their number of dimensions match
pub fn sqr_dist_nd<P,U>(p1: &P, p2: &U) -> Result<f64> where
    P: IsND,
    U: IsND {

    if P::n_dimensions() != U::n_dimensions() {
        return Err(ErrorKind::DimensionsDontMatch);
    }

    let mut result : f64 = 0.0;
    for i in 0..P::n_dimensions() {
        if let (Ok(val1), Ok(val2)) = (p1.get_position(i), p2.get_position(i)) {
            result += (val1 - val2).powi(2);
        } else {
            return Err(ErrorKind::IncorrectDimension);
        }
    }
    Ok(result)
}

