/*
Copyright 2017 Martin Buck

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

//! NonNegative, a wrapper for a f64 value, ensuring it is always >= 0

use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

use crate::*;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// NonNegative, a wrapper for a f64 value, ensuring it is always >= 0
pub struct NonNegative {
    val: f64,
}

impl NonNegative {
    /// Creates a new NonNegative if input >= 0, fails otherwise
    pub fn new(val: f64) -> Result<NonNegative> {
        if val >= 0.0 {
            return Ok(NonNegative { val });
        }
        Err(ErrorKind::NumberInWrongRange)
    }
    /// Creates a new NonNegative with value 0
    pub fn zero() -> NonNegative {
        NonNegative { val: 0.0 }
    }
    /// Creates a new NonNegative with value 1
    pub fn one() -> NonNegative {
        NonNegative { val: 1.0 }
    }
    /// Returns the wrapped value
    pub fn get(&self) -> f64 {
        self.val
    }
    /// Returns the square root
    pub fn sqrt(&self) -> NonNegative {
        NonNegative {
            val: self.val.sqrt(),
        }
    }
}

impl From<Positive> for NonNegative {
    fn from(x: Positive) -> Self {
        NonNegative { val: x.get() }
    }
}

impl Eq for NonNegative {}

impl Hash for NonNegative {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f64(self.val, state);
    }
}

impl Add for NonNegative {
    type Output = NonNegative;

    fn add(self, other: NonNegative) -> NonNegative {
        NonNegative {
            val: self.val + other.val,
        }
    }
}

impl Add<Positive> for NonNegative {
    type Output = NonNegative;

    fn add(self, other: Positive) -> NonNegative {
        NonNegative {
            val: self.val + other.get(),
        }
    }
}

impl AddAssign for NonNegative {
    fn add_assign(&mut self, other: NonNegative) {
        self.val += other.val;
    }
}

impl AddAssign<Positive> for NonNegative {
    fn add_assign(&mut self, other: Positive) {
        self.val += other.get();
    }
}

impl Mul for NonNegative {
    type Output = NonNegative;

    fn mul(self, other: NonNegative) -> NonNegative {
        NonNegative {
            val: self.val * other.val,
        }
    }
}

impl Mul<Positive> for NonNegative {
    type Output = NonNegative;

    fn mul(self, other: Positive) -> NonNegative {
        NonNegative {
            val: self.val * other.get(),
        }
    }
}

impl MulAssign for NonNegative {
    fn mul_assign(&mut self, other: NonNegative) {
        self.val *= other.val;
    }
}

impl MulAssign<Positive> for NonNegative {
    fn mul_assign(&mut self, other: Positive) {
        self.val *= other.get();
    }
}

impl Div for NonNegative {
    type Output = NonNegative;

    fn div(self, other: NonNegative) -> NonNegative {
        NonNegative {
            val: self.val / other.val,
        }
    }
}

impl Div<Positive> for NonNegative {
    type Output = NonNegative;

    fn div(self, other: Positive) -> NonNegative {
        NonNegative {
            val: self.val / other.get(),
        }
    }
}

impl DivAssign for NonNegative {
    fn div_assign(&mut self, other: NonNegative) {
        self.val /= other.val;
    }
}

impl DivAssign<Positive> for NonNegative {
    fn div_assign(&mut self, other: Positive) {
        self.val /= other.get();
    }
}

impl Into<f64> for NonNegative {
    fn into(self) -> f64 {
        self.val
    }
}

impl AsRef<f64> for NonNegative {
    fn as_ref(&self) -> &f64 {
        &self.val
    }
}

impl Default for NonNegative {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for NonNegative {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
