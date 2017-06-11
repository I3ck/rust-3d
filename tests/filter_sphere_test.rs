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

use rust_3d::traits::is_buildable_3d::*;
use rust_3d::traits::is_filter_3d::*;
use rust_3d::point_3d::*;
use rust_3d::positive::*;
use rust_3d::filters::filter_sphere::*;

#[test]
fn filter_sphere_test() {
    let center = *Point3D::build(10.0, -5.0, 1.0);
    let radius = Positive::new(3.0).unwrap();
    let filter = FilterSphere::build(center, radius);

    assert!( filter.is_allowed(&*Point3D::build(10.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(9.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(8.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(7.0, -5.0, 1.0)));
    assert!(!filter.is_allowed(&*Point3D::build(6.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(11.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(12.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(13.0, -5.0, 1.0)));
    assert!(!filter.is_allowed(&*Point3D::build(14.0, -5.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -4.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -3.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -2.0, 1.0)));
    assert!(!filter.is_allowed(&*Point3D::build(10.0, -1.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -6.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -7.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -8.0, 1.0)));
    assert!(!filter.is_allowed(&*Point3D::build(10.0, -9.0, 1.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -5.0, 0.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -5.0, -1.0)));
    assert!(!filter.is_allowed(&*Point3D::build(10.0, -5.0, -3.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -5.0, 2.0)));
    assert!( filter.is_allowed(&*Point3D::build(10.0, -5.0, 3.0)));
    assert!(!filter.is_allowed(&*Point3D::build(10.0, -5.0, 5.0)));
}
