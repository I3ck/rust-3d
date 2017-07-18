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

//! IsDirectionField2D is a trait used for fields of directions within 2D space

use prelude::*;

/// IsDirectionField2D is a trait used for fields of directions within 2D space
pub trait IsDirectionField2D {
    /// Should return the direction at the given position
    fn direction_at<P>(&self, p: &P) -> Norm2D where
        P: Is2D;
}
