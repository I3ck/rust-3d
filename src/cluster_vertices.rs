/*
Copyright 2019 Martin Buck

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

//! Algorithm to cluster nearby vertices within a mesh

use crate::*;

use fnv::FnvHashMap;
use std::hash::Hash;

//------------------------------------------------------------------------------

/// Algorithm to cluster nearby vertices within a mesh
pub fn cluster_vertices<P, M>(mesh: &M, cluster_size: f64) -> Result<M>
where
    M: IsFaceEditableMesh<P, Face3>
        + IsVertexEditableMesh<P, Face3>
        + Default
        + HasBoundingBox3DMaybe,
    P: IsBuildable3D + Eq + Hash + Clone,
{
    let bb = mesh
        .bounding_box_maybe()
        .ok_or(ErrorKind::BoundingBoxMissing)?;
    let [sx, sy, sz] = bb.sizes();
    let min = P::new_from(&bb.min_p());
    let (nx, ny, nz) = (
        (sx.get() / cluster_size) as usize,
        (sy.get() / cluster_size) as usize,
        (sz.get() / cluster_size) as usize,
    );
    if nx < 2 || ny < 2 || nz < 2 {
        return Err(ErrorKind::ClusterTooBig);
    }

    let cluster_of = |ref p| {
        let v = conn(&min, p);
        (
            (v.x() / cluster_size) as usize,
            (v.y() / cluster_size) as usize,
            (v.z() / cluster_size) as usize,
        )
    };

    let nv = mesh.num_vertices();
    let mut cluster_map = FnvHashMap::default();
    let mut clusters = Vec::with_capacity(nv);

    for i in 0..nv {
        let p = mesh.vertex(VId { val: i }).unwrap(); // safe, since index in range
        let cluster = cluster_of(p);
        cluster_map.insert(cluster, i); //@todo later this must keep the 'best' vertex instead of the last
        clusters.push(cluster);
    }

    let mut result = M::default();

    let new_vertex = |old_index| {
        let cluster = &clusters[old_index];
        mesh.vertex(VId {
            val: *cluster_map.get(cluster).unwrap(), // safe since any cluster in clusters also within cluster_map
        })
        .unwrap() // safe, since index in range
    };

    let nf = mesh.num_faces();
    result.reserve_faces(nf);
    result.reserve_vertices(3 * nf);
    for i in 0..nf {
        let f = mesh.face_vertex_ids(FId { val: i }).unwrap();

        result.add_face(
            new_vertex(f.a.val),
            new_vertex(f.b.val),
            new_vertex(f.c.val),
        );
    }

    result = heal_mesh(&result)?;

    Ok(result)
}
