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

//! IsKNearestSearchable trait used for search structures which can be queried for nearest neighbours.
//! You should only implement this, if your solution is rather efficient

use prelude::*;

/// IsKNearestSearchable trait used for search structures which can be queried for nearest neighbours.
/// You should only implement this, if your solution is rather efficient
pub trait IsKNearestSearchable<T> {
    /// Should return the nearest neighbour to search, if there is any
    fn nearest(&self, search: &T) -> Result<T>;
    /// Should return the k nearest neighbours to search
    fn knearest(&self, search: &T, n: usize) -> Vec<T>;
}
