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

//! IsNormalized3D trait used for types which are positioned within the 3D space and normalized

use prelude::*;

/// IsNormalized3D is a trait used for types which are positioned within the 3D space and normalized
pub trait IsNormalized3D : 
    Sized + 
    Is3D {
    
    fn new<P>(p: P) -> Result<Self> where
        P: Is3D;
    /// Should return a new normalized object which only points in the x-Direction
    fn norm_x() -> Self;
    /// Should return a new normalized object which only points in the y-Direction
    fn norm_y() -> Self;
    /// Should return a new normalized object which only points in the z-Direction
    fn norm_z() -> Self;
}
