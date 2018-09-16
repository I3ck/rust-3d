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

//! Subdivision algorithms to e.g. refine meshes

use prelude::*;

use std::cmp::{min, max};
use std::collections::HashMap;

/// Subdivides a mesh linearly by creating four faces for each input face
/// This will not smoothen the input mesh, since new vertices are placed only on existing edges
pub fn linear<V, MI, MO>(mi: &MI) -> Result<(MO)> where
    MI: IsMesh<V, Face3>,
    MO: IsFaceEditableMesh<V, Face3> + IsVertexEditableMesh<V, Face3> + Default,
    V:  IsBuildableND {
    
    let mut mo = MO::default();
    
    let n_vertices      = mi.num_vertices();
    let n_faces         = mi.num_faces();
    let mut added_edges = HashMap::new();
    
    for i in 0..n_vertices {
        mo.add_vertex(mi.vertex(VId{val: i})?);
    }
    
    for i in 0..n_faces {
        let f               = mi.face_vertex_ids(FId{val:i})?;
        let (vi1, vi2, vi3) = (f.get_vid(0)?, f.get_vid(1)?, f.get_vid(2)?);
        let (v1, v2, v3)    = mi.face_vertices(FId{ val:i })?;
        
        let ia = *added_edges.entry((min(vi1, vi2), max(vi1, vi2))).or_insert_with(|| mo.add_vertex(v1.center(&v2).unwrap()));
        let ib = *added_edges.entry((min(vi2, vi3), max(vi2, vi3))).or_insert_with(|| mo.add_vertex(v2.center(&v3).unwrap()));
        let ic = *added_edges.entry((min(vi3, vi1), max(vi3, vi1))).or_insert_with(|| mo.add_vertex(v3.center(&v1).unwrap()));
        
        mo.try_add_connection(vi1, ia, ic)?;
        mo.try_add_connection(ia, vi2, ib)?;
        mo.try_add_connection(ia, ib, ic)?;
        mo.try_add_connection(ic, ib, vi3)?;      
    }
    
    Ok(mo)
}
