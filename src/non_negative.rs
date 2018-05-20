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

//! NonNegative, a wrapper for a f64 value, ensuring it is always >= 0

use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign};
use std::hash::{Hash, Hasher};

use prelude::*;

#[derive (Debug, PartialEq, PartialOrd, Clone, Copy)]
/// NonNegative, a wrapper for a f64 value, ensuring it is always >= 0
pub struct NonNegative {
    val: f64
}

impl NonNegative {
    /// Creates a new NonNegative if input >= 0, fails otherwise
    pub fn new(val: f64) -> Result<NonNegative> {
        if val >= 0.0 {
            return Ok(NonNegative {val});
        }
        Err(ErrorKind::NumberInWrongRange)
    }
    /// Creates a new NonNegative with value 0
    pub fn zero() -> NonNegative {
        NonNegative {val: 0.0}
    }
    /// Creates a new NonNegative with value 1
    pub fn one() -> NonNegative {
        NonNegative {val : 1.0}
    }
    /// Returns the wrapped value
    pub fn get(&self) -> f64 {
        self.val
    }
    /// Returns the square root
    pub fn sqrt(&self) -> NonNegative {
        NonNegative{val: self.val.sqrt()}
    }
}

impl From<Positive> for NonNegative {
    fn from(x: Positive) -> Self {
        NonNegative {val: x.get() }
    }
}

impl Eq for NonNegative {}

impl Hash for NonNegative {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.val as u64).hash(state);
    }
}

impl Add for NonNegative {
    type Output = NonNegative;

    fn add(self, other: NonNegative) -> NonNegative {
        NonNegative {val: self.val + other.val}
    }
}

impl Add<Positive> for NonNegative {
    type Output = NonNegative;

    fn add(self, other: Positive) -> NonNegative {
        NonNegative {val: self.val + other.get()}
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
        NonNegative {val: self.val * other.val}
    }
}

impl Mul<Positive> for NonNegative {
    type Output = NonNegative;

    fn mul(self, other: Positive) -> NonNegative {
        NonNegative {val: self.val * other.get()}
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
        NonNegative {val: self.val / other.val}
    }
}

impl Div<Positive> for NonNegative {
    type Output = NonNegative;

    fn div(self, other: Positive) -> NonNegative {
        NonNegative {val: self.val / other.get()}
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
