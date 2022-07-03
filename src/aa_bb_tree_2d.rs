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

use crate::*;

use std::marker::PhantomData;

//------------------------------------------------------------------------------

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

//------------------------------------------------------------------------------

impl<HB> Default for AABBTree2D<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    fn default() -> Self {
        Self::Empty
    }
}

//------------------------------------------------------------------------------

//@todo currently often calculates the bounding box, try cache it
impl<HB> AABBTree2D<HB>
where
    HB: HasBoundingBox2D + Clone,
{
    pub fn new(data: Vec<HB>, maxdepth: usize, allowed_bucket_size: usize) -> Self {
        Self::new_rec(data, maxdepth, allowed_bucket_size, 0)
    }

    //@todo cache maxdepth / allowed_bucket_size as members
    pub fn add(&mut self, x: HB, maxdepth: usize, allowed_bucket_size: usize) {
        self.add_rec(x, maxdepth, allowed_bucket_size, 0)
    }

    //@todo cache maxdepth / allowed_bucket_size as members
    pub fn retain<F>(&mut self, maxdepth: usize, allowed_bucket_size: usize, f: &mut F)
    where
        F: FnMut(&HB) -> bool,
    {
        self.retain_rec(maxdepth, allowed_bucket_size, f, 0)
    }

    //mutates elements and MUST NOT affect the returned bounding box, otherwise order in tree will be incorrect
    pub fn mutate_elements<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut HB),
    {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.mutate_elements(f),
            Self::Branch(branch) => branch.mutate_elements(f),
        }
    }

    pub fn flatten_into(self, target: &mut Vec<HB>) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.flatten_into(target),
            Self::Branch(branch) => branch.flatten_into(target),
        }
    }

    pub fn any<'a, F>(&'a self, f: &F) -> bool
    where
        F: Fn(&HB) -> bool,
    {
        match self {
            Self::Empty => false,
            Self::Leaf(leaf) => leaf.any(f),
            Self::Branch(branch) => branch.any(f),
        }
    }

    pub fn for_each_collision_candidate<'a, F>(&'a self, bb: &BoundingBox2D, f: &mut F)
    where
        F: FnMut(&HB),
    {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.for_each_collision_candidate(bb, f),
            Self::Branch(branch) => branch.for_each_collision_candidate(bb, f),
        }
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox2D, result: &mut Vec<&'a HB>) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.bb_colliding(bb, result),
            Self::Branch(branch) => branch.bb_colliding(bb, result),
        }
    }

    pub fn bb_crossing_x_value<'a>(&'a self, x: f64, result: &mut Vec<&'a HB>) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.bb_crossing_x_value(x, result),
            Self::Branch(branch) => branch.bb_crossing_x_value(x, result),
        }
    }

    pub fn bb_crossing_y_value<'a>(&'a self, y: f64, result: &mut Vec<&'a HB>) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.bb_crossing_y_value(y, result),
            Self::Branch(branch) => branch.bb_crossing_y_value(y, result),
        }
    }

    pub fn stats(&self) -> AABBTree2DStats {
        self.stats_rec(0)
    }

    fn stats_rec(&self, depth_parent: u64) -> AABBTree2DStats {
        match self {
            Self::Empty => Default::default(), //@todo should here depth of one be counted instead?
            Self::Leaf(leaf) => leaf.stats(depth_parent),
            Self::Branch(branch) => branch.stats(depth_parent),
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
            bb.center_bb().x() < center.x()
        } else {
            bb.center_bb().y() < center.y()
        }
    }

    fn is_right_of(compx: bool, bb: &BoundingBox2D, center: &Point2D) -> bool {
        !Self::is_left_of(compx, bb, center)
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

    fn accepts(
        &self,
        bb: &BoundingBox2D,
        maxdepth: usize,
        allowed_bucket_size: usize,
        depth: usize,
    ) -> bool {
        match self {
            Self::Empty => true,
            Self::Leaf(leaf) => leaf.accepts(bb, maxdepth, allowed_bucket_size, depth),
            Self::Branch(branch) => branch.accepts(bb, maxdepth, allowed_bucket_size, depth),
        }
    }

    fn add_rec(&mut self, x: HB, maxdepth: usize, allowed_bucket_size: usize, depth: usize) {
        let bb = x.bounding_box();
        match self {
            Self::Empty => *self = Self::Leaf(AABBTree2DLeaf::new(vec![x], bb)),
            Self::Leaf(leaf) if leaf.accepts(&bb, maxdepth, allowed_bucket_size, depth) => {
                leaf.data.push(x)
            }
            Self::Branch(branch) if branch.accepts(&bb, maxdepth, allowed_bucket_size, depth) => {
                branch.add(x, maxdepth, allowed_bucket_size, depth)
            }
            _ => {
                let mut flat = Vec::new();
                std::mem::take(self).flatten_into(&mut flat);
                flat.push(x);
                *self = Self::new_rec(flat, maxdepth, allowed_bucket_size, depth);
            }
        }
    }

    fn retain_rec<F>(
        &mut self,
        maxdepth: usize,
        allowed_bucket_size: usize,
        f: &mut F,
        depth: usize,
    ) where
        F: FnMut(&HB) -> bool,
    {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => {
                if leaf.n_elements_after_retain(f) == 0 {
                    *self = Self::Empty
                } else {
                    leaf.retain(f)
                }
            }
            Self::Branch(branch) => {
                if branch.n_elements_after_retain(f) < allowed_bucket_size {
                    let mut flat = Vec::new();
                    std::mem::take(self).flatten_into(&mut flat);
                    flat.retain(f);
                    *self = Self::new_rec(flat, maxdepth, allowed_bucket_size, depth);
                } else {
                    branch.retain(maxdepth, allowed_bucket_size, f, depth);
                }
            }
        }
    }

    fn n_elements_after_retain<F>(&mut self, f: &mut F) -> usize
    where
        F: FnMut(&HB) -> bool,
    {
        match self {
            Self::Empty => 0,
            Self::Leaf(leaf) => leaf.n_elements_after_retain(f),
            Self::Branch(branch) => branch.n_elements_after_retain(f),
        }
    }
}

//------------------------------------------------------------------------------

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

    pub fn any<'a, F>(&'a self, f: &F) -> bool
    where
        F: Fn(&HB) -> bool,
    {
        for x in self.data.iter() {
            if f(x) {
                //unwrap fine due to early return in new
                return true;
            }
        }
        false
    }

    pub fn for_each_collision_candidate<'a, F>(&'a self, bb: &BoundingBox2D, f: &mut F)
    where
        F: FnMut(&HB),
    {
        if !self.bb.collides_with(bb) {
            return;
        }
        for x in self.data.iter() {
            if x.bounding_box().collides_with(bb) {
                f(x)
            }
        }
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

    pub fn stats(&self, depth_parent: u64) -> AABBTree2DStats {
        AABBTree2DStats {
            n_nodes: 1,
            n_elements: self.data.len(),
            max_depth: depth_parent + 1,
        }
    }

    pub fn flatten_into(mut self, target: &mut Vec<HB>) {
        target.append(&mut self.data);
    }

    fn accepts(
        &self,
        bb: &BoundingBox2D,
        _maxdepth: usize,
        allowed_bucket_size: usize,
        _depth: usize,
    ) -> bool {
        (*bb == self.bb || bb.is_inside(&self.bb)) && self.data.len() < allowed_bucket_size
    }

    fn n_elements_after_retain<F>(&self, f: &mut F) -> usize
    where
        F: FnMut(&HB) -> bool,
    {
        self.data.iter().filter(|x| f(x)).count()
    }

    fn retain<F>(&mut self, f: &mut F)
    where
        F: FnMut(&HB) -> bool,
    {
        self.data.retain(f);
        self.bb = AABBTree2D::bb_of(&self.data).unwrap(); // if would be empty, retain wouldn't be called
    }

    fn mutate_elements<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut HB),
    {
        for x in self.data.iter_mut() {
            f(x)
        }
    }
}

//------------------------------------------------------------------------------

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

    pub fn any<'a, F>(&'a self, f: &F) -> bool
    where
        F: Fn(&HB) -> bool,
    {
        self.left.any(f) || self.right.any(f)
    }

    pub fn for_each_collision_candidate<'a, F>(&'a self, bb: &BoundingBox2D, f: &mut F)
    where
        F: FnMut(&HB),
    {
        if !self.bb.collides_with(bb) {
            return;
        }

        self.left.for_each_collision_candidate(bb, f);
        self.right.for_each_collision_candidate(bb, f);
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

    pub fn stats(&self, depth_parent: u64) -> AABBTree2DStats {
        let sl = self.left.stats_rec(depth_parent + 1);
        let sr = self.right.stats_rec(depth_parent + 1);
        AABBTree2DStats {
            n_nodes: 1 + sl.n_nodes + sr.n_nodes,
            n_elements: sl.n_elements + sr.n_elements,
            max_depth: std::cmp::max(sl.max_depth, sr.max_depth),
        }
    }

    pub fn flatten_into(self, target: &mut Vec<HB>) {
        self.left.flatten_into(target);
        self.right.flatten_into(target);
    }

    fn accepts(
        &self,
        bb: &BoundingBox2D,
        maxdepth: usize,
        allowed_bucket_size: usize,
        depth: usize,
    ) -> bool {
        self.left
            .accepts(bb, maxdepth, allowed_bucket_size, depth + 1)
            || self
                .right
                .accepts(bb, maxdepth, allowed_bucket_size, depth + 1)
    }

    fn add(&mut self, x: HB, maxdepth: usize, allowed_bucket_size: usize, depth: usize) {
        let bb = x.bounding_box();
        if self
            .left
            .accepts(&bb, maxdepth, allowed_bucket_size, depth + 1)
        {
            self.left
                .add_rec(x, maxdepth, allowed_bucket_size, depth + 1)
        } else {
            self.right
                .add_rec(x, maxdepth, allowed_bucket_size, depth + 1)
        }
        self.bb.consume(bb);
    }

    fn n_elements_after_retain<F>(&mut self, f: &mut F) -> usize
    where
        F: FnMut(&HB) -> bool,
    {
        self.left.n_elements_after_retain(f) + self.right.n_elements_after_retain(f)
    }

    fn retain<F>(&mut self, maxdepth: usize, allowed_bucket_size: usize, f: &mut F, depth: usize)
    where
        F: FnMut(&HB) -> bool,
    {
        self.left
            .retain_rec(maxdepth, allowed_bucket_size, f, depth + 1);
        self.right
            .retain_rec(maxdepth, allowed_bucket_size, f, depth + 1);
    }

    fn mutate_elements<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut HB),
    {
        self.left.mutate_elements(f);
        self.right.mutate_elements(f);
    }
}

//------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct AABBTree2DStats {
    pub n_nodes: usize,
    pub n_elements: usize,
    pub max_depth: u64,
}
