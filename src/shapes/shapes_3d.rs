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

//! Containing basic 3D shape data types used by other types / algorithms

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Sphere, a sphere in 3D space
pub struct Sphere {
    pub center: Point2D,
    pub radius: Positive
}
impl Eq for Sphere {}

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash, Default)]
/// Box3D, a box in 3D space
pub struct Box3D {
    pub center: Point3D,
    pub size_x: Positive,
    pub size_y: Positive,
    pub size_z: Positive
}
impl Eq for Box3D {}
