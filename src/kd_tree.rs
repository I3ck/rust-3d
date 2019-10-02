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

//! KdTree https://en.wikipedia.org/wiki/K-d_tree

use std::cmp::Ordering;

use crate::distances_3d::*;
use crate::functions::{dimension_compare, dimension_dist};
use crate::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// KdTree https://en.wikipedia.org/wiki/K-d_tree
pub struct KdTree<P>
where
    P: Is3D,
{
    root: Option<KdNode<P>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct KdNode<P>
where
    P: Is3D,
{
    pub left: Option<Box<KdNode<P>>>,
    pub right: Option<Box<KdNode<P>>>,
    pub val: P,
    pub dimension: i8,
}

impl<P> IsTree3D<P> for KdTree<P>
where
    P: Is3D + Clone,
{
    fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) => node.size(),
        }
    }

    fn to_pointcloud(&self) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if let Some(ref node) = self.root {
            node.to_pointcloud_3d(&mut result);
        }
        result
    }

    fn build(&mut self, pc: PointCloud3D<P>) -> Result<()> {
        match pc.len() {
            0 => Err(ErrorKind::TooFewPoints),
            _ => {
                self.root = Some(KdNode::new(0, pc.data));
                Ok(())
            }
        }
    }
}

impl<PSearch, PFind> IsKNearestSearchable<PSearch, PFind> for KdTree<PFind>
where
    PSearch: Is3D,
    PFind: Is3D + Clone,
{
    fn knearest(&self, search: &PSearch, n: usize) -> Vec<PFind> {
        let mut result = Vec::new();
        if n < 1 {
            return result;
        }
        if let Some(ref node) = self.root {
            node.knearest(search, n, &mut result);
        }
        return result;
    }

    fn nearest(&self, search: &PSearch) -> Result<PFind> {
        //@todo implemented on its own, since the code can be faster without vecs
        let result = self.knearest(search, 1);
        match result.len() {
            0 => Err(ErrorKind::TooFewPoints),
            _ => {
                let p = result[0].clone();
                Ok(p)
            }
        }
    }
}

impl<P> IsSphereSearchable<P> for KdTree<P>
where
    P: Is3D + Clone,
{
    fn in_sphere(&self, sphere: &Sphere) -> Vec<P> {
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            node.in_sphere(sphere, &mut result);
        }
        return result;
    }
}

impl<P> IsBox3DSearchable<P> for KdTree<P>
where
    P: Is3D + Clone,
{
    fn in_box(&self, box_3d: &Box3D) -> Vec<P> {
        let mut result = Vec::new();
        if let Some(ref node) = self.root {
            node.in_box(box_3d, &mut result);
        }
        return result;
    }
}

impl<P> KdNode<P>
where
    P: Is3D,
{
    pub fn size(&self) -> usize {
        let mut result: usize = 0;
        if let Some(ref n) = (&self).left {
            result += n.size();
        }
        result += 1;
        if let Some(ref n) = (&self).right {
            result += n.size();
        }
        result
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl<P> KdNode<P>
where
    P: Is3D + Clone,
{
    pub fn to_pointcloud_3d(&self, pc: &mut PointCloud3D<P>) {
        if let Some(ref n) = (&self).left {
            n.to_pointcloud_3d(pc);
        }
        pc.push(self.val.clone());
        if let Some(ref n) = (&self).right {
            n.to_pointcloud_3d(pc);
        }
    }

    pub fn new(dim: i8, mut pc: Vec<P>) -> KdNode<P> {
        let dimension = dim % 2;
        if pc.len() == 1 {
            return KdNode {
                left: None,
                right: None,
                val: pc[0].clone(),
                dimension: dimension,
            };
        }

        pc.sort_by(|a, b| match dimension {
            0 => a.x().partial_cmp(&b.x()).unwrap_or(Ordering::Equal),
            1 => a.y().partial_cmp(&b.y()).unwrap_or(Ordering::Equal),
            2 => a.z().partial_cmp(&b.z()).unwrap_or(Ordering::Equal),
            _ => Ordering::Equal,
        });
        let median = pc.len() / 2;
        let mut pc_left = Vec::new();
        let mut pc_right = Vec::new();

        let val = pc[median].clone();

        for (i, p) in pc.into_iter().enumerate() {
            if i < median {
                pc_left.push(p);
            } else if i > median {
                pc_right.push(p);
            }
        }

        let left = match pc_left.len() {
            0 => None,
            _ => Some(Box::new(KdNode::new(dimension + 1, pc_left))),
        };

        let right = match pc_right.len() {
            0 => None,
            _ => Some(Box::new(KdNode::new(dimension + 1, pc_right))),
        };

        KdNode {
            left,
            right,
            val,
            dimension,
        }
    }
}

impl<P> KdNode<P>
where
    P: Is3D + Clone,
{
    pub fn knearest<PSearch>(&self, search: &PSearch, n: usize, pc: &mut Vec<P>)
    where
        PSearch: Is3D,
    {
        if pc.len() < n || sqr_dist_3d(search, &self.val) < sqr_dist_3d(search, &pc[&pc.len() - 1])
        {
            pc.push(self.val.clone());
        }

        let comp = dimension_compare(search, &self.val, self.dimension);

        match comp {
            Ok(res) => match res {
                Ordering::Less => {
                    if let Some(ref node) = (&self).left {
                        node.knearest(search, n, pc);
                    }
                }
                _ => {
                    if let Some(ref node) = (&self).right {
                        node.knearest(search, n, pc);
                    }
                }
            },
            Err(_) => {}
        }

        Self::sort_and_limit(pc, search, n);

        let (current_search, current_val) = match self.dimension {
            0 => (search.x(), self.val.x()),
            1 => (search.y(), self.val.y()),
            _ => (search.z(), self.val.z()),
        };

        let distance_best = dist_3d(search, &pc[&pc.len() - 1]);
        let border_left = current_search - distance_best;
        let border_right = current_search + distance_best;

        match comp {
            Ok(res) => match res {
                Ordering::Less => {
                    if let Some(ref node) = (&self).right {
                        if pc.len() < n || border_right >= current_val {
                            node.knearest(search, n, pc);
                        }
                    }
                }
                _ => {
                    if let Some(ref node) = (&self).left {
                        if pc.len() < n || border_left <= current_val {
                            node.knearest(search, n, pc);
                        }
                    }
                }
            },
            Err(_) => {}
        }

        Self::sort_and_limit(pc, search, n);
    }

    pub fn in_sphere(&self, sphere: &Sphere, pc: &mut Vec<P>) {
        if dist_3d(&sphere.center, &self.val) <= sphere.radius.get() {
            pc.push(self.val.clone());
        }

        if self.is_leaf() {
            return;
        }

        let comp = dimension_compare(&sphere.center, &self.val, self.dimension);

        match comp {
            Ok(res) => match res {
                Ordering::Less => {
                    if let Some(ref node) = (&self).left {
                        node.in_sphere(sphere, pc);
                    }
                }
                _ => {
                    if let Some(ref node) = (&self).right {
                        node.in_sphere(sphere, pc);
                    }
                }
            },
            Err(_) => {}
        }

        let (current_search, current_val) = match self.dimension {
            0 => (sphere.x(), self.val.x()),
            1 => (sphere.y(), self.val.y()),
            _ => (sphere.z(), self.val.z()),
        };

        let border_left = current_search - sphere.radius.get();
        let border_right = current_search + sphere.radius.get();

        match comp {
            Ok(res) => match res {
                Ordering::Less => {
                    if let Some(ref node) = (&self).right {
                        if border_right >= current_val {
                            node.in_sphere(sphere, pc);
                        }
                    }
                }
                _ => {
                    if let Some(ref node) = (&self).left {
                        if border_left <= current_val {
                            node.in_sphere(sphere, pc);
                        }
                    }
                }
            },
            Err(_) => {}
        }
    }

    pub fn in_box(&self, box_3d: &Box3D, pc: &mut Vec<P>) {
        if let (Ok(dist_x), Ok(dist_y), Ok(dist_z)) = (
            dimension_dist(&box_3d.center, &self.val, 0),
            dimension_dist(&box_3d.center, &self.val, 1),
            dimension_dist(&box_3d.center, &self.val, 2),
        ) {
            if dist_x <= 0.5 * box_3d.size_x.get()
                && dist_y <= 0.5 * box_3d.size_y.get()
                && dist_z <= 0.5 * box_3d.size_z.get()
            {
                pc.push(self.val.clone());
            }

            if self.is_leaf() {
                return;
            }

            let comp = dimension_compare(&box_3d.center, &self.val, self.dimension);

            match comp {
                Ok(res) => match res {
                    Ordering::Less => {
                        if let Some(ref node) = (&self).left {
                            node.in_box(box_3d, pc);
                        }
                    }
                    _ => {
                        if let Some(ref node) = (&self).right {
                            node.in_box(box_3d, pc);
                        }
                    }
                },
                Err(_) => {}
            }

            let (current_search, current_val, ref current_size) = match self.dimension {
                0 => (box_3d.x(), self.val.x(), &box_3d.size_x),
                1 => (box_3d.y(), self.val.y(), &box_3d.size_y),
                _ => (box_3d.z(), self.val.z(), &box_3d.size_z),
            };

            let border_left = current_search - 0.5 * current_size.get();
            let border_right = current_search + 0.5 * current_size.get();

            match comp {
                Ok(res) => match res {
                    Ordering::Less => {
                        if let Some(ref node) = (&self).right {
                            if border_right >= current_val {
                                node.in_box(box_3d, pc);
                            }
                        }
                    }
                    _ => {
                        if let Some(ref node) = (&self).left {
                            if border_left <= current_val {
                                node.in_box(box_3d, pc);
                            }
                        }
                    }
                },
                Err(_) => {}
            }
        }
    }

    fn sort_and_limit<'a, PSearch, PFind>(pc: &'a mut Vec<PFind>, search: &PSearch, max_size: usize)
    where
        PSearch: Is3D,
        PFind: Is3D + Clone,
    {
        if pc.len() > max_size {
            pc.sort_by(|a, b| {
                sqr_dist_3d(search, a)
                    .partial_cmp(&sqr_dist_3d(search, b))
                    .unwrap_or(Ordering::Equal)
            });
            let mut result: Vec<PFind>;
            result = Vec::new();
            for i in pc.iter().take(max_size) {
                result.push(i.clone());
            }
            *pc = result;
        }
    }
}
