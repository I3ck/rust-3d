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

//! Algorithm to unifiy the face orientation within a mesh

use crate::*;
use bitvec::bitvec;
use fnv::FnvHashSet;

//------------------------------------------------------------------------------

//@todo consider rewrite to mutate the input instead

/// Algorithm to unifiy the face orientation within a mesh
pub fn unify_faces<P, M>(mesh: &M) -> Result<M>
where
    M: IsFaceEditableMesh<P, Face3> + IsVertexEditableMesh<P, Face3> + Default,
    P: IsBuildable3D,
{
    let v_to_f = vertex_to_face(mesh);
    let n_f_total = mesh.num_faces();
    let n_v_total = mesh.num_vertices();
    let mut checked = bitvec![0; n_f_total];
    let mut must_flip = bitvec![0; n_f_total];
    let mut frontier = Vec::new();
    let mut checked_lowest = 0;
    let mut neighbour_buffer = Vec::new();

    loop {
        if checked_lowest == n_f_total {
            break;
        }

        while checked_lowest < n_f_total {
            let mut found = false;
            if !checked[checked_lowest] {
                frontier.push(checked_lowest);
                checked.set(checked_lowest, true);
                found = true;
            }
            checked_lowest += 1;
            if found {
                break;
            }
        }

        while let Some(this) = frontier.pop() {
            neighbour_buffer.clear();
            collect_neighbour_faces(mesh, &v_to_f, FId { val: this }, &mut neighbour_buffer)?;

            let [v1, v2, v3] = mesh.face_vertices(FId { val: this }).unwrap(); // safe since index is safe
            let n_this = normal_of_face(&v1, &v2, &v3);

            for neighbour in neighbour_buffer.iter() {
                if checked[neighbour] {
                    continue;
                }

                let [v1n, v2n, v3n] = mesh.face_vertices(FId { val: neighbour }).unwrap(); // safe since index is safe
                let n_neighbour = normal_of_face(&v1n, &v2n, &v3n);

                let are_different = n_this.dot(&n_neighbour) < 0.0;
                let flip_this = must_flip[this];
                must_flip.set(
                    neighbour,
                    if flip_this {
                        !are_different
                    } else {
                        are_different
                    },
                );
                frontier.push(neighbour);
                checked.set(neighbour, true);
            }
        }
    }

    let mut result = M::default();
    result.reserve_vertices(n_v_total);
    result.reserve_faces(n_f_total);
    for i in 0..n_v_total {
        result.add_vertex(mesh.vertex(VId { val: i }).unwrap()); // safe since index safe
    }

    for i in 0..n_f_total {
        let f = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe since index safe
        if must_flip[i] {
            //println!("add normal");
            result.try_add_connection(f.a, f.c, f.b).unwrap(); // safe assuming original mesh was valid
        } else {
            //println!("add flipped");
            result.try_add_connection(f.a, f.b, f.c).unwrap(); // safe assuming original mesh was valid
        }
    }

    Ok(result)
}

//------------------------------------------------------------------------------

fn vertex_to_face<P, M>(mesh: &M) -> Vec<FnvHashSet<usize>>
where
    M: IsMesh<P, Face3> + Default,
    P: Is3D,
{
    let nv = mesh.num_vertices();
    let nf = mesh.num_faces();
    let mut v_to_f = vec![FnvHashSet::default(); nv];

    for i in 0..nf {
        let f = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe
        v_to_f[f.a.val].insert(i);
        v_to_f[f.b.val].insert(i);
        v_to_f[f.c.val].insert(i);
    }

    v_to_f
}

//------------------------------------------------------------------------------

fn collect_neighbour_faces<P, M>(
    mesh: &M,
    v_to_f: &Vec<FnvHashSet<usize>>,
    fid: FId,
    neighbours: &mut Vec<usize>,
) -> Result<()>
where
    M: IsMesh<P, Face3> + Default,
    P: Is3D,
{
    let f = mesh
        .face_vertex_ids(fid)
        .ok_or(ErrorKind::IncorrectFaceID)?;
    neighbours.extend(v_to_f[f.a.val].iter());
    neighbours.extend(v_to_f[f.b.val].iter());
    neighbours.extend(v_to_f[f.c.val].iter());
    neighbours.sort();
    neighbours.dedup();
    neighbours.retain(|x| *x != fid.val);
    Ok(())
}
