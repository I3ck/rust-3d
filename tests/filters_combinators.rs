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
use rust_3d::traits::is_filter::*;
use rust_3d::traits::is_filter_random_accessible::*;
use rust_3d::traits::is_random_accessible::*;
use rust_3d::traits::is_view_buildable::*;
use rust_3d::point_3d::*;
use rust_3d::point_cloud_3d::*;
use rust_3d::positive::*;
use rust_3d::filters::filter_box_3d::*;
use rust_3d::filters::filter_sphere::*;
use rust_3d::filters::combinators::filter_negate::*;
use rust_3d::filters::combinators::filter_or::*;
use rust_3d::filters::combinators::filter_and::*;
use rust_3d::filters::combinators::filter_xor::*;
use rust_3d::filters::combinators::filter_all::*;
use rust_3d::filters::combinators::filter_any::*;
use rust_3d::filters::combinators::filter_outer_inner::*;
use rust_3d::filters::combinators::filter_allow::*;
use rust_3d::filters::combinators::filter_deny::*;
use rust_3d::filters::transformers::filter_random_accessible::*;
use rust_3d::view::*;
use rust_3d::io::xyz::*;
use rust_3d::test_helper::*;

#[test]
fn filter_or_test() {
    let filterSphere = FilterSphere::new(*Point3D::new(14.0, 14.0, 14.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::new(*Point3D::new(4.0, 4.0, 4.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let filter = FilterOR::new(filterSphere, filterBox);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_or.xyz", "or");
}

#[test]
fn filter_and_test() {
    let filterSphere = FilterSphere::new(*Point3D::new(10.0, 10.0, 10.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::new(*Point3D::new(8.0, 8.0, 8.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let filter = FilterAND::new(filterSphere, filterBox);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_and.xyz", "and");
}

#[test]
fn filter_xor_test() {
    let filterSphere = FilterSphere::new(*Point3D::new(13.0, 13.0, 13.0), Positive::new(10.0).unwrap());
    let filterBox    = FilterBox3D::new(*Point3D::new(8.0, 8.0, 8.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(30.0).unwrap());
    let filter = FilterXOR::new(filterSphere, filterBox);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_xor.xyz", "xor");
}

#[test]
fn filter_all_test() {
    let filterSphere = FilterSphere::new(*Point3D::new(10.0, 10.0, 10.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::new(*Point3D::new(8.0, 8.0, 8.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let mut filter = FilterAll::new();
    filter.filters.push(Box::new(filterSphere));
    filter.filters.push(Box::new(filterBox));
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_and.xyz", "and"); //same as the and test
}

#[test]
fn filter_any_test() {
    let filterSphere = FilterSphere::new(*Point3D::new(14.0, 14.0, 14.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::new(*Point3D::new(4.0, 4.0, 4.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let mut filter = FilterAny::new();
    filter.filters.push(Box::new(filterSphere));
    filter.filters.push(Box::new(filterBox));
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_or.xyz", "or"); //same as the or test
}

#[test]
fn filter_outer_inner_test() {
    let center      = *Point3D::new(10.0, 10.0, 10.0);
    let filterOuter = FilterSphere::new(center.clone(), Positive::new(4.0).unwrap());
    let filterInner = FilterSphere::new(center,         Positive::new(3.0).unwrap());
    let filter      = FilterOuterInner::new(filterOuter, filterInner);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_outer_inner.xyz", "outer_inner");
}

#[test]
fn filter_negate_test() {
    let center       = *Point3D::new(10.0, 10.0, 10.0);
    let filterSphere = FilterSphere::new(center.clone(), Positive::new(10.0).unwrap());
    let filter       = FilterNegate::new(filterSphere);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_negate.xyz", "negate");
}

#[test]
fn filter_allow_test() {
    let filter       = FilterAllow::new();
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_allow.xyz", "allow");
}

#[test]
fn filter_deny_test() {
    let filter       = FilterDeny::new();
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_deny.xyz", "deny");
}
