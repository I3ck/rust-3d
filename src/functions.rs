/*
Copyright 2016 Martin Buck

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

//! utility functions

use std::{cmp::Ordering, ops::Sub};

use crate::{distances_3d::sqr_dist_3d, prelude::*};

//@todo move these functions to better fitting files or make them methods of the correct types

/// Returns the center of two IsBuildable2D
pub fn center_2d<P>(p1: &P, p2: &P) -> P
where
    P: IsBuildable2D,
{
    P::new(
        p1.x() + (p2.x() - p1.x()) / 2.0,
        p1.y() + (p2.y() - p1.y()) / 2.0,
    )
}

/// Returns the center of two IsBuildable3D
pub fn center_3d<P>(p1: &P, p2: &P) -> P
where
    P: IsBuildable3D,
{
    P::new(
        p1.x() + (p2.x() - p1.x()) / 2.0,
        p1.y() + (p2.y() - p1.y()) / 2.0,
        p1.z() + (p2.z() - p1.z()) / 2.0,
    )
}

/// Returns the cross product between a Is3D and a IsBuildable3D
pub fn cross<P, U>(first: &P, other: &U) -> U
where
    P: Is3D,
    U: IsBuildable3D,
{
    let x = first.y() * other.z() - first.z() * other.y();
    let y = first.z() * other.x() - first.x() * other.z();
    let z = first.x() * other.y() - first.y() * other.x();
    U::new(x, y, z)
}

/// Compares two IsBuildable3D at a given dimensions
pub fn dimension_compare<P1, P2>(lhs: &P1, rhs: &P2, dim: i8) -> Result<Ordering>
where
    P1: Is3D,
    P2: Is3D,
{
    match dim {
        0 => lhs
            .x()
            .partial_cmp(&rhs.x())
            .ok_or(ErrorKind::ComparisionFailed),
        1 => lhs
            .y()
            .partial_cmp(&rhs.y())
            .ok_or(ErrorKind::ComparisionFailed),
        2 => lhs
            .z()
            .partial_cmp(&rhs.z())
            .ok_or(ErrorKind::ComparisionFailed),
        _ => Err(ErrorKind::DimensionsDontMatch),
    }
}

/// Calculates the distance within a given dimension between two IsBuildable3D
pub fn dimension_dist<P1, P2>(lhs: &P1, rhs: &P2, dim: i8) -> Result<f64>
where
    P1: Is3D,
    P2: Is3D,
{
    match dim {
        0 => Ok((lhs.x() - rhs.x()).abs()),
        1 => Ok((lhs.y() - rhs.y()).abs()),
        2 => Ok((lhs.z() - rhs.z()).abs()),
        _ => Err(ErrorKind::DimensionsDontMatch),
    }
}

/// Helper function to sort a Vec of Is2D by x
pub fn sort_vec_2d_x<P>(xs: &mut Vec<P>)
where
    P: Is2D,
{
    xs.sort_by(|a, b| {
        a.x()
            .partial_cmp(&b.x())
            .or_else(|| a.y().partial_cmp(&b.y()))
            .unwrap_or(Ordering::Equal)
    });
}

/// Helper function to sort a Vec of Is2D by y
pub fn sort_vec_2d_y<P>(xs: &mut Vec<P>)
where
    P: Is2D,
{
    xs.sort_by(|a, b| {
        a.y()
            .partial_cmp(&b.y())
            .or_else(|| a.x().partial_cmp(&b.x()))
            .unwrap_or(Ordering::Equal)
    });
}

/// Helper function to sort a Vec of Is3D by x
pub fn sort_vec_3d_x<P>(xs: &mut Vec<P>)
where
    P: Is3D,
{
    xs.sort_by(|a, b| {
        a.x()
            .partial_cmp(&b.x())
            .or_else(|| a.y().partial_cmp(&b.y()))
            .or_else(|| a.z().partial_cmp(&b.z()))
            .unwrap_or(Ordering::Equal)
    });
}

/// Helper function to sort a Vec of Is3D by y
pub fn sort_vec_3d_y<P>(xs: &mut Vec<P>)
where
    P: Is3D,
{
    xs.sort_by(|a, b| {
        a.y()
            .partial_cmp(&b.y())
            .or_else(|| a.z().partial_cmp(&b.z()))
            .or_else(|| a.x().partial_cmp(&b.x()))
            .unwrap_or(Ordering::Equal)
    });
}

/// Helper function to sort a Vec of Is3D by z
pub fn sort_vec_3d_z<P>(xs: &mut Vec<P>)
where
    P: Is3D,
{
    xs.sort_by(|a, b| {
        a.z()
            .partial_cmp(&b.z())
            .or_else(|| a.x().partial_cmp(&b.x()))
            .or_else(|| a.y().partial_cmp(&b.y()))
            .unwrap_or(Ordering::Equal)
    });
}

//@todo move to plane or use there
/// Extrudes a 2D point cloud into 3D space with a given center and direction
pub fn extrude<P2, P3>(pc2d: &Vec<P2>, dir: &P3) -> (PointCloud3D<P3>, PointCloud3D<P3>)
where
    P2: IsTransFormableTo3D,
    P3: IsBuildable3D + IsMovable3D + Clone,
{
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
pub fn conn<P>(p_from: &P, p_to: &P) -> P
where
    P: IsBuildable3D,
{
    P::new(
        p_to.x() - p_from.x(),
        p_to.y() - p_from.y(),
        p_to.z() - p_from.z(),
    )
}

/// Positions the object in such a way that its center is at origin
pub fn center<T>(x: &mut T)
where
    T: HasBoundingBox3DMaybe + IsMovable3D,
{
    if let Ok(bb) = x.bounding_box_maybe() {
        let center = bb.center_bb();
        x.move_by(-center.x(), -center.y(), -center.z());
    }
}

/// Scales the object to the required size
pub fn set_size<T>(x: &mut T, size: [Positive; 3])
where
    T: HasBoundingBox3DMaybe + IsMatrix4Transformable,
{
    if let Ok(bb) = x.bounding_box_maybe() {
        let m = Matrix4::scale(
            size[0].get() / bb.size_x().get(),
            size[1].get() / bb.size_y().get(),
            size[2].get() / bb.size_z().get(),
        );
        x.transform(&m);
    }
}

/// Collects all intersections between a ray and mesh
pub fn collect_intersections_ray_mesh<P, M>(ray: &Ray3D, mesh: &M, intersections: &mut Vec<P>)
where
    M: IsMesh<P, Face3>,
    P: IsBuildable3D + Sub<Output = P> + Clone,
{
    let nf = mesh.num_faces();

    for i in 0..nf {
        let [v1, v2, v3] = mesh.face_vertices(FId { val: i }).unwrap(); // safe
                                                                        //println!("face_vertices");
        if let Some(intersection) = intersection_ray_triangle(ray, &v1, &v2, &v3) {
            intersections.push(intersection);
        }
    }
}

//@todo more generic types?
//@todo many clones required, check whether sub add etc. can be implemented more efficently
/// Finds the intersection between a ray and triangle
pub fn intersection_ray_triangle<P>(ray: &Ray3D, v1: &P, v2: &P, v3: &P) -> Option<P>
where
    P: IsBuildable3D + Sub<Output = P> + Clone,
{
    let orig = &ray.line.anchor;
    let dir = &ray.line.dir;
    let n = normal_of_face(v1, v2, v3);

    let w1 = orig.clone() - v1.clone();
    let a = -n.dot(&w1);
    let b = n.dot(dir);

    if b == 0.0 {
        return None;
    } //@todo eps

    let r = a / b;

    if r <= 0.0 {
        return None;
    }

    let p = orig.clone() + dir.clone() * r;

    let e1 = v2.clone() - v1.clone();
    let vp1 = p.clone() - v1.clone();
    if n.dot(&cross(&e1, &vp1)) <= 0.0 {
        return None;
    }

    let e2 = v3.clone() - v2.clone();
    let vp2 = p.clone() - v2.clone();
    if n.dot(&cross(&e2, &vp2)) <= 0.0 {
        return None;
    }

    let e3 = v1.clone() - v3.clone();
    let vp3 = p.clone() - v3.clone();
    if n.dot(&cross(&e3, &vp3)) <= 0.0 {
        return None;
    }

    Some(P::new_from(&p))
}

/// Applies the function to each intersection candidate
pub fn for_each_intersecting<'c, I, HB>(
    ray: &Ray3D,
    hbs: I,
    f: &mut dyn FnMut(&Point3D, &'c mut HB),
) where
    I: Iterator<Item = &'c mut HB>,
    HB: HasBoundingBox3DMaybe,
{
    for hb in hbs {
        if let Ok(bb) = hb.bounding_box_maybe() {
            if let Some(i) = intersection(&ray.line, &bb) {
                f(&i, hb)
            }
        }
    }
}

/// Returns the closest intersection with the ray
pub fn closest_intersecting_mut<'c, I, HB>(ray: &Ray3D, hbs: I) -> Option<(Point3D, &'c mut HB)>
where
    I: Iterator<Item = &'c mut HB>,
    HB: HasBoundingBox3DMaybe,
{
    let mut result: Option<(Point3D, &'c mut HB)> = None;

    for hb in hbs {
        if let Ok(bb) = hb.bounding_box_maybe() {
            if let Some(i) = intersection(&ray.line, &bb) {
                if let Some(r) = &result {
                    if sqr_dist_3d(&ray.line.anchor, &i) < sqr_dist_3d(&ray.line.anchor, &r.0) {
                        result = Some((i, hb))
                    }
                } else {
                    result = Some((i, hb))
                }
            }
        }
    }

    result
}

/// Returns the closest intersection with the ray
pub fn closest_intersecting<'c, I, HB>(ray: &Ray3D, hbs: I) -> Option<(Point3D, &'c HB)>
where
    I: Iterator<Item = &'c HB>,
    HB: HasBoundingBox3DMaybe,
{
    let mut result: Option<(Point3D, &'c HB)> = None;

    for hb in hbs {
        if let Ok(bb) = hb.bounding_box_maybe() {
            if let Some(i) = intersection(&ray.line, &bb) {
                if let Some(r) = &result {
                    if sqr_dist_3d(&ray.line.anchor, &i) < sqr_dist_3d(&ray.line.anchor, &r.0) {
                        result = Some((i, hb))
                    }
                } else {
                    result = Some((i, hb))
                }
            }
        }
    }

    result
}

/// Returns the index of the closest intersection with the ray
pub fn index_closest_intersecting<'c, I, HB>(ray: &Ray3D, hbs: I) -> Option<(Point3D, usize)>
where
    I: Iterator<Item = &'c HB>,
    HB: 'c + HasBoundingBox3DMaybe,
{
    let mut result: Option<(Point3D, usize)> = None;

    for (i, hb) in hbs.enumerate() {
        if let Ok(bb) = hb.bounding_box_maybe() {
            if let Some(inter) = intersection(&ray.line, &bb) {
                if let Some(r) = &result {
                    if sqr_dist_3d(&ray.line.anchor, &inter) < sqr_dist_3d(&ray.line.anchor, &r.0) {
                        result = Some((inter, i))
                    }
                } else {
                    result = Some((inter, i))
                }
            }
        }
    }

    result
}

/// Calculates the normal of a face given by three vertices
pub fn normal_of_face<P>(v1: &P, v2: &P, v3: &P) -> Norm3D
where
    P: IsBuildable3D,
{
    let v12 = conn(v1, v2);
    let v23 = conn(v2, v3);
    Norm3D::new(cross(&v12, &v23)).unwrap_or(Norm3D::norm_z())
}

/// Projects a point onto a plane
pub fn project_point_on_plane<PL, P2, P3, N>(plane: &PL, point: &P3) -> P2
where
    PL: IsPlane3D<P3, N>,
    P2: IsBuildable2D,
    P3: IsBuildable3D + IsTransFormableTo2D,
    N: IsNormalized3D,
{
    let relative = conn(&plane.origin(), point);
    let mut p2transf = point.transform_to_2d::<P2>();
    let mut tmp = Point2D::default();

    tmp.set_x(plane.u().dot(&relative));
    tmp.set_y(plane.v().dot(&relative));

    p2transf.from(&tmp);
    p2transf
}

/// Minimum of two f64 values
pub fn min64(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

/// Maximum of two f64 values
pub fn max64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

//@todo better location and as trait?
/// The intersection between a line and BoundingBox if there is any
pub fn intersection(l: &Line3D, b: &BoundingBox3D) -> Option<Point3D> {
    let inv_dir = [1.0 / l.dir.x(), 1.0 / l.dir.y(), 1.0 / l.dir.z()];
    let min = b.min_p();
    let max = b.max_p();

    let tx1 = (min.x() - l.anchor.x()) * inv_dir[0];
    let tx2 = (max.x() - l.anchor.x()) * inv_dir[0];

    let mut tmin = min64(tx1, tx2);
    let mut tmax = max64(tx1, tx2);

    let ty1 = (min.y() - l.anchor.y()) * inv_dir[1];
    let ty2 = (max.y() - l.anchor.y()) * inv_dir[1];

    tmin = max64(tmin, min64(ty1, ty2));
    tmax = min64(tmax, max64(ty1, ty2));

    let tz1 = (min.z() - l.anchor.z()) * inv_dir[2];
    let tz2 = (max.z() - l.anchor.z()) * inv_dir[2];

    tmin = max64(tmin, min64(tz1, tz2));
    tmax = min64(tmax, max64(tz1, tz2));

    if tmax >= tmin && tmax >= 0.0 {
        Some(l.anchor.clone() + l.dir.clone() * tmin) //@todo avoid cloning
    } else {
        None
    }
}
