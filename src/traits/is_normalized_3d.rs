/*
Copyright 2016 Martin Buck
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

use result::*;
use traits::is_3d::*;

pub trait IsNormalized3D : Is3D {
    fn new<P>(p: P) -> Result<Box<Self>> where
        P: Is3D;

    fn norm_x() -> Self;

    fn norm_y() -> Self;

    fn norm_z() -> Self;
}
