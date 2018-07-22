/*
Copyright 2018 Martin Buck
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

//! AABBTree2D, an axis aligned bounding box tree in 2D for fast collision detection

use prelude::*;

use std::marker::PhantomData;

/// AABBTree2D, an axis aligned bounding box tree in 2D for fast collision detection
pub enum AABBTree2D<HB> where
    HB: HasBoundingBox2D + Clone {

    Empty,
    Leaf(AABBTree2DLeaf<HB>),
    Branch(AABBTree2DBranch<HB>)
}

// currently often calculates the bounding box, try cache it
impl<HB> AABBTree2D<HB> where
    HB: HasBoundingBox2D + Clone {

    pub fn new(data: Vec<HB>, maxdepth: usize) -> Result<Self> {
        for x in data.iter() {
            x.bounding_box()?; //ensure bbs are known
        }
        Ok(Self::new_rec(data, maxdepth, 0))
    }

    pub fn bb_colliding(&self, bb: &BoundingBox2D) -> Vec<&HB> {
        match self {
            AABBTree2D::Empty          => Vec::new(),
            AABBTree2D::Leaf(leaf)     => leaf.bb_colliding(bb),
            AABBTree2D::Branch(branch) => branch.bb_colliding(bb)
        }
    }

    pub fn bb_crossing_x_value(&self, x: f64) -> Vec<&HB> {
        match self {
            AABBTree2D::Empty          => Vec::new(),
            AABBTree2D::Leaf(leaf)     => leaf.bb_crossing_x_value(x),
            AABBTree2D::Branch(branch) => branch.bb_crossing_x_value(x)
        }
    }

    pub fn bb_crossing_y_value(&self, y: f64) -> Vec<&HB> {
        match self {
            AABBTree2D::Empty          => Vec::new(),
            AABBTree2D::Leaf(leaf)     => leaf.bb_crossing_y_value(y),
            AABBTree2D::Branch(branch) => branch.bb_crossing_y_value(y)
        }
    }

    fn new_rec(data: Vec<HB>, maxdepth: usize, depth: usize) -> Self {
        match data.len() {
            0 => AABBTree2D::Empty,
            1 => {
                let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                AABBTree2D::Leaf(AABBTree2DLeaf::new(data, bb))
            },
            _ => {
                if depth >= maxdepth {
                    let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                    AABBTree2D::Leaf(AABBTree2DLeaf::new(data, bb))
                } else {
                    let compx  = depth % 2 != 0;
                    let mut bb = Self::bb_of(&data).unwrap(); //unwrap fine due to early return in new and data not empty
                    let center = bb.center_bb();

                    let dleft  = data.iter().cloned().filter(|x| Self::is_left_of(compx,  &x.bounding_box().unwrap(), &center)).collect::<Vec<_>>(); //unwrap fine due to early return in new
                    let dright = data.iter().cloned().filter(|x| Self::is_right_of(compx, &x.bounding_box().unwrap(), &center)).collect::<Vec<_>>(); //unwrap fine due to early return in new

                    if (dleft.len() == dright.len()) && dleft.len() == data.len() {
                        AABBTree2D::Leaf(AABBTree2DLeaf::new(data, bb))
                    } else {
                        let left  = Box::new(Self::new_rec(dleft, maxdepth, depth+1));
                        let right = Box::new(Self::new_rec(dright, maxdepth, depth+1));

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
            return Err(ErrorKind::IndexOutOfBounds) //@todo better type?
        }
        let mut result = data[0].bounding_box().unwrap(); //unwrap fine due to early return in new
        for x in data.iter() {
            result.consume(x.bounding_box().unwrap()); //unwrap fine due to early return in new
        }

        Ok(result)
    }
}

//todo describe
pub struct AABBTree2DLeaf<HB> where
    HB: HasBoundingBox2D {

    data: Vec<HB>,
    bb: BoundingBox2D,
    _marker: PhantomData<HB>
}

impl<HB> AABBTree2DLeaf<HB> where
    HB: HasBoundingBox2D + Clone {

    pub fn new(data: Vec<HB>, bb: BoundingBox2D) -> Self {
        AABBTree2DLeaf{data, bb, _marker: PhantomData}
    }

    pub fn bb_colliding(&self, bb: &BoundingBox2D) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.collides_with(bb) {
            return result;
        }
        for x in self.data.iter() {
            if x.bounding_box().unwrap().collides_with(bb) { //unwrap fine due to early return in new
                result.push(x)
            }
        }
        result
    }

    pub fn bb_crossing_x_value(&self, x: f64) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.crossing_x_value(x) {
            return result;
        }
        for d in self.data.iter() {
            if d.bounding_box().unwrap().crossing_x_value(x) { //unwrap fine due to early return in new
                result.push(d)
            }
        }
        result
    }

    pub fn bb_crossing_y_value(&self, y: f64) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.crossing_y_value(y) {
            return result;
        }
        for d in self.data.iter() {
            if d.bounding_box().unwrap().crossing_y_value(y) { //unwrap fine due to early return in new
                result.push(d)
            }
        }
        result
    }
}

//todo describe
pub struct AABBTree2DBranch<HB> where
    HB: HasBoundingBox2D + Clone {

    left: Box<AABBTree2D<HB>>,
    right: Box<AABBTree2D<HB>>,
    bb: BoundingBox2D,
    _marker: PhantomData<HB>
}

impl<HB> AABBTree2DBranch<HB> where
    HB: HasBoundingBox2D + Clone {

    pub fn new(left: Box<AABBTree2D<HB>>, right: Box<AABBTree2D<HB>>, bb: BoundingBox2D) -> Self {
        AABBTree2DBranch{left, right, bb, _marker: PhantomData}
    }

    pub fn bb_colliding(&self, bb: &BoundingBox2D) -> Vec<&HB> {
        if !self.bb.collides_with(bb) {
            return Vec::new();
        }

        let mut result = self.left.bb_colliding(bb);
        result.append(&mut self.right.bb_colliding(bb));
        result
    }

    pub fn bb_crossing_x_value(&self, x: f64) -> Vec<&HB> {
        if !self.bb.crossing_x_value(x) {
            return Vec::new();
        }

        let mut result = self.left.bb_crossing_x_value(x);
        result.append(&mut self.right.bb_crossing_x_value(x));
        result
    }

    pub fn bb_crossing_y_value(&self, y: f64) -> Vec<&HB> {
        if !self.bb.crossing_y_value(y) {
            return Vec::new();
        }

        let mut result = self.left.bb_crossing_y_value(y);
        result.append(&mut self.right.bb_crossing_y_value(y));
        result
    }
}
