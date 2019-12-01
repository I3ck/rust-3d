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

//! Trait for types that have 3D colliders

use crate::prelude::*;

//------------------------------------------------------------------------------

/// Trait for types that have 3D colliders
pub trait HasColliders3D: HasBoundingBox3D {
    /// Whether has colliders in addition to the bounding box, otherwise just use AABB
    fn has_additional_colliders(&self) -> bool;
    /// Apply function to each of the colliders
    fn with_colliders(&self, f: &mut dyn FnMut(&Collider3D));

    fn collides_with_collider(&self, other: &Collider3D) -> bool {
        if self.has_additional_colliders() {
            let mut result = false;
            self.with_colliders(&mut |collider| {
                if !result {
                    result = collider.collides_with(other);
                }
            });
            result
        } else {
            let self_collider = Collider3D::AABB(self.bounding_box());
            self_collider.collides_with(other)
        }
    }

    fn collides_with(&self, other: &dyn HasColliders3D) -> bool {
        if other.has_additional_colliders() {
            let mut result = false;
            other.with_colliders(&mut |collider| {
                if !result {
                    result = self.collides_with_collider(collider)
                }
            });
            result
        } else {
            let other_collider = Collider3D::AABB(other.bounding_box());
            self.collides_with_collider(&other_collider)
        }
    }
}
