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

//! Douglas Peucker algorithm for 2D https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm

use prelude::*;

/// Douglas Peucker algorithm for 2D https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
pub fn douglas_peucker_2d<P>(mut pc: PointCloud2D<P>, epsilon: f64) -> PointCloud2D<P> where //@todo error if pc too small
    P: Is2D + Clone {

    if pc.len() < 1 { return pc; }

    let mut dmax = 0.0;
    let mut index = 0;
    let end = pc.len()-1;

    for i in 1..end {
        let d = distance_point_line(&pc.data[i], &pc.data[0], &pc.data[end]);
        if d > dmax {
            index = i;
            dmax = d;
        }
    }

    if dmax > epsilon {
        let mut pc1 = pc.clone();
        let mut pc2 = pc.clone();

        pc1.data = pc.data.iter().cloned().take(index).collect();
        pc1 = douglas_peucker_2d(pc1, epsilon);

        pc2.data = pc.data.into_iter().skip(index).collect();
        pc2 = douglas_peucker_2d(pc2, epsilon);

        let n_take = pc1.data.len() - 1;
        pc1.data = pc1.data.into_iter().take(n_take).collect();
        pc = pc1.combine(&pc2);
    }
    else {
        let p1 = pc.data[0].clone();
        let p2 = pc.data[end].clone();
        pc.data.clear();
        pc.data.push(p1);
        pc.data.push(p2);
    }
    return pc;
}

//@todo consider implementing in utils or via HasDistanceTo
fn distance_point_line<P,Q,R>(p: &P, l1: &Q, l2: &R) -> f64 where
    P: Is2D,
    Q: Is2D,
    R: Is2D {

    let a1 = l1.x();
    let a2 = l1.y();

    let b1 = l2.x();
    let b2 = l2.y();

    let c1 = p.x();
    let c2 = p.y();

    let x  =  (a1*a1*c1 - a1*a2*b2 + a1*a2*c2 - 2.0*a1*b1*c1 + a1*b2*b2 - a1*b2*c2 + a2*a2*b1 - a2*b1*b2 - a2*b1*c2 + b1*b1*c1 + b1*b2*c2)
            / (a1*a1 - 2.0*a1*b1 + a2*a2 - 2.0*a2*b2 + b1*b1 + b2*b2);

    let y  = ((a2 - b2) * x + a1*b2 - a2*b1) / (a1 - b1);

    return (  (x-p.x()).powi(2) + (y-p.y()).powi(2)  ).sqrt();
}
