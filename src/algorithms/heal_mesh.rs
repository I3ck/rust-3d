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

//! Algorithm remove duplicate and degenerate faces from a mesh

use crate::prelude::*;

use std::{collections::HashMap, hash::Hash};

//------------------------------------------------------------------------------

/// Algorithm remove duplicate and degenerate faces from a mesh
pub fn heal_mesh<P, M>(mesh: &M) -> Result<M>
where
    M: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3> + Default,
    P: Is3D + Eq + Hash + Clone,
{
    let nf = mesh.num_faces();

    let mut duped_vertices = Vec::with_capacity(3 * nf);

    for i in 0..nf {
        let [v1, v2, v3] = mesh.face_vertices(FId { val: i }).unwrap(); // safe
        duped_vertices.push(v1);
        duped_vertices.push(v2);
        duped_vertices.push(v3);
    }

    let (unduped, indices) = pack_dedup_indexed(duped_vertices.iter());

    let mut result = M::default();

    for x in unduped {
        result.add_vertex(x);
    }

    let nf_new = indices.len() / 3;
    for f in 0..nf_new {
        let (a, b, c) = (indices[3 * f + 0], indices[3 * f + 1], indices[3 * f + 2]);
        if a == b || a == c || b == c {
            continue;
        }
        result.try_add_connection(VId { val: a }, VId { val: b }, VId { val: c })?;
    }

    Ok(result)
}

//------------------------------------------------------------------------------

//@todo better name
//@todo at least move to general utils
pub fn pack_dedup_indexed<'a, I, T>(idata: I) -> (Vec<T>, Vec<usize>)
where
    I: Iterator<Item = &'a T>,
    T: 'a + Eq + Hash + Clone,
{
    let mut map = HashMap::new();
    let mut packed_data = Vec::new();
    let mut ids = Vec::new();
    for x in idata {
        let id = map.entry(x).or_insert_with(|| {
            let value = packed_data.len();
            packed_data.push(x.clone());
            value
        });
        ids.push(*id);
    }

    (packed_data, ids)
}
