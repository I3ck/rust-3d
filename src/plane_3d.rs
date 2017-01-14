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

use traits::is_plane_3d::IsPlane3D;
use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_normalized_3d::IsNormalized3D;

pub struct Plane3D<P,N> where
    P: Is3D,
    N: IsNormalized3D {

    pub origin: P,
    pub u: N,
    pub v: N
}


impl<P,N> IsPlane3D<P,N> for Plane3D<P,N> where
    P: IsBuildable3D + Clone,
    N: IsNormalized3D + Clone {

    fn new() -> Box<Self> {
        Box::new(Plane3D {
            origin: *P::build(0.0, 0.0, 0.0),
            u: N::norm_x(),
            v: N::norm_y()
        })
    }

    fn build(origin: P, u: N, v: N) -> Box<Self> {
        Box::new(Plane3D {
            origin: origin,
            u: u,
            v: v
        })
    }

    fn origin(&self) -> P {
        self.origin.clone()
    }

    fn u(&self) -> N {
        self.u.clone()
    }

    fn v(&self) -> N {
        self.v.clone()
    }
}
