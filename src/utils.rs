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

/// Splits a line into its words
pub fn to_words(line: &str) -> SkipEmptyString<'_, std::str::Split<'_, &str>> {
    line.trim().split(" ").skip_empty_string()
}

/// Returns all until delimiter
pub fn until<'a>(line: &'a str, delimiter: &str) -> &'a str {
    line.split(delimiter).next().unwrap_or("")
}

/// Max of two f64 values
pub fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}

/// Max of three f64 values
pub fn max_f64_3(a: f64, b: f64, c: f64) -> f64 {
    max_f64(max_f64(a, b), c)
}

/// Generates the hash of an f64
pub fn hash_f64<H>(x: f64, state: &mut H)
where
    H: Hasher,
{
    let (m, e, s) = integer_decode(x);
    m.hash(state);
    e.hash(state);
    s.hash(state);
}

/// Returns the mantissa, exponent and sign as integers.
/// taken from https://github.com/rust-lang/rust/blob/5c674a11471ec0569f616854d715941757a48a0a/src/libcore/num/f64.rs#L203-L216
fn integer_decode(x: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { std::mem::transmute(x) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
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

pub fn normals_of_mesh<P, M>(mesh: &M) -> Vec<Norm3D>
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D + Default + Clone,
{
    let n = mesh.num_vertices();
    let nf = mesh.num_faces();
    let mut ns = vec![P::default(); n]; //@todo Vec3D type?

    for i in 0..nf {
        let face = mesh.face_vertex_ids(FId { val: i }).unwrap(); // safe
        let [v1, v2, v3] = mesh.face_vertices(FId { val: i }).unwrap(); // safe
        let v12 = conn(&v1, &v2);
        let v13 = conn(&v1, &v3);
        let n = Norm3D::new(cross(&v12, &v13)).unwrap_or(Norm3D::norm_z());
        for j in 0..3 {
            let new = add(&ns[face.vid(j).unwrap().val], &n); // safe since iterating 0..3
            ns[face.vid(j).unwrap().val] = new; // safe since iterating 0..3
        }
    }

    ns.into_iter()
        .map(|x| Norm3D::new(x).unwrap_or(Norm3D::norm_z()))
        .collect()
}

pub fn estimate_delimiter(minimum_count: usize, line: &str) -> Option<char> {
    for candidate in [' ', ';', ','].iter() {
        if line.chars().filter(|c| *c == *candidate).count() >= minimum_count {
            return Some(*candidate);
        }
    }
    None
}

pub fn add<P, Q>(p: &P, q: &Q) -> P
where
    P: IsBuildable3D,
    Q: Is3D,
{
    P::new(p.x() + q.x(), p.y() + q.y(), p.z() + q.z())
}
