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

//! Containing basic 2D shape data types used by other types / algorithms

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash)]
/// Circle, a circle in 2D space
pub struct Circle {
    pub center: Point2D,
    pub radius: Positive
}

#[derive (Debug, PartialEq, PartialOrd, Clone, Hash)]
/// Box2D, a box in 2D space
pub struct Box2D {
    pub center: Point2D,
    pub size_x: Positive,
    pub size_y: Positive
}
