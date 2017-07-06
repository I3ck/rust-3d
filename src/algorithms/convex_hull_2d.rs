/*
Copyright 2017 Martin Buck
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

//! Convex hull algorithm returning a Vec of the hull where the points are ordered according to the hull
//! Using Andrew's monotone chain convex hull algorithm https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain

use point_cloud_2d::*;
use traits::is_2d::*;
use traits::is_buildable_2d::*;
use traits::is_sortable_2d::*;
use traits::is_random_accessible::*;

/// Convex hull algorithm returning a Vec of the hull where the points are ordered according to the hull
/// Using Andrew's monotone chain convex hull algorithm https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain
pub fn convex_hull_2d<RA, P>(ra: &RA) -> Vec<P> where
    RA: IsRandomAccessible<P>,
    P: Is2D + IsBuildable2D + Clone {
        let n = ra.len();

        let mut sorted = PointCloud2D::new();
        sorted.append_ra(ra);
        sorted.sort_x();
        let sorted = sorted;

        let mut lower = Vec::<P>::new();
        for i in 0..n {
            while lower.len() >= 2 && ccw(&lower[lower.len()-2], &lower[lower.len()-1], &sorted[i]) <= 0.0 {
                println!("pop");
                lower.pop().unwrap(); //safe, since only called if len > 0
            }
            lower.push(sorted[i].clone());
        }

        let mut upper = Vec::<P>::new();
        for i in (0..n).rev() {
            while upper.len() >= 2 && ccw(&upper[upper.len()-2], &upper[upper.len()-1], &sorted[i]) <= 0.0 {
                upper.pop().unwrap(); //safe, since only called if len > 0
            }
            upper.push(sorted[i].clone());
        }

        if lower.len() > 0 { lower.pop().unwrap(); } //safe, since len > 0
        if upper.len() > 0 { upper.pop().unwrap(); } //safe, since len > 0

        let mut result = Vec::<P>::new();
        result.extend(lower);
        result.extend(upper);
        result
}

fn ccw<P1, P2, P3>(p1: &P1, p2: &P2, p3: &P3) -> f64 where
    P1: Is2D,
    P2: Is2D,
    P3: Is2D {

    (p2.x() - p1.x())*(p3.y() - p1.y()) - (p2.y() - p1.y())*(p3.x() - p1.x())
}
