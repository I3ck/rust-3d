/*
Copyright 2018 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
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
        let (vi1, vi2, vi3) = (f.vid(0)?, f.vid(1)?, f.vid(2)?);
        let [v1, v2, v3]    = mi.face_vertices(FId{ val:i })?;
        
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
