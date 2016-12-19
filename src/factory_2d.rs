use point_2d::Point2D;
use point_cloud_2d::PointCloud2D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_editable_2d::IsEditable2D;

///@todo entire file has to be added to tests
///@todo add some type level checks like diameter > 0 etc., or return Option types
///@todo define trait for pc2d factories, later for 3d as well


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
