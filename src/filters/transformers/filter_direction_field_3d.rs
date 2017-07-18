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

//! FilterDirectionField3D, a filter which can transform any IsDirectionField3D into a filter for (Is3D, IsNormalized3D)

use prelude::*;

/// FilterDirectionField3D, a filter which can transform any IsDirectionField3D into a filter for (Is3D, IsNormalized3D)
pub struct FilterDirectionField3D<DF> where
    DF: IsDirectionField3D {

    field: Box<DF>,
    eps: Rad
}

impl<DF> FilterDirectionField3D<DF> where
    DF: IsDirectionField3D {

    pub fn new(field: DF, eps: Rad ) -> Self {
        FilterDirectionField3D {field: Box::new(field), eps: eps}
    }
}

impl<DF, P, N> IsFilter<(P, N)> for FilterDirectionField3D<DF> where
    DF: IsDirectionField3D,
    P:  Is3D,
    N:  IsNormalized3D {

    fn is_allowed(&self, pn: &(P, N)) -> bool {
        let expected = self.field.direction_at(&pn.0);
        return expected.rad_to(&pn.1) <= self.eps;
    }
}