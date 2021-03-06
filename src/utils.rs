/*
Copyright 2017 Martin Buck

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

//! internal utility functions

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use crate::*;

//------------------------------------------------------------------------------

/// Splits an ASCII line into its words, skipping empty elements
pub fn to_words_skip_empty(line: &[u8]) -> impl Iterator<Item = &[u8]> {
    line.split(|x| *x == b' ' || *x == b'\t').skip_empty()
}

/// Returns all until delimiter
pub fn until<'a>(line: &'a str, delimiter: &str) -> &'a str {
    line.split(delimiter).next().unwrap_or("")
}

/// Returns all until delimiter
pub fn until_bytes<'a>(line: &'a [u8], delimiter: u8) -> &'a [u8] {
    line.split(|x| *x == delimiter).next().unwrap_or(&[])
}

/// Max of two f64 values
pub fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}

/// Checks whether haystack contains needle
pub fn contains<T>(haystack: &[T], needle: &[T]) -> bool
where
    T: PartialEq,
{
    haystack.windows(needle.len()).any(|x| x == needle)
}

/// Max of three f64 values
pub fn max_f64_3(a: f64, b: f64, c: f64) -> f64 {
    max_f64(max_f64(a, b), c)
}

/// Generates the hash of an f64
#[inline(always)]
pub fn hash_f64<H>(x: f64, state: &mut H)
where
    H: Hasher,
{
    x.to_bits().hash(state);
}

/// Returns a container with duplicates removed and indices representing the original order
pub fn pack_dupes_indexed<'a, I, T>(idata: I) -> (Vec<T>, Vec<usize>)
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

/// Calculates the normals of a mesh
pub fn normals_of_mesh<P, M>(mesh: &M) -> Vec<Norm3D>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D + Default + Clone,
{
    let n = mesh.num_vertices();
    let nf = mesh.num_faces();
    let mut ns = vec![P::default(); n];

    for i in 0..nf {
        let face = mesh.face_vertex_ids(FId(i)).unwrap(); // safe
        let [v1, v2, v3] = mesh.face_vertices(FId(i)).unwrap(); // safe
        let v12 = conn(&v1, &v2);
        let v13 = conn(&v1, &v3);
        let n = Norm3D::new(cross(&v12, &v13)).unwrap_or(Norm3D::norm_z());
        for j in 0..3 {
            let new = add(&ns[face.vid(j).unwrap().0], &n); // safe since iterating 0..3
            ns[face.vid(j).unwrap().0] = new; // safe since iterating 0..3
        }
    }

    ns.into_iter()
        .map(|x| Norm3D::new(x).unwrap_or(Norm3D::norm_z()))
        .collect()
}

/// Estimates the used delimiter within a string
pub fn estimate_delimiter(minimum_count: usize, line: &[u8]) -> Option<u8> {
    for candidate in [b' ', b';', b',', b'\t'].iter() {
        if line.iter().filter(|c| **c == *candidate).count() >= minimum_count {
            return Some(*candidate);
        }
    }
    None
}

/// Adds two Is3D values
pub fn add<P, Q>(p: &P, q: &Q) -> P
where
    P: IsBuildable3D,
    Q: Is3D,
{
    P::new(p.x() + q.x(), p.y() + q.y(), p.z() + q.z())
}
