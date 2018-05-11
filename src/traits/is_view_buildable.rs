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

//! IsViewBuildable trait used for types which can be constructed from a view / have a view applied to them

use prelude::*;

/// IsViewBuildable trait used for types which can be constructed from a view / have a view applied to them
pub trait IsViewBuildable : Sized {
    /// Should apply the view and only keep items indexed within the view. Should return an error if any index is out of bounds
    fn apply_view(&mut self, view: &View) -> Result<()>;
    /// Should create a new object consisting only of items indexed within the view. Should return an error if any index is out of bounds
    fn from_view(&self, view: &View) -> Result<Self>;
}
