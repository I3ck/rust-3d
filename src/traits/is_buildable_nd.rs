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

//! IsBuildableND trait used for types which are positioned in n-dimensional space and can be constructed

use prelude::*;

/// IsBuildableND is a trait used for types which are positioned in n-dimensional space and can be constructed
pub trait IsBuildableND : 
    Sized +
    IsND {
    
    /// Should build an object from the correct number of coordinates
    fn new_nd(coords: &Vec<f64>) -> Result<Self>;
    /// Should use the coordinates of another as its own
    fn from_nd<P>(&mut self, other: P) -> Result<()> where
        P: IsBuildableND;
    
    /// Returns the center between this and other
    fn center<P>(&self, other:& P) -> Result<Self> where
        P: IsND {
        
        let n = Self::n_dimensions();
        
        if n != P::n_dimensions() {
            return Err(ErrorKind::IncorrectDimension);
        }
        
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(0.5 * (self.get_position(i)? + other.get_position(i)?));
        }
        
        Self::new_nd(&v)
    }
}
