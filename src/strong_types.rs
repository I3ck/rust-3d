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
strong_usize!(SId);

strong_f64!(Deg);
strong_f64!(Rad);
