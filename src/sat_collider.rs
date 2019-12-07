/*
Copyright 2019 Martin Buck

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

//! Helper to check for collisions between IsSATObject

use crate::*;

//------------------------------------------------------------------------------

/// Helper to check for collisions between IsSATObject
pub struct SATCollider {}

//------------------------------------------------------------------------------

impl SATCollider {
    pub fn collide<A, B>(a: &A, b: &B) -> bool
    where
        A: IsSATObject,
        B: IsSATObject,
    {
        let mut all_overlap = true;

        let mut f = |axis: &Norm3D| {
            if all_overlap {
                let (mut min_a, mut max_a, mut min_b, mut max_b) = (None, None, None, None);
                a.for_each_point(&mut |p: &Point3D| {
                    let x = p.dot(axis);
                    match min_a {
                        None => min_a = Some(x),
                        Some(min) => {
                            if x < min {
                                min_a = Some(x)
                            }
                        }
                    }
                    match max_a {
                        None => max_a = Some(x),
                        Some(max) => {
                            if x > max {
                                max_a = Some(x)
                            }
                        }
                    }
                });
                b.for_each_point(&mut |p: &Point3D| {
                    let x = p.dot(axis);
                    match min_b {
                        None => min_b = Some(x),
                        Some(min) => {
                            if x < min {
                                min_b = Some(x)
                            }
                        }
                    }
                    match max_b {
                        None => max_b = Some(x),
                        Some(max) => {
                            if x > max {
                                max_b = Some(x)
                            }
                        }
                    }
                });

                if let (Some(mina), Some(maxa), Some(minb), Some(maxb)) =
                    (min_a, max_a, min_b, max_b)
                {
                    let not_overlap = maxa < minb || maxb < mina;
                    if not_overlap {
                        all_overlap = false
                    }
                }
            }
        };

        a.for_each_axis(&mut f);
        b.for_each_axis(&mut f);
        all_overlap
    }
}
