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

//! FilterOutlier3D, a filter which removes outliers by counting their neighbours in a search radius
//! This can be used to compare two sets of points by removing those in A which aren't close enough to B
//! Or to remove outliers within a single set
//! For this use the same input to build this filter as to filter against
//! Points will find themselves, so increase the required count by 1

//@todo write 2D version once KdTree also supports 2D

use crate::*;

use std::sync::Mutex;

use std::marker::PhantomData;

/// FilterOutlier3D, a filter which removes outliers by counting their neighbours in a search radius
/// This can be used to compare two sets of points by removing those in A which aren't close enough to B
/// Or to remove outliers within a single set
/// For this use the same input to build this filter as to filter against
/// Points will find themselves, so increase the required count by 1
#[derive(Debug, Default)]
pub struct FilterOutlier3D<S, P>
where
    P: Is3D,
    S: IsSphereSearchable<P>,
{
    search_distance: Positive,
    min_neighbours: usize,
    searchable: S,
    cache: Mutex<Vec<P>>, 
    phantom_search: PhantomData<P>,
}

impl<S, P> FilterOutlier3D<S, P>
where
    P: Is3D,
    S: IsSphereSearchable<P>,
{
    /// Creates a new FilterOutlier3D from a search distance and the min number of neighbours to be found in this distance
    pub fn new(searchable: S, search_distance: Positive, min_neighbours: usize) -> Result<Self> {
        Ok(FilterOutlier3D {
            search_distance,
            min_neighbours,
            searchable,
            cache: Mutex::new(Vec::new()),
            phantom_search: PhantomData,
        })
    }
}

impl<S, P, PSearch> IsFilter<PSearch> for FilterOutlier3D<S, P>
where
    P: Is3D,
    PSearch: Is3D,
    S: IsSphereSearchable<P>,
{
    fn is_allowed(&self, p: &PSearch) -> bool {
        let mut pts = self.cache.lock().unwrap(); //@todo any way to properly handle failure here?
        pts.clear();
        self.searchable.in_sphere(
            &Sphere {
                center: Point3D {
                    x: p.x(),
                    y: p.y(),
                    z: p.z(),
                },
                radius: self.search_distance,
            },
            &mut pts,
        );
        pts.len() >= self.min_neighbours
    }
}

impl<S, P> IsScalable for FilterOutlier3D<S, P>
where
    P: Is3D,
    S: IsSphereSearchable<P>,
{
    fn scale(&mut self, factor: Positive) {
        self.search_distance *= factor;
    }
}
