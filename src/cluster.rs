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

//! Spatial clustering of data on the x/y plane

use crate::*;

use std::collections::HashMap;

//------------------------------------------------------------------------------

/// Spatial clustering of data on the x/y plane
pub struct Cluster<HB>
where
    HB: HasBoundingBox3D,
{
    data: Vec<HB>,
    map: HashMap<(u16, u16), Vec<usize>>,
    start_x: f64,
    start_y: f64,
    incr_x: f64,
    incr_y: f64,
}

//------------------------------------------------------------------------------

impl<HB> Cluster<HB>
where
    HB: HasBoundingBox3D,
{
    pub fn new(data: Vec<HB>, count_x: u16, count_y: u16) -> Result<Self> {
        let mut map: HashMap<(u16, u16), Vec<usize>> = HashMap::new();
        let mut bb_all: Result<BoundingBox3D> = Err(ErrorKind::BoundingBoxMissing);
        for x in data.iter() {
            let bb = x.bounding_box();
            if let Ok(all) = bb_all {
                bb_all = Ok(all.combine(&&bb));
            } else {
                bb_all = Ok(bb);
            }
        }

        let bb = bb_all?;
        let (start_x, start_y) = (bb.min_p().x(), bb.min_p().y());
        let (size_x, size_y) = (*bb.size_x(), *bb.size_y());
        let (incr_x, incr_y) = (size_x / count_x as f64, size_y / count_y as f64);

        for (i, x) in data.iter().enumerate() {
            let xbb = x.bounding_box();
            let (min_x, min_y) = (xbb.min_p().x(), xbb.min_p().y());
            let (max_x, max_y) = (xbb.max_p().x(), xbb.max_p().y());

            let c_min_x = ((min_x - start_x) / incr_x) as u16;
            let c_min_y = ((min_y - start_y) / incr_y) as u16;

            let c_max_x = ((max_x - start_x) / incr_x) as u16;
            let c_max_y = ((max_y - start_y) / incr_y) as u16;

            for cx in c_min_x..=c_max_x {
                for cy in c_min_y..=c_max_y {
                    map.entry((cx, cy))
                        .and_modify(|e| e.push(i))
                        .or_insert(vec![i]);
                }
            }
        }
        Ok(Self {
            data,
            map,
            start_x,
            start_y,
            incr_x,
            incr_y,
        })
    }

    pub fn for_each_candidate<'a>(&'a self, x: f64, y: f64, f: &mut dyn FnMut(&HB)) {
        let c_x = ((x - self.start_x) / self.incr_x) as u16;
        let c_y = ((y - self.start_y) / self.incr_y) as u16;

        if let Some(is) = self.map.get(&(c_x, c_y)) {
            for i in is {
                f(&self.data[*i]);
            }
        }
    }
}
