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

//! Module containing the IsRandomAccessible2D trait used for collections of positions within 2D space

use result::*;
use traits::is_2d::*;

/// IsRandomAccessible2D is a trait used for collections of positions within 2D space
pub trait IsRandomAccessible2D<P> where
    P: Is2D {
    /// Should return the number of points within the collection
    fn n_points(&self) -> usize;
    /// Should return the point of the given id
    fn get_point(&self, index: usize) -> Result<P>;
    /// Should append a point to the collection
    fn append_point(&mut self, point: P);
    /// Should insert a point at the given index
    fn insert_point(&mut self, index: usize, point: P) -> Result<()>;
    /// Should apply the function to the point at the given index
    fn map_point(&mut self, index: usize, f: &mut FnMut(&mut P)) -> Result<()>;
}
