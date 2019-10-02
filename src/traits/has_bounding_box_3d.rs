/*
Copyright 2016 Martin Buck

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

//! HasBoundingBox3D and HasBoundingBox3DMaybe traits for types which (might) have a bounding box

use crate::prelude::*;

/// HasBoundingBox3D is a trait for types which have a bounding box
pub trait HasBoundingBox3D : HasBoundingBox3DMaybe {
    /// Should return the bounding box
    fn bounding_box(&self) -> BoundingBox3D;
}

/// HasBoundingBox3DMaybe is a trait for types which might have a bounding box
pub trait HasBoundingBox3DMaybe {
    /// Should return the bounding box if it can be calculated
    fn bounding_box_maybe(&self) -> Result<BoundingBox3D>;
}

//------------------------------------------------------------------------------

//@todo consider moving somewhere else

/// Wrapper helper to convert the Maybe type to the non-Maybe type
pub struct HasBoundingBox3DConverted<T> where
    T: HasBoundingBox3DMaybe {
    
    data: T
}

impl<T> HasBoundingBox3DConverted<T> where
    T: HasBoundingBox3DMaybe {
        
    pub fn new(data: T) -> Result<Self> {
        data.bounding_box_maybe()?; // ensure present
        Ok(Self { data })
    }

    pub fn get(&self) -> &T {
        &self.data
    }
}

impl<T> HasBoundingBox3DMaybe for HasBoundingBox3DConverted<T> where
    T: HasBoundingBox3DMaybe {

    fn bounding_box_maybe(&self) -> Result<BoundingBox3D> {
        self.data.bounding_box_maybe()
    } 
}

impl<T> HasBoundingBox3D for HasBoundingBox3DConverted<T> where
    T: HasBoundingBox3DMaybe {

    fn bounding_box(&self) -> BoundingBox3D {
        self.data.bounding_box_maybe().unwrap() // safe since enforced on construction
    } 
}