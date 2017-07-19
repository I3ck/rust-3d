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

//! rust-3d trait implementations for the standard Box

use prelude::*;

impl<P> IsND for Box<P> where
    P: IsND {

    fn n_dimensions() -> usize {
        P::n_dimensions()
    }

    fn get_position(&self, dimension: usize) -> Result<f64> {
        self.as_ref().get_position(dimension)
    }
}

impl<P> Is2D for Box<P> where
    P: IsND + Is2D {

    fn x(&self) -> f64 {
        self.as_ref().x()
    }

    fn y(&self) -> f64 {
        self.as_ref().y()
    }
}

impl<P> Is3D for Box<P> where
    P: IsND + Is3D {

    fn x(&self) -> f64 {
        self.as_ref().x()
    }

    fn y(&self) -> f64 {
        self.as_ref().y()
    }

    fn z(&self) -> f64 {
        self.as_ref().z()
    }
}
