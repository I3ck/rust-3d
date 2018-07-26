/*
Copyright 2018 Martin Buck
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

//! IsMatrix3Transformable trait used for types that can be transformed by a 3x3 Matrix


use prelude::*;

/// IsMatrix3Transformable trait used for types that can be transformed by a 3x3 Matrix
pub trait IsMatrix3Transformable {
    /// Should return self transformed by the Matrix
    fn transformed(&self, m: &Matrix3) -> Self;
    /// Should transform self by the Matrix
    fn transform(&mut self, m: &Matrix3);
}
