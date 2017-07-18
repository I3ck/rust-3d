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

//! Containing strong type definitions for safer usage

use std::fmt;

macro_rules! strong_usize {
    ($NEW_NAME:ident) => (
        #[derive (Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $NEW_NAME {
            pub val: usize
        }
        impl fmt::Display for $NEW_NAME {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.val)
            }
        }
    )
}

macro_rules! strong_f64 {
    ($NEW_NAME:ident) => (
        #[derive (Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $NEW_NAME {
            pub val: f64
        }
        impl fmt::Display for $NEW_NAME {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.val)
            }
        }
    )
}

strong_usize!(VId);
strong_usize!(FId);
strong_usize!(EId);

strong_f64!(Deg);
strong_f64!(Rad);
