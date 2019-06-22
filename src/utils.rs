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

use std::hash::{Hash, Hasher};

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

/// Generates the hash of an f64
pub fn hash_f64<H>(x: f64, state: &mut H) where
    H: Hasher {

    let (m, e, s) = integer_decode(x);
    m.hash(state);
    e.hash(state);
    s.hash(state);
}

/// Returns the mantissa, exponent and sign as integers.
/// taken from https://github.com/rust-lang/rust/blob/5c674a11471ec0569f616854d715941757a48a0a/src/libcore/num/f64.rs#L203-L216
fn integer_decode(x: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { std::mem::transmute(x) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    // Exponent bias + mantissa shift
    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}
