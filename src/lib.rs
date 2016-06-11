pub mod traits;
pub mod functions;
pub mod point_2d;
pub mod point_3d;
pub mod plane_3d;
pub mod point_cloud_2d;
pub mod point_cloud_3d;
pub mod norm_2d;
pub mod norm_3d;
pub mod matrix4;
pub mod matrix4_pipe;
pub mod compressed_point_3d;
pub mod compressed_point_cloud_3d;
pub mod projection_to_plane;
pub mod kd_tree;
pub mod mesh_3d;
pub mod oc_node;
pub mod oc_tree;

#[cfg(test)]
pub mod tests {


    #[test]
    fn test_point_2d() {
        use std::f64::consts;
        use traits::is_2d::*;
        use traits::is_buildable_2d::*;
        use traits::is_editable_2d::*;
        use traits::is_moveable_2d::*;
        use point_2d::*;

        let eps = 0.0000001;
        let origin = *Point2D::new();


        let mut p1 = *Point2D::new();

        assert!(p1.x() == 0.0);
        assert!(p1.y() == 0.0);
        assert!(p1.abs() == 0.0);

        let mut p2 = *Point2D::build(1.0, 0.0);
        assert!(p2.x() == 1.0);
        assert!(p2.y() == 0.0);
        assert!(p2.abs() == 1.0);

        assert!(p1.rad_to(&p2) == 0.0);
        assert!(p2.rad_to(&p1) == consts::PI);

        let mut p3 = *Point2D::build(2.0, 2.0);
        assert!(p1.cross(&p2) == 0.0);
        assert!(p1.dot(&p2) == 0.0);
        assert!(p2.cross(&p3) == 2.0);
        assert!(p2.dot(&p3) == 2.0);

        assert!(p2.pos() == (p2.x(), p2.y()));
        let mut p2Clone = p2.clone();
        assert!(p2Clone.pos() == p2.pos());
        assert!(p2.to_str() == "1 0");

        p2Clone.from(p1.clone());
        assert!(p2Clone.pos() == p1.pos());

        let p1Norm = p1.normalized();
        assert!(p1Norm.is_none());

        let p3Norm = p3.normalized();
        assert!(p3Norm.is_some());

        match p3Norm {
            None => {},
            Some(n) => {
                assert!((1.0 - n.abs()).abs() < eps) ;
                assert!(n.x() == p3.x() / p3.abs());
                assert!(n.y() == p3.y() / p3.abs());
            }
        }

        p1.set_x(3.0);
        p1.set_y(10.0);
        assert!(p1.x() == 3.0);
        assert!(p1.y() == 10.0);

        p1.set_pos(3.0, 11.0);
        assert!(p1.x() == 3.0);
        assert!(p1.y() == 11.0);

        p2.set_pos(1.0, 2.0);
        p1.add(&p2);
        assert!(p1.x() == 4.0);
        assert!(p1.y() == 13.0);

        p1.substract(&p2);
        assert!(p1.x() == 3.0);
        assert!(p1.y() == 11.0);

        p1.scale(2.0);
        assert!(p1.x() == 6.0);
        assert!(p1.y() == 22.0);

        p1.set_pos(1.0, 0.0);
        p1.rotate(0.0, &origin);
        assert!(p1.x() == 1.0);
        assert!(p1.y() == 0.0);

        p1.rotate(2.0 * consts::PI, &origin);
        assert!((1.0 - p1.x()).abs() < eps);
        assert!((0.0 - p1.y()).abs() < eps);


        p1.rotate(consts::PI, &origin);
        assert!((-1.0 - p1.x()).abs() < eps);
        assert!((0.0 - p1.y()).abs() < eps);

        match Point2D::parse("1.3 7.9".to_string()) {
            None => assert!(false),
            Some(bp) => {
                assert!(bp.x() == 1.3);
                assert!(bp.y() == 7.9);
            }
        }

        p1.set_pos(1.0, 2.0);
        p1.move_by(0.1, 0.2);
        assert!(p1.x() == 1.1);
        assert!(p1.y() == 2.2);
    }


    #[test]
    fn test_point_3d() {
        use std::f64::consts;
        use traits::is_3d::*;
        use traits::is_buildable_3d::*;
        use traits::is_editable_3d::*;
        use traits::is_moveable_3d::*;
        use point_3d::*;

        let eps = 0.0000001;
        let origin = *Point3D::new();


        let mut p1 = *Point3D::new();

        assert!(p1.x() == 0.0);
        assert!(p1.y() == 0.0);
        assert!(p1.z() == 0.0);
        assert!(p1.abs() == 0.0);

        let mut p2 = *Point3D::build(1.0, 0.0, 0.0);
        assert!(p2.x() == 1.0);
        assert!(p2.y() == 0.0);
        assert!(p2.z() == 0.0);
        assert!(p2.abs() == 1.0);

        let mut p3 = *Point3D::build(2.0, 2.0, 2.0);

        let cross12: Point3D;
        cross12 = *p1.cross(&p2);
        assert!(cross12.x() == 0.0);
        assert!(cross12.y() == 0.0);
        assert!(cross12.z() == 0.0);

        assert!(p1.dot(&p2) == 0.0);

        let cross23: Point3D;
        cross23 = *p2.cross(&p3);
        assert!(cross23.x() == 0.0 * 2.0 - 0.0 * 2.0);
        assert!(cross23.y() == 0.0 * 2.0 - 1.0 * 2.0);
        assert!(cross23.z() == 1.0 * 2.0 - 0.0 * 2.0);


        assert!(p2.pos() == (p2.x(), p2.y(), p2.z()));
        let mut p2Clone = p2.clone();
        assert!(p2Clone.pos() == p2.pos());
        assert!(p2.to_str() == "1 0 0");

        p2Clone.from(p1.clone());
        assert!(p2Clone.pos() == p1.pos());

        let p1Norm = p1.normalized();
        assert!(p1Norm.is_none());

        let p3Norm = p3.normalized();
        assert!(p3Norm.is_some());

        match p3Norm {
            None => {},
            Some(n) => {
                assert!((1.0 - n.abs()).abs() < eps) ;
                assert!(n.x() == p3.x() / p3.abs());
                assert!(n.y() == p3.y() / p3.abs());
                assert!(n.z() == p3.z() / p3.abs());
            }
        }

        p1.set_x(3.0);
        p1.set_y(10.0);
        p1.set_z(11.0);
        assert!(p1.x() == 3.0);
        assert!(p1.y() == 10.0);
        assert!(p1.z() == 11.0);

        p1.set_pos(3.0, 11.0, 14.0);
        assert!(p1.x() == 3.0);
        assert!(p1.y() == 11.0);
        assert!(p1.z() == 14.0);

        p2.set_pos(1.0, 2.0, 3.0);
        p1.add(&p2);
        assert!(p1.x() == 4.0);
        assert!(p1.y() == 13.0);
        assert!(p1.z() == 17.0);

        p1.substract(&p2);
        assert!(p1.x() == 3.0);
        assert!(p1.y() == 11.0);
        assert!(p1.z() == 14.0);

        p1.scale(2.0);
        assert!(p1.x() == 6.0);
        assert!(p1.y() == 22.0);
        assert!(p1.z() == 28.0);

        match Point3D::parse("1.3 7.9 13.7".to_string()) {
            None => assert!(false),
            Some(bp) => {
                assert!(bp.x() == 1.3);
                assert!(bp.y() == 7.9);
                assert!(bp.z() == 13.7);
            }
        }

        p1.set_pos(1.0, 2.0, 3.0);
        p1.move_by(0.1, 0.2, 0.3);
        assert!(p1.x() == 1.1);
        assert!(p1.y() == 2.2);
        assert!(p1.z() == 3.3);

        //@todo missing tests for matrix multiplication
    }

    #[test]
    fn test_point_cloud_2d() {
        use std::f64::consts;
        use traits::is_2d::*;
        use traits::is_buildable_2d::*;
        use traits::is_editable_2d::*;
        use traits::is_moveable_2d::*;
        use traits::has_bounding_box_2d::*;
        use point_2d::*;
        use point_cloud_2d::*;

        let mut pc = PointCloud2D::<Point2D>::new();

        assert!(pc.len() == 0);

        let p = *Point2D::build(0.1, 0.2);
        pc.push(p);

        assert!(pc.len() == 1);
        assert!(pc.data[0].x() == 0.1);
        assert!(pc.data[0].y() == 0.2);

        assert!(pc.bounding_box().is_none());

        let p = *Point2D::build(0.2, 0.3);
        pc.push(p);
        assert!(pc.len() == 2);

        assert!(pc.bounding_box().is_some());

        match pc.bounding_box() {
            None => assert!(false),
            Some((bbmin, bbmax)) => {
                assert!(bbmin.x() == 0.1);
                assert!(bbmin.y() == 0.2);
                assert!(bbmax.x() == 0.2);
                assert!(bbmax.y() == 0.3);
            }
        }
        assert!(pc.to_str() == "0.1 0.2\n0.2 0.3\n");

        match PointCloud2D::<Point2D>::parse(pc.to_str()) {
            None => assert!(false),
            Some(pcparsed) => assert!(pcparsed.to_str() == "0.1 0.2\n0.2 0.3\n")
        };

        let pccloned = pc.clone();
        assert!(pccloned.to_str() == "0.1 0.2\n0.2 0.3\n");

        pc.move_by(1.0, 2.0);
        println!("pc: {}", pc);
        assert!(pc.to_str() == "1.1 2.2\n1.2 2.3\n");
    }


    #[test]
    fn test_point_cloud_3d() {
        use std::f64::consts;
        use traits::is_3d::*;
        use traits::is_buildable_3d::*;
        use traits::is_editable_3d::*;
        use traits::is_moveable_3d::*;
        use traits::has_bounding_box_3d::*;
        use point_3d::*;
        use point_cloud_3d::*;

        let mut pc = PointCloud3D::<Point3D>::new();

        assert!(pc.len() == 0);

        let p = *Point3D::build(0.1, 0.2, 0.3);
        pc.push(p);

        assert!(pc.len() == 1);
        assert!(pc.data[0].x() == 0.1);
        assert!(pc.data[0].y() == 0.2);
        assert!(pc.data[0].z() == 0.3);

        assert!(pc.bounding_box().is_none());

        let p = *Point3D::build(0.2, 0.3, 0.4);
        pc.push(p);
        assert!(pc.len() == 2);

        assert!(pc.bounding_box().is_some());

        match pc.bounding_box() {
            None => assert!(false),
            Some((bbmin, bbmax)) => {
                assert!(bbmin.x() == 0.1);
                assert!(bbmin.y() == 0.2);
                assert!(bbmin.z() == 0.3);
                assert!(bbmax.x() == 0.2);
                assert!(bbmax.y() == 0.3);
                assert!(bbmax.z() == 0.4);
            }
        }
        assert!(pc.to_str() == "0.1 0.2 0.3\n0.2 0.3 0.4\n");

        match PointCloud3D::<Point3D>::parse(pc.to_str()) {
            None => assert!(false),
            Some(pcparsed) => assert!(pcparsed.to_str() == "0.1 0.2 0.3\n0.2 0.3 0.4\n")
        };

        let pccloned = pc.clone();
        assert!(pccloned.to_str() == "0.1 0.2 0.3\n0.2 0.3 0.4\n");

        pc.move_by(1.0, 2.0, 3.0);
        println!("pc: {}", pc);
        assert!(pc.to_str() == "1.1 2.2 3.3\n1.2 2.3 3.4\n");
    }
}
