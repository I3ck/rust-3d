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

//! IsPlane3D trait used for planes within 3D space

use prelude::*;

/// IsPlane3D is a trait used for planes within 3D space
pub trait IsPlane3D<P,N> where
    P: Is3D,
    N: IsNormalized3D {
    /// Should return a new plane with the given origin, u and v vectors
    fn new(origin: P, u: N, v: N) -> Box<Self>;
    /// Should return the origin of the plane
    fn origin(&self) -> P;
    /// Should return the u vector of the plane
    fn u(&self) -> N;
    /// Should return the v vector of the plane
    fn v(&self) -> N;
}
