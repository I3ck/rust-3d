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

//! Collider enumeration for 3D space

use crate::*;

//------------------------------------------------------------------------------

/// Collider enumeration for 3D space
pub enum Collider3D {
    AABB(BoundingBox3D),
    Box3(BoxUnaligned3D),
    //Sphere(Sphere), //@todo add
    Face(TriFace3D),
}

//------------------------------------------------------------------------------

impl Collider3D {
    pub fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AABB(a), Self::AABB(b)) => a.collides_with(b),

            (Self::AABB(a), Self::Box3(b)) => SATCollider::collide(a, b),
            (Self::Box3(_), Self::AABB(_)) => other.collides_with(self),

            (Self::AABB(a), Self::Face(b)) => SATCollider::collide(a, b),
            (Self::Face(_), Self::AABB(_)) => other.collides_with(self),

            (Self::Box3(a), Self::Box3(b)) => SATCollider::collide(a, b),

            (Self::Box3(a), Self::Face(b)) => SATCollider::collide(a, b),
            (Self::Face(_), Self::Box3(_)) => other.collides_with(self),

            (Self::Face(a), Self::Face(b)) => SATCollider::collide(a, b),
        }
    }
}

//------------------------------------------------------------------------------

impl HasBoundingBox3D for Collider3D {
    fn bounding_box(&self) -> BoundingBox3D {
        match self {
            Self::AABB(x) => x.clone(),
            Self::Box3(x) => x.bounding_box(),
            Self::Face(x) => x.bounding_box(),
        }
    }
}

impl HasBoundingBox3DMaybe for Collider3D {
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        Ok(self.bounding_box())
    }
}

//------------------------------------------------------------------------------

impl HasColliders3D for Collider3D {
    fn has_additional_colliders(&self) -> bool {
        match self {
            Self::AABB(_) => false,
            Self::Box3(_) => true,
            Self::Face(_) => true,
        }
    }

    fn with_colliders(&self, f: &mut dyn FnMut(&Collider3D)) {
        f(self)
    }
}
