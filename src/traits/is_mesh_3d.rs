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

use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_normalized_3d::IsNormalized3D;
use functions::{conn, cross};
use norm_3d::Norm3D;

pub trait IsMesh3D<P> where
    P: IsBuildable3D {

    fn num_faces(&self) -> usize;

    fn num_vertices(&self) -> usize;

    fn face_vertex_ids(&self, faceid: usize) -> Option<(usize, usize, usize)>;

    fn face_vertices(&self, faceid: usize) -> Option<(P, P, P)>;

    fn vertex(&self, vertexid: usize) -> Option<P>;

    fn face_normal(&self, faceid: usize) -> Option<Norm3D> {
        let (v1, v2, v3) = match self.face_vertices(faceid) {
            None => return None,
            Some((v1, v2, v3)) => (v1, v2, v3)
        };

        let v12 = conn(&v1, &v2);
        let v23 = conn(&v2, &v3);

        let n = cross(&v12, &v23);

        match Norm3D::new(*n) {
            None => None,
            Some(b) => Some(*b)
        }
    }
}
