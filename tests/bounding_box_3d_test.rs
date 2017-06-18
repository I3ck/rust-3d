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

extern crate rust_3d;

use rust_3d::traits::is_3d::*;
use rust_3d::traits::is_buildable_3d::*;
use rust_3d::traits::has_bounding_box_3d::*;
use rust_3d::traits::is_random_insertible::*;
use rust_3d::point_3d::*;
use rust_3d::point_cloud_3d::*;

#[test]
fn test_bounding_box_3d() {
    let mut pc1 = PointCloud3D::<Point3D>::new();
    let mut pc2 = PointCloud3D::<Point3D>::new();
    let mut pc3 = PointCloud3D::<Point3D>::new();
    let mut pc4 = PointCloud3D::<Point3D>::new();

    pc1.push(*Point3D::build(0.0, 0.0, 0.0));
    pc1.push(*Point3D::build(1.0, 1.0, 1.0));

    pc2.push(*Point3D::build(0.0, 0.0, 0.0));
    pc2.push(*Point3D::build(0.5, 0.5, 0.5));

    pc3.push(*Point3D::build(-1.0, -1.0, -1.0));
    pc3.push(*Point3D::build(2.0, 2.0, 2.0));

    pc4.push(*Point3D::build(-10.0, -10.0, -10.0));
    pc4.push(*Point3D::build(-11.0, -11.0, -11.0));

    assert!(pc1.min_pos().unwrap().x() == 0.0);
    assert!(pc1.min_pos().unwrap().y() == 0.0);
    assert!(pc1.min_pos().unwrap().z() == 0.0);
    assert!(pc1.max_pos().unwrap().x() == 1.0);
    assert!(pc1.max_pos().unwrap().y() == 1.0);
    assert!(pc1.max_pos().unwrap().z() == 1.0);

    assert!(pc3.min_pos().unwrap().x() == -1.0);
    assert!(pc3.min_pos().unwrap().y() == -1.0);
    assert!(pc3.min_pos().unwrap().z() == -1.0);
    assert!(pc3.max_pos().unwrap().x() == 2.0);
    assert!(pc3.max_pos().unwrap().y() == 2.0);
    assert!(pc3.max_pos().unwrap().z() == 2.0);

    assert!(!pc4.is_inside(&pc1).unwrap());
    assert!(!pc4.is_inside(&pc2).unwrap());
    assert!(!pc4.is_inside(&pc3).unwrap());

    assert!(!pc1.is_inside(&pc2).unwrap());
    assert!(!pc1.has_inside(&pc2).unwrap());

    assert!(!pc2.is_inside(&pc1).unwrap());
    assert!(!pc2.has_inside(&pc1).unwrap());

    assert!(pc1.collides_with(&pc2).unwrap());
    assert!(pc2.collides_with(&pc1).unwrap());

    assert!(pc3.has_inside(&pc1).unwrap());
    assert!(pc3.has_inside(&pc2).unwrap());
    assert!(pc3.collides_with(&pc1).unwrap());
    assert!(pc3.collides_with(&pc2).unwrap());

    assert!(!pc1.contains(&*Point3D::build(5.0, 5.0, 5.0)).unwrap());
    assert!(pc1.contains(&*Point3D::build(0.5, 0.5, 0.5)).unwrap());
}
