/*
Copyright 2018 Martin Buck
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

//! IsPolygon is a trait used for closed polygons

use prelude::*;

/// IsPolygon is a trait used for closed polygons
pub trait IsPolygon<V> {
    /// Should return the number of segments within the polygon
    fn num_segments(&self) -> usize;
    /// Should return the ids of vertices of the given segment
    fn segment_vertex_ids(&self, segmentid: SId) -> Result<(VId, VId)>;
    /// Should return the vertices of the given segment
    fn segment_vertices(&self, segmentid: SId) -> Result<(V, V)>;
    /// Should return the vertex with the given id
    fn vertex(&self, vertexid: VId) -> Result<V>;

    /// Returns the number of vertices within the polygon
    fn num_vertices(&self) -> usize {
        self.num_segments()
    }
}
