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

#![deny(warnings)]

extern crate rust_3d;

use rust_3d::prelude::*;

#[test]
fn test_bounding_box_3d() {
    let mut pc1 = PointCloud3D::<Point3D>::new();
    let mut pc2 = PointCloud3D::<Point3D>::new();
    let mut pc3 = PointCloud3D::<Point3D>::new();
    let mut pc4 = PointCloud3D::<Point3D>::new();

    pc1.push(Point3D::new(0.0, 0.0, 0.0));
    pc1.push(Point3D::new(1.0, 1.0, 1.0));

    pc2.push(Point3D::new(0.0, 0.0, 0.0));
    pc2.push(Point3D::new(0.5, 0.5, 0.5));

    pc3.push(Point3D::new(-1.0, -1.0, -1.0));
    pc3.push(Point3D::new(2.0, 2.0, 2.0));

    pc4.push(Point3D::new(-10.0, -10.0, -10.0));
    pc4.push(Point3D::new(-11.0, -11.0, -11.0));

    let bb1 = pc1.bounding_box().unwrap();
    let bb2 = pc2.bounding_box().unwrap();
    let bb3 = pc3.bounding_box().unwrap();
    let bb4 = pc4.bounding_box().unwrap();

    assert!(bb1.min_p().x() == 0.0);
    assert!(bb1.min_p().y() == 0.0);
    assert!(bb1.min_p().z() == 0.0);
    assert!(bb1.max_p().x() == 1.0);
    assert!(bb1.max_p().y() == 1.0);
    assert!(bb1.max_p().z() == 1.0);

    assert!(bb3.min_p().x() == -1.0);
    assert!(bb3.min_p().y() == -1.0);
    assert!(bb3.min_p().z() == -1.0);
    assert!(bb3.max_p().x() == 2.0);
    assert!(bb3.max_p().y() == 2.0);
    assert!(bb3.max_p().z() == 2.0);

    assert!(!bb4.is_inside(&bb1));
    assert!(!bb4.is_inside(&bb2));
    assert!(!bb4.is_inside(&bb3));

    assert!(!bb1.is_inside(&bb2));
    assert!(!bb1.has_inside(&bb2));

    assert!(!bb2.is_inside(&bb1));
    assert!(!bb2.has_inside(&bb1));

    assert!(bb1.collides_with(&bb2));
    assert!(bb2.collides_with(&bb1));

    assert!(bb3.has_inside(&bb1));
    assert!(bb3.has_inside(&bb2));
    assert!(bb3.collides_with(&bb1));
    assert!(bb3.collides_with(&bb2));

    assert!(!bb1.contains(&Point3D::new(5.0, 5.0, 5.0)));
    assert!(bb1.contains(&Point3D::new(0.5, 0.5, 0.5)));
}
