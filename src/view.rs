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

//! View, which defines a restricted / full view onto any T. E.g. used when filtering collections of points.

use std::collections::HashSet;

#[derive(Clone)]
/// View, which defines a restricted / full view onto any T. E.g. used when filtering collections of points.
pub enum View {
    Full,
    Restricted(HashSet<usize>)
}

impl View {
    /// Merges two Views
    pub fn union(&mut self, other: View) {
        match other {
            View::Full => { *self = other }
            View::Restricted(indices_other) => {
                match self {
                    &mut View::Full => {}
                    &mut View::Restricted(ref mut indices_source) => {
                        indices_source.extend(&indices_other);
                    }
                }
            }
        }
    }
}
