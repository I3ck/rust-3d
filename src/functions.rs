use std::cmp::Ordering;

use point_2d::{Point2D};
use point_3d::{Point3D};
use point_cloud_2d::{PointCloud2D};
use point_cloud_3d::{PointCloud3D};
use oc_node::{Direction};
use traits::is_2d::Is2D;
use traits::is_3d::Is3D;
use traits::has_position_2d::HasPosition2D;
use traits::has_position_3d::HasPosition3D;
use traits::has_editable_position_2d::HasEditablePosition2D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use traits::transformable_to_2d::TransFormableTo2D;
use traits::transformable_to_3d::TransFormableTo3D;
use traits::is_plane_3d::IsPlane3D;
use traits::is_normalized_3d::IsNormalized3D;
use traits::is_moveable_3d::IsMoveable3D;

pub fn center<P>(p1: &P, p2: &P) -> Box<P> where P: HasPosition3D {
    P::build(
        p1.x() + (p2.x() - p1.x()) / 2.0,
        p1.y() + (p2.y() - p1.y()) / 2.0,
        p1.z() + (p2.z() - p1.z()) / 2.0
    )
}

pub fn dist2D<P,U>(p1: &P, p2: &U) -> f64 where P: Is2D, U: Is2D {
    sqr_dist2D(p1,p2).sqrt()
}

pub fn dist3D<P,U>(p1: &P, p2: &U) -> f64 where P: Is3D, U: Is3D {
    sqr_dist3D(p1,p2).sqrt()
}

pub fn sqr_dist2D<P,U>(p1: &P, p2: &U) -> f64 where P: Is2D, U: Is2D {
    (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2)
}

pub fn sqr_dist3D<P,U>(p1: &P, p2: &U) -> f64 where P: Is3D, U: Is3D {
    (p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2) + (p1.z() - p2.z()).powi(2)
}

pub fn dimension_compare<P>(lhs: &P, rhs: &P, dim: i8) -> Option<Ordering> where P: HasPosition3D {
    match dim {
        0 => lhs.x().partial_cmp(&rhs.x()),
        1 => lhs.y().partial_cmp(&rhs.y()),
        2 => lhs.z().partial_cmp(&rhs.z()),
        _ => None
    }
}

pub fn dimension_dist<P>(lhs: &P, rhs: &P, dim: i8) -> Option<f64> where P: HasPosition3D {
    match dim {
        0 => Some((lhs.x() - rhs.x()).abs()),
        1 => Some((lhs.y() - rhs.y()).abs()),
        2 => Some((lhs.z() - rhs.z()).abs()),
        _ => None
    }
}

pub fn sort_and_limit<P>(mut pc: &mut PointCloud3D<P>, search: &P, maxSize: usize) where P: HasEditablePosition3D {
    if pc.len() > maxSize {
        pc.data.sort_by(|a, b| sqr_dist3D(search, &**a).partial_cmp(&sqr_dist3D(search, &**b)).unwrap_or(Ordering::Equal));
        let mut result : Vec<Box<P>>;
        result = Vec::new();
        for i in pc.data.iter().take(maxSize) {
            result.push(Box::new((*i).clone()));
        }
        pc.data = result;

    }
}
//@todo move to plane or use there
pub fn extrude<P2,P3>(pc2d: &Vec<Box<P2>>, dir: &P3) -> (PointCloud3D<P3>, PointCloud3D<P3>) where P2: HasPosition2D + TransFormableTo3D, P3: HasEditablePosition3D + IsMoveable3D {
    let mut pc3dA = PointCloud3D::new();
    let mut pc3dB = PointCloud3D::new();

    for p in pc2d {
        let pTransformed = p.transform_to_3D::<P3>(0.0);
        pc3dA.push(pTransformed.clone());
        pc3dB.push(pTransformed);
    }

    pc3dB.move_by(dir.x(), dir.y(), dir.z());
    (pc3dA, pc3dB)
}

pub fn calc_direction<P>(reference: &Point3D, p: &Point3D) -> Direction where P: HasPosition3D {
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

//@todo refactor to work with HasPosition3D?
pub fn calc_sub_min_max<P>(dir: Direction, min: &P, max: &P) -> (P, P) where P: HasPosition3D { //@todo better name
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

pub fn in_bb<P>(p: &P, min: &P, max: &P) -> bool where P: HasPosition3D {
    p.x() >= min.x() && p.x() <= max.x() &&
    p.y() >= min.y() && p.y() <= max.y() &&
    p.z() >= min.z() && p.z() <= max.z()
}

//@todo rename or overload operators
//@todo implement for 2D aswell, maybe move to traits
pub fn conn<P>(pFrom: &P, pTo: &P) -> P where P: HasPosition3D
{
    *P::build(
        pTo.x() - pFrom.x(),
        pTo.y() - pFrom.y(),
        pTo.z() - pFrom.z()
    )
}

pub fn project_point_on_plane<PL,P2,P3,N>(plane: &PL, point: &P3) -> P2 where PL: IsPlane3D<P3,N>, P2: HasPosition2D, P3: HasPosition3D + TransFormableTo2D, N: IsNormalized3D {
    let relative = conn(&plane.origin(), point);
    let mut p2transf = point.transform_to_2D::<P2>();
    let mut tmp = Point2D::new();

    tmp.set_x(plane.u().dot(&relative));
    tmp.set_y(plane.v().dot(&relative));

    p2transf.from(*tmp);
    p2transf
}
