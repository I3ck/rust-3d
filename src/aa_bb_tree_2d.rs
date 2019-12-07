/*
Copyright 2018 Martin Buck

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

//! AABBTree2D, an axis aligned bounding box tree in 2D for fast collision detection

use crate::prelude::*;

use std::marker::PhantomData;

#[derive(Clone)]
/// AABBTree2D, an axis aligned bounding box tree in 2D for fast collision detection
pub enum AABBTree2D<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    Empty,
    Leaf(AABBTree2DLeaf<HB>),
    Branch(AABBTree2DBranch<HB>),
}

// currently often calculates the bounding box, try cache it
impl<HB> AABBTree2D<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    pub fn new(data: Vec<HB>, maxdepth: usize, allowed_bucket_size: usize) -> Self {
        Self::new_rec(data, maxdepth, allowed_bucket_size, 0)
    }

    pub fn any<'a>(&'a self, f: &dyn Fn(&HB) -> bool) -> bool {
        match self {
            AABBTree2D::Empty => false,
            AABBTree2D::Leaf(leaf) => leaf.any(f),
            AABBTree2D::Branch(branch) => branch.any(f),
        }
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox2D, result: &mut Vec<&'a HB>) {
        match self {
            AABBTree2D::Empty => (),
            AABBTree2D::Leaf(leaf) => leaf.bb_colliding(bb, result),
            AABBTree2D::Branch(branch) => branch.bb_colliding(bb, result),
        }
    }

    pub fn bb_crossing_x_value<'a>(&'a self, x: f64, result: &mut Vec<&'a HB>) {
        match self {
            AABBTree2D::Empty => (),
            AABBTree2D::Leaf(leaf) => leaf.bb_crossing_x_value(x, result),
            AABBTree2D::Branch(branch) => branch.bb_crossing_x_value(x, result),
        }
    }

    pub fn bb_crossing_y_value<'a>(&'a self, y: f64, result: &mut Vec<&'a HB>) {
        match self {
            AABBTree2D::Empty => (),
            AABBTree2D::Leaf(leaf) => leaf.bb_crossing_y_value(y, result),
            AABBTree2D::Branch(branch) => branch.bb_crossing_y_value(y, result),
        }
    }

    fn new_rec(data: Vec<HB>, maxdepth: usize, allowed_bucket_size: usize, depth: usize) -> Self {
        match data.len() {
            0 => AABBTree2D::Empty,
            1 => {
                let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                AABBTree2D::Leaf(AABBTree2DLeaf::new(data, bb))
            }
            _ => {
                if depth >= maxdepth || data.len() <= allowed_bucket_size {
                    let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                    AABBTree2D::Leaf(AABBTree2DLeaf::new(data, bb))
                } else {
                    let compx = depth % 2 != 0;
                    let bb = Self::bb_of(&data).unwrap(); //unwrap fine due to early return in new and data not empty
                    let center = bb.center_bb();

                    let dleft = data
                        .iter()
                        .cloned()
                        .filter(|x| Self::is_left_of(compx, &x.bounding_box(), &center))
                        .collect::<Vec<_>>();
                    let dright = data
                        .iter()
                        .cloned()
                        .filter(|x| Self::is_right_of(compx, &x.bounding_box(), &center))
                        .collect::<Vec<_>>();

                    if (dleft.len() == dright.len()) && dleft.len() == data.len() {
                        AABBTree2D::Leaf(AABBTree2DLeaf::new(data, bb))
                    } else {
                        let left = Box::new(Self::new_rec(
                            dleft,
                            maxdepth,
                            allowed_bucket_size,
                            depth + 1,
                        ));
                        let right = Box::new(Self::new_rec(
                            dright,
                            maxdepth,
                            allowed_bucket_size,
                            depth + 1,
                        ));

                        AABBTree2D::Branch(AABBTree2DBranch::new(left, right, bb))
                    }
                }
            }
        }
    }

    fn is_left_of(compx: bool, bb: &BoundingBox2D, center: &Point2D) -> bool {
        if compx {
            bb.min_p().x() < center.x()
        } else {
            bb.min_p().y() < center.y()
        }
    }

    fn is_right_of(compx: bool, bb: &BoundingBox2D, center: &Point2D) -> bool {
        if compx {
            bb.max_p().x() >= center.x()
        } else {
            bb.max_p().y() >= center.y()
        }
    }

    fn bb_of(data: &Vec<HB>) -> Result<BoundingBox2D> {
        if data.len() == 0 {
            return Err(ErrorKind::TooFewPoints);
        }
        let mut result = data[0].bounding_box();
        for x in data.iter() {
            result.consume(x.bounding_box());
        }

        Ok(result)
    }
}

#[derive(Clone)]
/// Leaf of the AABBTree2D
pub struct AABBTree2DLeaf<HB>
where
    HB: HasBoundingBox2D,
{
    data: Vec<HB>,
    bb: BoundingBox2D,
    _marker: PhantomData<HB>,
}

impl<HB> AABBTree2DLeaf<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    pub fn new(data: Vec<HB>, bb: BoundingBox2D) -> Self {
        AABBTree2DLeaf {
            data,
            bb,
            _marker: PhantomData,
        }
    }

    pub fn any<'a>(&'a self, f: &dyn Fn(&HB) -> bool) -> bool {
        for x in self.data.iter() {
            if f(x) {
                //unwrap fine due to early return in new
                return true;
            }
        }
        false
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox2D, result: &mut Vec<&'a HB>) {
        if self.bb.collides_with(bb) {
            for x in self.data.iter() {
                if x.bounding_box().collides_with(bb) {
                    result.push(x)
                }
            }
        }
    }

    pub fn bb_crossing_x_value<'a>(&'a self, x: f64, result: &mut Vec<&'a HB>) {
        if self.bb.crossing_x_value(x) {
            for d in self.data.iter() {
                if d.bounding_box().crossing_x_value(x) {
                    result.push(d)
                }
            }
        }
    }

    pub fn bb_crossing_y_value<'a>(&'a self, y: f64, result: &mut Vec<&'a HB>) {
        if self.bb.crossing_y_value(y) {
            for d in self.data.iter() {
                if d.bounding_box().crossing_y_value(y) {
                    result.push(d)
                }
            }
        }
    }
}

#[derive(Clone)]
/// Branch of the AABBTree2D
pub struct AABBTree2DBranch<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    left: Box<AABBTree2D<HB>>,
    right: Box<AABBTree2D<HB>>,
    bb: BoundingBox2D,
    _marker: PhantomData<HB>,
}

impl<HB> AABBTree2DBranch<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    pub fn new(left: Box<AABBTree2D<HB>>, right: Box<AABBTree2D<HB>>, bb: BoundingBox2D) -> Self {
        AABBTree2DBranch {
            left,
            right,
            bb,
            _marker: PhantomData,
        }
    }

    pub fn any<'a>(&'a self, f: &dyn Fn(&HB) -> bool) -> bool {
        self.left.any(f) || self.right.any(f)
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox2D, result: &mut Vec<&'a HB>) {
        if self.bb.collides_with(bb) {
            self.left.bb_colliding(bb, result);
            self.right.bb_colliding(bb, result);
        }
    }

    pub fn bb_crossing_x_value<'a>(&'a self, x: f64, result: &mut Vec<&'a HB>) {
        if self.bb.crossing_x_value(x) {
            self.left.bb_crossing_x_value(x, result);
            self.right.bb_crossing_x_value(x, result);
        }
    }

    pub fn bb_crossing_y_value<'a>(&'a self, y: f64, result: &mut Vec<&'a HB>) {
        if self.bb.crossing_y_value(y) {
            self.left.bb_crossing_y_value(y, result);
            self.right.bb_crossing_y_value(y, result);
        }
    }
}
