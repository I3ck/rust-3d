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

//! FilterOutlier3D, a filter which removes outliers by counting their neighbours in a search radius
//! This can be used to compare two sets of points by removing those in A which aren't close enough to B
//! Or to remove outliers within a single set
//! For this use the same input to build this filter as to filter against
//! Points will find themselves, so increase the required count by 1

//@todo make this work for IsRandomAccessible once KdTree supports it
//@todo or even prevent copy once KdTree is referencing
//@todo KdTree needs constructor (best would be to directly build from IsRandomAcc)
//@todo write 2D version once KdTree also supports 2D
//@todo fix docs

use prelude::*;

use std::marker::PhantomData;


/// FilterOutlier3D, a filter which removes outliers by counting their neighbours in a search radius
/// This can be used to compare two sets of points by removing those in A which aren't close enough to B
/// Or to remove outliers within a single set
/// For this use the same input to build this filter as to filter against
/// Points will find themselves, so increase the required count by 1
#[derive (Debug, PartialEq, PartialOrd, Default, Clone)]
pub struct FilterOutlier3D<S, PSearch, PFind> where
    PSearch: Is3D,
    PFind: Is3D,
    S: IsSphereSearchable<PSearch, PFind> {

    search_distance: Positive,
    min_neighbours: usize, //@todo should be usize >= 1 add new type for that?
    searchable: S,
    phantom_search: PhantomData<PSearch>,
    phantom_find: PhantomData<PFind>
}

impl<S, PSearch, PFind> FilterOutlier3D<S, PSearch, PFind> where
    PSearch: Is3D,
    PFind: Is3D,
    S: IsSphereSearchable<PSearch, PFind> {
    /// Creates a new FilterOutlier3D from a search distance and the min number of neighbours to be found in this distance
    pub fn new(searchable: S, search_distance: Positive, min_neighbours: usize) -> Result<Self> {
        Ok(FilterOutlier3D { search_distance: search_distance, min_neighbours: min_neighbours, searchable: searchable, phantom_search: PhantomData, phantom_find: PhantomData})
    }
}

impl<S, PSearch, PFind> IsFilter<PSearch> for FilterOutlier3D <S, PSearch, PFind>where
    PSearch: Is3D,
    PFind: Is3D,
    S: IsSphereSearchable<PSearch, PFind> {

    fn is_allowed(&self, p: &PSearch) -> bool {
        let pts = self.searchable.in_sphere(p, self.search_distance.get());
        pts.len() >= self.min_neighbours
    }
}

