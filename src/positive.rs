/*
Copyright 2016 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! Positive, a wrapper for a f64 value, ensuring it is always > 0

use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::hash::{Hash, Hasher};

use crate::prelude::*;
use crate::utils::hash_f64;

#[derive (Debug, PartialEq, PartialOrd, Clone, Copy)]
/// Positive, a wrapper for a f64 value, ensuring it is always > 0
pub struct Positive {
    val: f64
}

impl Positive {
    /// Creates a new Positive if input > 0, fails otherwise
    pub fn new(val: f64) -> Result<Positive> {
        if val > 0.0 {
            return Ok(Positive {val});
        }
        Err(ErrorKind::NumberInWrongRange)
    }
    /// Creates a new Positive with value 1
    pub fn one() -> Positive {
        Positive {val : 1.0}
    }
    /// Returns the wrapped value
    pub fn get(&self) -> f64 {
        self.val
    }
    /// Returns the square root
    pub fn sqrt(&self) -> Positive {
        Positive{val: self.val.sqrt()}
    }
}

impl Eq for Positive {}

impl Hash for Positive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64(self.val, state);
    }
}

impl Add for Positive {
    type Output = Positive;

    fn add(self, other: Positive) -> Positive {
        Positive {val: self.val + other.val}
    }
}

impl Add<NonNegative> for Positive {
    type Output = Positive;

    fn add(self, other: NonNegative) -> Positive {
        Positive { val: self.val + other.get() }
    }
}

impl AddAssign for Positive {
    fn add_assign(&mut self, other: Positive) {
        self.val += other.val;
    }
} 

impl AddAssign<NonNegative> for Positive {
    fn add_assign(&mut self, other: NonNegative) {
        self.val += other.get();
    }
}

impl Mul for Positive {
    type Output = Positive;
    
    fn mul(self, other: Positive) -> Positive {
        Positive {val: self.val * other.val }
    }
}

impl MulAssign for Positive {
    fn mul_assign(&mut self, other: Positive) {
        self.val *= other.get();
    }
}

impl Into<f64> for Positive {
    fn into(self) -> f64 {
        self.val
    }
}

impl AsRef<f64> for Positive {
    fn as_ref(&self) -> &f64 {
        &self.val
    }
}

impl Default for Positive {
    fn default() -> Self {
        Self::one()
    }
}

impl fmt::Display for Positive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
