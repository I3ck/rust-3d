/*
Copyright 2020 Martin Buck

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

#![deny(warnings)]

use rust_3d::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

//------------------------------------------------------------------------------

fn get_vertices<V, TU, M>(mesh: &M) -> f64
where
    TU: IsTopologyUnit,
    V: Is3D,
    M: IsMesh<V, TU>,
{
    let n = mesh.num_vertices();
    let mut sum_x = 0.0;

    for i in 0..n {
        if let Some(v) = mesh.vertex(VId(i)) {
            sum_x += v.x();
        }
    }

    sum_x
}

fn get_vertices_benchmark(c: &mut Criterion) {
    let mut mesh = Mesh3D::<Point3D, PointCloud3D<Point3D>, Vec<usize>>::default();
    for i in 0..10000000 {
        let f = i as f64;
        let v1 = Point3D::new(f, 2.0 * f, 3.0 * f);
        let v2 = Point3D::new(2.0 * f, f, 3.0 * f);
        let v3 = Point3D::new(f, f, f);
        mesh.add_face(v1, v2, v3);
    }
    c.bench_function("mesh get_vertices", |b| {
        b.iter(|| get_vertices(black_box(&mesh)))
    });
}

criterion_group!(benches, get_vertices_benchmark);
criterion_main!(benches);

//------------------------------------------------------------------------------
