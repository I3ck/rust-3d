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

//! Module containing helper functions for testing

use std::io::prelude::*;
use std::fs::File;

//@todo maybe move directly to tests directory
/// Ensures the content of two files is equal
pub fn assert_files_equal(filepath1: &str, filepath2: &str) {
    let mut f1 = File::open(filepath1).unwrap();
    let mut f2 = File::open(filepath2).unwrap();

    let mut s1 = String::new();
    f1.read_to_string(&mut s1).unwrap();

    let mut s2 = String::new();
    f2.read_to_string(&mut s2).unwrap();

    assert!(s1 == s2);
}
