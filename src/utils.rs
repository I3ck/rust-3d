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

//! internal utility functions

/// Splits a line into a Vec of its words
pub fn to_words(line: &str) -> Vec<&str> {
    let split = line.trim().split(" ");
    split.collect::<Vec<&str>>()
}

/// Allows random adds anywhere on the Vec<Vec> by automatically resizing it with empty vectors
pub fn safe_append_at<T>(vec: &mut Vec<Vec<T>>, i: usize, val: T) where
    T: Clone {

    if i >= vec.len() {
        vec.resize(i+1, Vec::new());
    }

    vec[i].push(val);
}

/// Max of two f64 values
pub fn max_f64(a: f64, b: f64) -> f64 {
    if a > b { return a; }
    b
}

/// Max of three f64 values
pub fn max_f64_3(a: f64, b: f64, c: f64) -> f64 {
    max_f64(max_f64(a, b), c)
}

