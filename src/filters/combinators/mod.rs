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

//! Containing filter combinators

mod filter_all_random_accessible;
pub use self::filter_all_random_accessible::FilterAllRandomAccessible;

mod filter_any_random_accessible;
pub use self::filter_any_random_accessible::FilterAnyRandomAccessible;

mod filter_all;
pub use self::filter_all::FilterAll;

mod filter_any;
pub use self::filter_any::FilterAny;

mod filter_negate;
pub use self::filter_negate::FilterNegate;

mod filter_and;
pub use self::filter_and::FilterAND;

mod filter_or;
pub use self::filter_or::FilterOR;

mod filter_xor;
pub use self::filter_xor::FilterXOR;

mod filter_outer_inner;
pub use self::filter_outer_inner::FilterOuterInner;

mod filter_allow;
pub use self::filter_allow::FilterAllow;

mod filter_deny;
pub use self::filter_deny::FilterDeny;
