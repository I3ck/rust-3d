/*
Copyright 2016 Martin Buck
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

//! utility functions

use std::cmp::Ordering;

use prelude::*;

//@todo move these functions to better fitting files or make them methods of the correct types

/// Returns the center of two IsBuildable3D
pub fn center<P>(p1: &P, p2: &P) -> Box<P> where
    P: IsBuildable3D {

    P::new(
        p1.x() + (p2.x() - p1.x()) / 2.0,
        p1.y() + (p2.y() - p1.y()) / 2.0,
        p1.z() + (p2.z() - p1.z()) / 2.0
    )
}

/// Returns the cross product between a Is3D and a IsBuildable3D
pub fn cross<P,U>(first: &P, other: &U) -> Box<U> where
    P: Is3D,
    U: IsBuildable3D {

    let x = first.y() * other.z() - first.z() * other.y();
    let y = first.z() * other.x() - first.x() * other.z();
    let z = first.x() * other.y() - first.y() * other.x();
    U::new(x, y, z)
}

/// Compares two IsBuildable3D at a given dimensions
pub fn dimension_compare<P1, P2>(lhs: &P1, rhs: &P2, dim: i8) -> Result<Ordering> where
    P1: Is3D,
    P2: Is3D {

    match dim {
        0 => lhs.x().partial_cmp(&rhs.x()).ok_or(ErrorKind::ComparisionFailed),
        1 => lhs.y().partial_cmp(&rhs.y()).ok_or(ErrorKind::ComparisionFailed),
        2 => lhs.z().partial_cmp(&rhs.z()).ok_or(ErrorKind::ComparisionFailed),
        _ => Err(ErrorKind::DimensionsDontMatch)
    }
}

/// Calculates the distance within a given dimension between two IsBuildable3D
pub fn dimension_dist<P1, P2>(lhs: &P1, rhs: &P2, dim: i8) -> Result<f64> where
    P1: Is3D,
    P2: Is3D {

    match dim {
        0 => Ok((lhs.x() - rhs.x()).abs()),
        1 => Ok((lhs.y() - rhs.y()).abs()),
        2 => Ok((lhs.z() - rhs.z()).abs()),
        _ => Err(ErrorKind::DimensionsDontMatch)
    }
}

/// Helper function to sort a Vec of Is2D by x
pub fn sort_vec_2d_x<P>(xs: &mut Vec<P>) where
    P: Is2D {

        xs.sort_by(|a, b|
            a.x().partial_cmp(&b.x())
                .or_else(|| a.y().partial_cmp(&b.y()))
                .unwrap_or(Ordering::Equal));
}

/// Helper function to sort a Vec of Is2D by y
pub fn sort_vec_2d_y<P>(xs: &mut Vec<P>) where
    P: Is2D {

        xs.sort_by(|a, b|
            a.y().partial_cmp(&b.y())
                .or_else(|| a.x().partial_cmp(&b.x()))
                .unwrap_or(Ordering::Equal));
}

/// Helper function to sort a Vec of Is3D by x
pub fn sort_vec_3d_x<P>(xs: &mut Vec<P>) where
    P: Is3D {

        xs.sort_by(|a, b|
            a.x().partial_cmp(&b.x())
                .or_else(|| a.y().partial_cmp(&b.y()))
                .or_else(|| a.z().partial_cmp(&b.z()))
                .unwrap_or(Ordering::Equal));
}

/// Helper function to sort a Vec of Is3D by y
pub fn sort_vec_3d_y<P>(xs: &mut Vec<P>) where
    P: Is3D {

        xs.sort_by(|a, b|
            a.y().partial_cmp(&b.y())
                .or_else(|| a.z().partial_cmp(&b.z()))
                .or_else(|| a.x().partial_cmp(&b.x()))
                .unwrap_or(Ordering::Equal));
}

/// Helper function to sort a Vec of Is3D by z
pub fn sort_vec_3d_z<P>(xs: &mut Vec<P>) where
    P: Is3D {

        xs.sort_by(|a, b|
            a.z().partial_cmp(&b.z())
                .or_else(|| a.x().partial_cmp(&b.x()))
                .or_else(|| a.y().partial_cmp(&b.y()))
                .unwrap_or(Ordering::Equal));
}

//@todo move to plane or use there
/// Extrudes a 2D point cloud into 3D space with a given center and direction
pub fn extrude<P2,P3>(pc2d: &Vec<Box<P2>>, dir: &P3) -> (PointCloud3D<P3>, PointCloud3D<P3>) where
    P2: IsTransFormableTo3D,
    P3: IsBuildable3D + IsMovable3D + Clone {

    let mut pc_3d_a = PointCloud3D::new();
    let mut pc_3d_b = PointCloud3D::new();

    for p in pc2d {
        let p_transformed = p.transform_to_3d::<P3>(0.0);
        pc_3d_a.push(p_transformed.clone());
        pc_3d_b.push(p_transformed);
    }

    pc_3d_b.move_by(dir.x(), dir.y(), dir.z());
    (pc_3d_a, pc_3d_b)
}

//@todo rename or overload operators
//@todo implement for 2D as well, maybe move to traits
/// Calculates the vector between two positions
pub fn conn<P>(p_from: &P, p_to: &P) -> P where
    P: IsBuildable3D {

    *P::new(
        p_to.x() - p_from.x(),
        p_to.y() - p_from.y(),
        p_to.z() - p_from.z()
    )
}

/// Projects a point onto a plane
pub fn project_point_on_plane<PL,P2,P3,N>(plane: &PL, point: &P3) -> P2 where
    PL: IsPlane3D<P3,N>,
    P2: IsBuildable2D,
    P3: IsBuildable3D + IsTransFormableTo2D,
    N:  IsNormalized3D {

    let relative = conn(&plane.origin(), point);
    let mut p2transf = point.transform_to_2d::<P2>();
    let mut tmp = Point2D::default();

    tmp.set_x(plane.u().dot(&relative));
    tmp.set_y(plane.v().dot(&relative));

    p2transf.from(tmp);
    p2transf
}
