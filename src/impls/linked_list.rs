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

//! rust-3d trait implementations for the standard LinkedList

use std::collections::LinkedList;

use prelude::*;

impl<T> IsViewBuildable for LinkedList<T> where
    T: Clone {

    fn apply_view(&mut self, view: &View) -> Result<()> {
        match view {
            View::Full => { Ok(()) }
            View::Restricted(indices) => {
                let n = self.len();
                if indices.iter().any(|x| x >= &n) {
                    return Err(ErrorKind::IndexOutOfBounds);
                }
                let mut new_data = LinkedList::new();
                for (i, p) in self.iter().enumerate() {
                    if indices.contains(&i) {
                        new_data.push_back(p.clone());
                    }
                }
                *self = new_data;
                Ok(())
            }
        }
    }

    fn from_view(&self, view: &View) -> Result<Self> {
        let mut cloned = self.clone();
        cloned.apply_view(view)?;
        Ok(cloned)
    }
}

impl<T> IsMovable2D for LinkedList<T> where
    T: IsMovable2D {

    fn move_by(&mut self, x: f64, y: f64) {
        for ref mut p in self.iter_mut() {
            p.move_by(x, y);
        }
    }
}

impl<T> IsMovable3D for LinkedList<T> where
    T: IsMovable3D {

    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for ref mut p in self.iter_mut() {
            p.move_by(x, y, z);
        }
    }
}
