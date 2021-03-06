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

//! rust-3d trait implementations for the standard Vec

use crate::*;

//------------------------------------------------------------------------------

impl<T> IsRandomAccessible<T> for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> IsRandomInsertible<T> for Vec<T> {
    fn insert(&mut self, index: usize, x: T) -> Result<()> {
        if index >= self.len() {
            return Err(ErrorKind::IndexOutOfBounds);
        }
        self.insert(index, x);
        Ok(())
    }
}

impl<T> IsPushable<T> for Vec<T> {
    fn push(&mut self, x: T) {
        self.push(x)
    }

    fn reserve(&mut self, n: usize) {
        self.reserve(n)
    }
}

impl<T> IsDataContainer<T> for Vec<T>
where
    T: Clone,
{
    fn reserve_d(&mut self, n: usize) {
        self.reserve(n)
    }
    fn len_d(&self) -> usize {
        self.len()
    }
    fn push_d(&mut self, x: T) {
        self.push(x)
    }
    fn get_d(&self, index: usize) -> Option<T> {
        self.get(index).cloned()
    }
    fn set_d(&mut self, index: usize, x: T) {
        self[index] = x
    }
}

impl<T> IsViewBuildable for Vec<T>
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
                let mut new_data = Vec::new();
                for (i, p) in self.iter().enumerate() {
                    if indices.contains(&i) {
                        new_data.push(p.clone());
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

impl<T> IsMovable2D for Vec<T>
where
    T: IsMovable2D,
{
    fn move_by(&mut self, x: f64, y: f64) {
        for ref mut p in self.iter_mut() {
            p.move_by(x, y);
        }
    }
}

impl<T> IsMovable3D for Vec<T>
where
    T: IsMovable3D,
{
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for ref mut p in self.iter_mut() {
            p.move_by(x, y, z);
        }
    }
}

impl<HB> IsColliderContainer3D for Vec<HB>
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

impl IsIndexContainer for Vec<usize> {
    fn ensure_supported(&mut self, _x: usize) {
        // always supported
    }

    fn reserve(&mut self, n: usize) {
        self.reserve(n)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> usize {
        self[index]
    }

    fn set(&mut self, index: usize, value: usize) {
        self[index] = value
    }

    fn push(&mut self, value: usize) {
        self.push(value)
    }

    fn iter(&self) -> IsIndexContainerIterator<Self> {
        IsIndexContainerIterator::new(self)
    }
}
