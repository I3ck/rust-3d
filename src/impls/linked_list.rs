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

//! rust-3d trait implementations for the standard LinkedList

use std::collections::LinkedList;

use crate::*;

//------------------------------------------------------------------------------

impl<T> IsViewBuildable for LinkedList<T>
where
    T: Clone,
{
    fn apply_view(&mut self, view: &View) -> Result<()> {
        match view {
            View::Full => Ok(()),
            View::Restricted(indices) => {
                let n = self.len();
                if indices.iter().any(|x| x >= &n) {
                    return Err(ErrorKind::IndexOutOfBounds);
                }
                let mut new_data = LinkedList::new();
                for (i, p) in self.iter().enumerate() {
                    if indices.contains(&i) {
                        new_data.push_back(p.clone());
                    }
                }
                *self = new_data;
                Ok(())
            }
        }
    }

    fn from_view(&self, view: &View) -> Option<Self> {
        let mut cloned = self.clone();
        cloned.apply_view(view).ok()?;
        Some(cloned)
    }
}

impl<T> IsMovable2D for LinkedList<T>
where
    T: IsMovable2D,
{
    fn move_by(&mut self, x: f64, y: f64) {
        for ref mut p in self.iter_mut() {
            p.move_by(x, y);
        }
    }
}

impl<T> IsMovable3D for LinkedList<T>
where
    T: IsMovable3D,
{
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for ref mut p in self.iter_mut() {
            p.move_by(x, y, z);
        }
    }
}

impl<HB> IsColliderContainer3D for LinkedList<HB>
where
    HB: HasColliders3D,
{
    fn any_element_collides_with_collider(&self, other: &dyn HasColliders3D) -> bool {
        self.iter().any(|candidate| candidate.collides_with(other))
    }

    fn any_element_collides_with_bounding(&self, other: &dyn HasBoundingBox3D) -> bool {
        self.iter().any(|candidate| {
            other
                .bounding_box()
                .collides_with(&candidate.bounding_box())
        })
    }
}
