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
use rust_3d::filters::transformers::filter_random_accessible::*;
use rust_3d::view::*;
use rust_3d::io::xyz::*;
use rust_3d::test_helper::assert_files_equal;

fn test_filter_3d<F, P>(f: F, path_expected: &str, unique_identifier: &str) where
    F: IsFilter<P>,
    P: IsBuildable3D + Clone {

    let path_tmp = ["tests/tmp/tmp", unique_identifier, ".xyz"].join("");
    let filter = FilterRandomAccessible::build(f);

    let mut view = View::Full;
    let mut pc = PointCloud3D::<P>::new();
    load_xyz(&mut pc, "tests/data/test_cube.xyz", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view);
    save_xyz(&pc, &path_tmp, " ", "\n");
    assert_files_equal(path_expected, &path_tmp);
}

fn write_expected<F, P>(f: F, path_expected: &str) where
    F: IsFilter<P>,
    P: IsBuildable3D + Clone {

    let filter = FilterRandomAccessible::build(f);

    let mut view = View::Full;
    let mut pc = PointCloud3D::<P>::new();
    load_xyz(&mut pc, "tests/data/test_cube.xyz", " ", "\n").unwrap();

    filter.filter(&pc, &mut view);

    pc.apply_view(&view);
    save_xyz(&pc, &path_expected, " ", "\n");
}

#[test]
fn filter_box_3d_test() {
    let center = *Point3D::build(10.0, 10.0, 10.0);
    let size_x = Positive::new(2.1).unwrap();
    let size_y = Positive::new(2.1).unwrap();
    let size_z = Positive::new(2.1).unwrap();
    test_filter_3d::<_, Point3D>(FilterBox3D::build(center, size_x, size_y, size_z), "tests/data/expected_filter_box_3d.xyz", "box3d");
}

#[test]
fn filter_sphere_test() {
    let center = *Point3D::build(10.0, 10.0, 10.0);
    let radius = Positive::new(4.0).unwrap();
    let filter = FilterSphere::build(center, radius);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_sphere.xyz", "sphere");
}

#[test]
fn filter_or_test() {
    let filterSphere = FilterSphere::build(*Point3D::build(14.0, 14.0, 14.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::build(*Point3D::build(4.0, 4.0, 4.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let filter = FilterOR::build(filterSphere, filterBox);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_or.xyz", "or");
}

#[test]
fn filter_and_test() {
    let filterSphere = FilterSphere::build(*Point3D::build(10.0, 10.0, 10.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::build(*Point3D::build(8.0, 8.0, 8.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let filter = FilterAND::build(filterSphere, filterBox);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_and.xyz", "and");
}

#[test]
fn filter_xor_test() {
    let filterSphere = FilterSphere::build(*Point3D::build(13.0, 13.0, 13.0), Positive::new(10.0).unwrap());
    let filterBox    = FilterBox3D::build(*Point3D::build(8.0, 8.0, 8.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(30.0).unwrap());
    let filter = FilterXOR::build(filterSphere, filterBox);
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_xor.xyz", "xor");
}

#[test]
fn filter_all_test() {
    let filterSphere = FilterSphere::build(*Point3D::build(10.0, 10.0, 10.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::build(*Point3D::build(8.0, 8.0, 8.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let mut filter = FilterAll::new();
    filter.filters.push(Box::new(filterSphere));
    filter.filters.push(Box::new(filterBox));
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_and.xyz", "and"); //same as the and test
}

#[test]
fn filter_any_test() {
    let filterSphere = FilterSphere::build(*Point3D::build(14.0, 14.0, 14.0), Positive::new(5.0).unwrap());
    let filterBox    = FilterBox3D::build(*Point3D::build(4.0, 4.0, 4.0), Positive::new(5.0).unwrap(), Positive::new(7.0).unwrap(), Positive::new(15.0).unwrap());
    let mut filter = FilterAny::new();
    filter.filters.push(Box::new(filterSphere));
    filter.filters.push(Box::new(filterBox));
    test_filter_3d::<_, Point3D>(filter, "tests/data/expected_filter_or.xyz", "or"); //same as the or test
}

//any outer_inner
