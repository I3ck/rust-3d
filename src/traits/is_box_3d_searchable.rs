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

//! IsBox3DSearchable trait used for search structures which can be queried for elements within a 3D box
//! You should only implement this, if your solution is rather efficient

use prelude::*;

/// IsBox3DSearchable trait used for search structures which can be queried for elements within a 3D box
/// You should only implement this, if your solution is rather efficient
pub trait IsBox3DSearchable<PSearch, PFind> where
    PSearch: Is3D,
    PFind:   Is3D {
    /// Should return all positions within a box around search
    fn in_box(&self, search: &PSearch, x_size: f64, y_size: f64, z_size: f64) -> Vec<PFind>;
}
