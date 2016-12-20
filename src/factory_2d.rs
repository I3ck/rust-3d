use std::f64::consts::PI;

use point_2d::Point2D;
use point_cloud_2d::PointCloud2D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_editable_2d::IsEditable2D;
use functions::dist_2d;

///@todo entire file has to be added to tests
///@todo add some type level checks like diameter > 0 etc., or return Option types (similar to flaggedT?)
///@todo define trait for pc2d factories, later for 3d as well
///@todo remove center as param and create all around origin
///@todo correct reserving
///@todo order parameters (e.g. center and n_points always first)
///@todo some algorithms (e.g. bezier) can be ported to 3d, maybe write them directly generic over the dimension

pub fn origin() -> Box<Point2D> {
    Point2D::build(0.0, 0.0)
}

pub fn rectangle<P>(center: &P, width: f64, height: f64) -> Box<PointCloud2D<P>> where
    P: IsEditable2D + IsBuildable2D {

    let mut pc = PointCloud2D::new();
    pc.push(*P::build(center.x() - width / 2.0, center.y() - height / 2.0));
    pc.push(*P::build(center.x() + width / 2.0, center.y() - height / 2.0));
    pc.push(*P::build(center.x() + width / 2.0, center.y() + height / 2.0));
    pc.push(*P::build(center.x() - width / 2.0, center.y() + height / 2.0));
    Box::new(pc)
}

pub fn involut_circle<P>(center: &P, diameter: f64, n_points: usize, radians_start: f64, radians_end: f64) -> Box<PointCloud2D<P>> where
    P: IsEditable2D + IsBuildable2D {

    //@todo reserve
    let mut pc = PointCloud2D::new();
    let p_dist = (radians_end - radians_start).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let current = (i as f64) * p_dist;
        pc.push(*P::build(center.x() + diameter/2.0 * (current.cos() + current * current.sin()),
                          center.y() + diameter/2.0 * (current.sin() - current * current.cos())));
    }
    Box::new(pc)
}

pub fn arc<P>(center: &P, diameter: f64, n_points: usize, radians_start: f64, radians_end: f64) -> Box<PointCloud2D<P>> where
    P: IsEditable2D + IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let p_dist = (radians_end - radians_start).abs() / (n_points - 1) as f64;

    for i in 0..n_points {
        let radians = radians_start + (i as f64) * p_dist;
        pc.push(*P::build(center.x() + diameter/2.0 * radians_start.cos(),
                          center.y() + diameter/2.0 * radians_start.sin()));
    }
    Box::new(pc)
}

pub fn ellipse<P>(center: &P, a: f64, b: f64, n_points: usize) -> Box<PointCloud2D<P>> where
    P: IsEditable2D + IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let p_dist = PI / (n_points - 1) as f64;
    let angle: f64 = 0.0; //@todo as parameter? or just drop from formulas?

    for i in 0..n_points {
        let radians = (i as f64) * p_dist;
        pc.push(*P::build(center.x() + a * radians.cos() * angle.cos() - b * radians.sin() * angle.sin(),
                          center.y() + a * radians.cos() * angle.sin() + b * radians.sin() * angle.cos()));
    }
    Box::new(pc)
}

fn factorial(number: usize) -> usize {
    let mut result = 1;

    for i in 1..number+1 {
        result *= i;
    }
    result
}

fn binominal_coefficient(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n-k))
}

fn bernstein_polynomial(n: usize, i: usize, t: f64) -> f64 {
    (binominal_coefficient(n, i) as f64) * t.powi(i as i32) * (1.0 - t).powi((n-i) as i32) //@todo get rid of casts
}

fn control_polygon<P>(path: &PointCloud2D<P>, n_points: usize, t: f64) -> Box<P> where
    P: IsEditable2D + IsBuildable2D {

    let mut x : f64 = 0.0;
    let mut y : f64 = 0.0;

    //@todo possible bug with +1, c++ version had this, tho. Check again
    for i in 0..n_points+1 {
        let bp = bernstein_polynomial(n_points, i, t);
        x += bp * path.data[i].x();
        y += bp * path.data[i].y();
    }
    P::build(x,y)
}

pub fn interpolate_bezier<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Box<PointCloud2D<P>> where
    P: IsEditable2D + IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let p_dist = 1.0 / (n_points as f64);

    for i in 0..n_points {
        pc.push(*control_polygon(base_points, base_points.len()-1, (i as f64) * p_dist));
    }
    Box::new(pc)
}

pub fn interpolate_cosine<P>(base_points: &PointCloud2D<P>, n_points: usize) -> Box<PointCloud2D<P>> where
    P : IsEditable2D + IsBuildable2D {

    let mut pc = PointCloud2D::new();
    let p_dist = base_points.path_length() / (n_points - 1) as f64;

    for i in 0..n_points {
        let mut traveled : f64 = 0.0;
        let mut traveled_before : f64 = 0.0;

        for j in 1..base_points.len() {
            let ref p_prev = base_points.data[j-1];
            let ref p_now  = base_points.data[j];

            traveled += ( (p_now.x() - p_prev.x()).powi(2) + (p_now.y() - p_prev.y()).powi(2) ).sqrt();

            if traveled >= p_dist*(i as f64) {
                let proportion = ((i as f64)*p_dist - traveled_before) / (traveled - traveled_before);
                let proportion2 = (1.0 - (proportion*PI).cos() ) / 2.0;
                pc.push(*P::build(p_prev.x() + proportion * (p_now.x() - p_prev.x()),
                                  p_prev.y() * (1.0 - proportion2) + p_now.y()*proportion2));
                traveled_before = traveled;
                break;
            }
            traveled_before = traveled;
        }
    }
    Box::new(pc)

    }
