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

use std::cmp::Ordering;

use result::*;
use point_2d::Point2D;
use point_3d::Point3D;
use point_cloud_2d::PointCloud2D;
use point_cloud_3d::PointCloud3D;
use oc_node::Direction;
use view::View;
use traits::is_nd::IsND;
use traits::is_2d::Is2D;
use traits::is_3d::Is3D;
use traits::is_buildable_nd::IsBuildableND;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_2d::IsEditable2D;
use traits::is_editable_3d::IsEditable3D;
use traits::transformable_to_2d::TransFormableTo2D;
use traits::transformable_to_3d::TransFormableTo3D;
use traits::is_plane_3d::IsPlane3D;
use traits::is_normalized_3d::IsNormalized3D;
use traits::is_moveable_3d::IsMoveable3D;

///@todo move these functions to better fitting files or make them methods of the correct types

pub fn center<P>(p1: &P, p2: &P) -> Box<P> where
    P: IsBuildable3D {

    P::build(
        p1.x() + (p2.x() - p1.x()) / 2.0,
        p1.y() + (p2.y() - p1.y()) / 2.0,
        p1.z() + (p2.z() - p1.z()) / 2.0
    )
}

pub fn dist<P,U>(p1: &P, p2: &U) -> Result<f64> where
    P: IsND,
    U: IsND {

    sqr_dist(p1,p2).map(|x| x.sqrt())
}

pub fn sqr_dist<P,U>(p1: &P, p2: &U) -> Result<f64> where
    P: IsND,
    U: IsND {

    if P::n_dimensions() != U::n_dimensions() {
        return Err(ErrorKind::DimensionsDontMatch);
    }

    let mut result : f64 = 0.0;
    for i in 0..P::n_dimensions() {
        if let (Ok(val1), Ok(val2)) = (p1.get_position(i), p2.get_position(i)) {
            result += (val1 - val2).powi(2);
        } else {
            return Err(ErrorKind::IncorrectDimension);
        }
    }
    Ok(result)
}

pub fn cross<P,U>(first: &P, other: &U) -> Box<U> where //@todo try to implement in Is3D trait
    P: Is3D,
    U: IsBuildable3D {

    let x = first.y() * other.z() - first.z() * other.y();
    let y = first.z() * other.x() - first.x() * other.z();
    let z = first.x() * other.y() - first.y() * other.x();
    U::build(x, y, z)
}

pub fn dist_nd<P, U>(p1: &Is2D, p2: &Is2D) -> f64 where
    P: IsND,
    U: IsND {

    sqr_dist_nd(p1,p2).sqrt()
}

pub fn dist_2d(p1: &Is2D, p2: &Is2D) -> f64 {
    sqr_dist_2d(p1,p2).sqrt()
}

pub fn dist_3d(p1: &Is3D, p2: &Is3D) -> f64 {
    sqr_dist_3d(p1,p2).sqrt()
}

pub fn sqr_dist_nd<P, U>(p1: &P, p2: &U) -> Result<f64> where
    P: IsND,
    U: IsND {

    if P::n_dimensions() != U::n_dimensions() {
        return Err(ErrorKind::DimensionsDontMatch);
    }
    let mut result : f64 = 0.0;
    for i in 0..P::n_dimensions() {
        result += (try!(p1.get_position(i)) - try!(p2.get_position(i))).powi(2);
    }
    Ok(result)
}

pub fn sqr_dist_2d(p1: &Is2D, p2: &Is2D) -> f64 {
    (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2)
}

pub fn sqr_dist_3d(p1: &Is3D, p2: &Is3D) -> f64 {
    (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2) + (p1.z() - p2.z()).powi(2)
}

pub fn dimension_compare<P>(lhs: &P, rhs: &P, dim: i8) -> Option<Ordering> where
    P: IsBuildable3D {

    match dim {
        0 => lhs.x().partial_cmp(&rhs.x()),
        1 => lhs.y().partial_cmp(&rhs.y()),
        2 => lhs.z().partial_cmp(&rhs.z()),
        _ => None
    }
}

pub fn dimension_dist<P>(lhs: &P, rhs: &P, dim: i8) -> Option<f64> where
    P: IsBuildable3D {

    match dim {
        0 => Some((lhs.x() - rhs.x()).abs()),
        1 => Some((lhs.y() - rhs.y()).abs()),
        2 => Some((lhs.z() - rhs.z()).abs()),
        _ => None
    }
}

pub fn sort_and_limit<P>(mut pc: &mut PointCloud3D<P>, search: &P, max_size: usize) where
    P: Is3D + Clone {

    if pc.len() > max_size {
        pc.data.sort_by(|a, b| sqr_dist_3d(search, &**a).partial_cmp(&sqr_dist_3d(search, &**b)).unwrap_or(Ordering::Equal));
        let mut result : Vec<Box<P>>;
        result = Vec::new();
        for i in pc.data.iter().take(max_size) {
            result.push(i.clone());
        }
        pc.data = result;
    }
}

//@todo move to plane or use there
pub fn extrude<P2,P3>(pc2d: &Vec<Box<P2>>, dir: &P3) -> (PointCloud3D<P3>, PointCloud3D<P3>) where
    P2: TransFormableTo3D,
    P3: IsBuildable3D + IsMoveable3D + Clone {

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

pub fn calc_direction<P>(reference: &Point3D, p: &Point3D) -> Direction where
    P: Is3D {

    if p.x() >= reference.x() && p.y() >= reference.y() && p.z() >= reference.z() {
        Direction::PPP
    } else if p.x() >= reference.x() && p.y() >= reference.y() && p.z() < reference.z() {
        Direction::PPN
    } else if p.x() >= reference.x() && p.y() < reference.y() && p.z() >= reference.z() {
        Direction::PNP
    } else if p.x() >= reference.x() && p.y() < reference.y() && p.z() < reference.z() {
        Direction::PNN
    } else if p.x() < reference.x() && p.y() >= reference.y() && p.z() >= reference.z() {
        Direction::NPP
    } else if p.x() < reference.x() && p.y() >= reference.y() && p.z() < reference.z() {
        Direction::NPN
    } else if p.x() >= reference.x() && p.y() < reference.y() && p.z() >= reference.z() {
        Direction::NNP
    } else { //if p.x() < reference.x() && p.y() < reference.y() && p.z() < reference.z() {
        Direction::NNN
    }
}

//@todo refactor to work with IsBuildable3D?
pub fn calc_sub_min_max<P>(dir: Direction, min: &P, max: &P) -> (P, P) where
    P: IsBuildable3D + Clone { //@todo better name

    let middle = center(min, max);

    let px = max.x();
    let py = max.y();
    let pz = max.z();
    let nx = min.x();
    let ny = min.y();
    let nz = min.z();
    let mx = middle.x();
    let my = middle.y();
    let mz = middle.z();

    match dir {
        Direction::PPP => (*middle,                 max.clone()),
        Direction::PPN => (*P::build(mx, my, nz),   *P::build(px, py, mz)),
        Direction::PNP => (*P::build(mx, ny, mz),   *P::build(px, my, pz)),
        Direction::PNN => (*P::build(mx, ny, nz),   *P::build(px, my, mz)),
        Direction::NPP => (*P::build(nx, my, mz),   *P::build(mx, py, pz)),
        Direction::NPN => (*P::build(nx, my, nz),   *P::build(mx, py, mz)),
        Direction::NNP => (*P::build(nx, ny, mz),   *P::build(mx, my, pz)),
        Direction::NNN => (min.clone(),             *middle)
    }
}

pub fn in_bb<P>(p: &P, min: &P, max: &P) -> bool where
    P: Is3D {

    p.x() >= min.x() && p.x() <= max.x() &&
    p.y() >= min.y() && p.y() <= max.y() &&
    p.z() >= min.z() && p.z() <= max.z()
}

//@todo rename or overload operators
//@todo implement for 2D aswell, maybe move to traits
pub fn conn<P>(p_from: &P, p_to: &P) -> P where
    P: IsBuildable3D {

    *P::build(
        p_to.x() - p_from.x(),
        p_to.y() - p_from.y(),
        p_to.z() - p_from.z()
    )
}

pub fn project_point_on_plane<PL,P2,P3,N>(plane: &PL, point: &P3) -> P2 where
    PL: IsPlane3D<P3,N>,
    P2: IsBuildable2D,
    P3: IsBuildable3D + TransFormableTo2D,
    N:  IsNormalized3D {

    let relative = conn(&plane.origin(), point);
    let mut p2transf = point.transform_to_2d::<P2>();
    let mut tmp = Point2D::new();

    tmp.set_x(plane.u().dot(&relative));
    tmp.set_y(plane.v().dot(&relative));

    p2transf.from(*tmp);
    p2transf
}

pub fn apply_view_2d<P>(view: View, pc: PointCloud2D<P>) -> PointCloud2D<P> where
    P: Is2D + Clone {

    match view {
        View::Full => { return pc; }
        View::Restricted(indices) => {
            let mut result = PointCloud2D::<P>::new();
            result.data.reserve(indices.len());
            let max = pc.len() - 1;

            for index in indices.into_iter() {
                if index > max {
                    continue;
                }
                result.push((*pc.data[index]).clone());
            }
            return result;
        }
    }
}

pub fn apply_view_3d<P>(view: View, pc: PointCloud3D<P>) -> PointCloud3D<P> where
    P: Is3D + Clone {

    match view {
        View::Full => { return pc; }
        View::Restricted(indices) => {
            let mut result = PointCloud3D::<P>::new();
            result.data.reserve(indices.len());
            let max = pc.len() - 1;

            for index in indices.into_iter() {
                if index > max {
                    continue;
                }
                result.push((*pc.data[index]).clone());
            }
            return result;
        }
    }
}
