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

//! AABBTree3D, an axis aligned bounding box tree in 3D for fast collision detection

use crate::*;

use std::marker::PhantomData;

#[derive(Clone)]
/// AABBTree3D, an axis aligned bounding box tree in 3D for fast collision detection
pub enum AABBTree3D<HB>
where
    HB: HasBoundingBox3D + Clone,
{
    Empty,
    Leaf(AABBTree3DLeaf<HB>),
    Branch(AABBTree3DBranch<HB>),
}

// currently often calculates the bounding box, try cache it
impl<HB> AABBTree3D<HB>
where
    HB: HasBoundingBox3D + Clone,
{
    pub fn new(data: Vec<HB>, maxdepth: usize, allowed_bucket_size: usize) -> Self {
        Self::new_rec(data, maxdepth, allowed_bucket_size, 0)
    }

    pub fn any<'a>(&'a self, f: &dyn Fn(&HB) -> bool) -> bool {
        match self {
            Self::Empty => false,
            Self::Leaf(leaf) => leaf.any(f),
            Self::Branch(branch) => branch.any(f),
        }
    }

    pub fn for_each_intersection_candidate<'a>(&'a self, line: &Line3D, f: &mut dyn FnMut(&HB)) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.for_each_intersection_candidate(line, f),
            Self::Branch(branch) => branch.for_each_intersection_candidate(line, f),
        }
    }

    pub fn for_each_collision_candidate<'a>(&'a self, bb: &BoundingBox3D, f: &mut dyn FnMut(&HB)) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.for_each_collision_candidate(bb, f),
            Self::Branch(branch) => branch.for_each_collision_candidate(bb, f),
        }
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox3D, result: &mut Vec<&'a HB>) {
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

    pub fn bb_crossing_z_value<'a>(&'a self, z: f64, result: &mut Vec<&'a HB>) {
        match self {
            Self::Empty => (),
            Self::Leaf(leaf) => leaf.bb_crossing_z_value(z, result),
            Self::Branch(branch) => branch.bb_crossing_z_value(z, result),
        }
    }

    fn new_rec(data: Vec<HB>, maxdepth: usize, allowed_bucket_size: usize, depth: usize) -> Self {
        match data.len() {
            0 => AABBTree3D::Empty,
            1 => {
                let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                AABBTree3D::Leaf(AABBTree3DLeaf::new(data, bb))
            }
            _ => {
                if depth >= maxdepth || data.len() <= allowed_bucket_size {
                    let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                    AABBTree3D::Leaf(AABBTree3DLeaf::new(data, bb))
                } else {
                    let comp = match depth % 3 {
                        0 => Compare::X,
                        1 => Compare::Y,
                        _ => Compare::Z,
                    };
                    let bb = Self::bb_of(&data).unwrap(); //unwrap fine due to early return in new and data not empty
                    let center = bb.center_bb();

                    let dleft = data
                        .iter()
                        .cloned()
                        .filter(|x| Self::is_left_of(&comp, &x.bounding_box(), &center))
                        .collect::<Vec<_>>();
                    let dright = data
                        .iter()
                        .cloned()
                        .filter(|x| Self::is_right_of(&comp, &x.bounding_box(), &center))
                        .collect::<Vec<_>>();

                    if (dleft.len() == dright.len()) && dleft.len() == data.len() {
                        AABBTree3D::Leaf(AABBTree3DLeaf::new(data, bb))
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

                        AABBTree3D::Branch(AABBTree3DBranch::new(left, right, bb))
                    }
                }
            }
        }
    }

    fn is_left_of(comp: &Compare, bb: &BoundingBox3D, center: &Point3D) -> bool {
        match comp {
            Compare::X => bb.min_p().x() < center.x(),
            Compare::Y => bb.min_p().y() < center.y(),
            Compare::Z => bb.min_p().z() < center.z(),
        }
    }

    fn is_right_of(comp: &Compare, bb: &BoundingBox3D, center: &Point3D) -> bool {
        match comp {
            Compare::X => bb.max_p().x() >= center.x(),
            Compare::Y => bb.max_p().y() >= center.y(),
            Compare::Z => bb.max_p().z() >= center.z(),
        }
    }

    fn bb_of(data: &Vec<HB>) -> Result<BoundingBox3D> {
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

enum Compare {
    X,
    Y,
    Z,
}

#[derive(Clone)]
/// Leaf of the AABBTree3D
pub struct AABBTree3DLeaf<HB>
where
    HB: HasBoundingBox3D,
{
    data: Vec<HB>,
    bb: BoundingBox3D,
    _marker: PhantomData<HB>,
}

impl<HB> AABBTree3DLeaf<HB>
where
    HB: HasBoundingBox3D + Clone,
{
    pub fn new(data: Vec<HB>, bb: BoundingBox3D) -> Self {
        AABBTree3DLeaf {
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

    pub fn for_each_intersection_candidate<'a>(&'a self, line: &Line3D, f: &mut dyn FnMut(&HB)) {
        if intersection(line, &self.bb).is_none() {
            return;
        }
        for x in self.data.iter() {
            if intersection(line, &x.bounding_box()).is_some() {
                f(x)
            }
        }
    }

    pub fn for_each_collision_candidate<'a>(&'a self, bb: &BoundingBox3D, f: &mut dyn FnMut(&HB)) {
        if !self.bb.collides_with(bb) {
            return;
        }
        for x in self.data.iter() {
            if x.bounding_box().collides_with(bb) {
                f(x)
            }
        }
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox3D, result: &mut Vec<&'a HB>) {
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

    pub fn bb_crossing_z_value<'a>(&'a self, z: f64, result: &mut Vec<&'a HB>) {
        if self.bb.crossing_z_value(z) {
            for d in self.data.iter() {
                if d.bounding_box().crossing_z_value(z) {
                    result.push(d)
                }
            }
        }
    }
}

#[derive(Clone)]
/// Branch of the AABBTree3D
pub struct AABBTree3DBranch<HB>
where
    HB: HasBoundingBox3D + Clone,
{
    left: Box<AABBTree3D<HB>>,
    right: Box<AABBTree3D<HB>>,
    bb: BoundingBox3D,
    _marker: PhantomData<HB>,
}

impl<HB> AABBTree3DBranch<HB>
where
    HB: HasBoundingBox3D + Clone,
{
    pub fn new(left: Box<AABBTree3D<HB>>, right: Box<AABBTree3D<HB>>, bb: BoundingBox3D) -> Self {
        AABBTree3DBranch {
            left,
            right,
            bb,
            _marker: PhantomData,
        }
    }

    pub fn any<'a>(&'a self, f: &dyn Fn(&HB) -> bool) -> bool {
        self.left.any(f) || self.right.any(f)
    }

    pub fn for_each_intersection_candidate<'a>(&'a self, line: &Line3D, f: &mut dyn FnMut(&HB)) {
        if intersection(line, &self.bb).is_none() {
            return;
        }

        self.left.for_each_intersection_candidate(line, f);
        self.right.for_each_intersection_candidate(line, f);
    }

    pub fn for_each_collision_candidate<'a>(&'a self, bb: &BoundingBox3D, f: &mut dyn FnMut(&HB)) {
        if !self.bb.collides_with(bb) {
            return;
        }

        self.left.for_each_collision_candidate(bb, f);
        self.right.for_each_collision_candidate(bb, f);
    }

    pub fn bb_colliding<'a>(&'a self, bb: &BoundingBox3D, result: &mut Vec<&'a HB>) {
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

    pub fn bb_crossing_z_value<'a>(&'a self, z: f64, result: &mut Vec<&'a HB>) {
        if self.bb.crossing_z_value(z) {
            self.left.bb_crossing_z_value(z, result);
            self.right.bb_crossing_z_value(z, result);
        }
    }
}

//------------------------------------------------------------------------------

impl<HB> IsColliderContainer3D for AABBTree3D<HB>
where
    HB: Clone + HasColliders3D + Sized,
{
    fn any_element_collides_with_collider(&self, other: &dyn HasColliders3D) -> bool {
        let mut any_collides = false;
        self.for_each_collision_candidate(&other.bounding_box(), &mut |candidate| {
            if !any_collides {
                any_collides = candidate.collides_with(other);
            }
        });

        any_collides
    }

    fn any_element_collides_with_bounding(&self, other: &dyn HasBoundingBox3D) -> bool {
        let mut any_collides = false;
        self.for_each_collision_candidate(&other.bounding_box(), &mut |_candidate| {
            any_collides = true;
        });

        any_collides
    }
}
