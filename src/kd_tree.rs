/*
Copyright 2016 Martin Buck
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

use std::cmp::Ordering;

use point_cloud_3d::{PointCloud3D};
use functions::{dist_3d, sqr_dist_3d, dimension_compare, dimension_dist, sort_and_limit};

use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_3d::IsEditable3D;
use traits::is_tree_3d::IsTree3D;
use traits::is_kd_tree_3d::IsKdTree3D;

pub struct KdTree<P> where
    P: IsEditable3D {

    pub root: Option<KdNode<P>>
}
pub struct KdNode<P> where
    P: IsEditable3D {

    pub left: Option<Box<KdNode<P>>>,
    pub right: Option<Box<KdNode<P>>>,
    pub val: P,
    pub dimension: i8
}

impl<P> IsTree3D<P> for KdTree<P> where
    P: IsEditable3D {

    fn new() -> KdTree<P> {
        KdTree { root: None }
    }

    fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) => node.size()
        }
    }

    fn to_pointcloud(&self) -> PointCloud3D<P>{
        let mut result = PointCloud3D::new();
        if let Some(ref node) = self.root {
            node.to_pointcloud_3d(&mut result);
        }
        result
    }

    fn build(&mut self, pc: PointCloud3D<P>) -> bool {
        match pc.len() {
            0 => false,
            _ => {
                self.root = Some(KdNode::new(0, pc.data));
                true
            }
        }
    }

}

impl<P> IsKdTree3D<P> for KdTree<P> where
    P: IsEditable3D {

    fn knearest(&self, search: &P, n: usize) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if n < 1 { return result; }
        if let Some(ref node) = self.root {
            node.knearest(search, n, &mut result);
        }
        return result;
    }

    fn in_sphere(&self, search: &P, radius: f64) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if radius <= 0.0 { return result; }
        if let Some(ref node) = self.root {
            node.in_sphere(search, radius, &mut result);
        }
        return result;
    }

    fn in_box(&self, search: &P, x_size: f64, y_size: f64, z_size: f64) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if x_size <= 0.0 || y_size <= 0.0 || z_size <= 0.0 { return result; }
        if let Some(ref node) = self.root {
            node.in_box(search, x_size, y_size, z_size, &mut result);
        }
        return result;
    }

    fn nearest(&self, search: &P) -> Option<P> { //@todo implemented on its own, since the code can be faster without vecs
        let result = self.knearest(search, 1);
        match result.len() {
            0 => None,
            _ => {
                let p = result.data[0].clone();
                Some(p)
            }
        }
    }
}

impl<P> KdNode<P> where
    P: IsEditable3D {

    pub fn new(dim: i8, mut pc: Vec<Box<P>>) -> KdNode<P> {
        let dimension = dim % 2;
        if pc.len() == 1 {
            return KdNode {
                left: None,
                right: None,
                val: pc[0].clone(),
                dimension: dimension
            }
        }

        pc.sort_by(|a, b| match dimension {
            0 => a.x().partial_cmp(&b.x()).unwrap_or(Ordering::Equal),
            1 => a.y().partial_cmp(&b.y()).unwrap_or(Ordering::Equal),
            2 => a.z().partial_cmp(&b.z()).unwrap_or(Ordering::Equal),
            _ => Ordering::Equal
        });
        let median = pc.len() / 2;
        let mut pc_left = Vec::new();
        let mut pc_right = Vec::new();

        let mut val = P::new();

        for (i, p) in pc.into_iter().enumerate() {
            if      i < median  { pc_left.push(p); }
            else if i > median  { pc_right.push(p); }
            else                { val = p; }
        }

        let left = match pc_left.len() {
            0 => None,
            _ => Some(Box::new(KdNode::new(dimension+1, pc_left)))
        };

        let right = match pc_right.len() {
            0 => None,
            _ => Some(Box::new(KdNode::new(dimension+1, pc_right)))
        };

        KdNode {
            left: left,
            right: right,
            val: *val,
            dimension: dimension
        }
    }

    pub fn size(&self) -> usize {
        let mut result: usize = 0;
        if let Some(ref n) = (&self).left { result += n.size(); }
        result += 1;
        if let Some(ref n) = (&self).right { result += n.size(); }
        result
    }

    pub fn to_pointcloud_3d(&self, pc: &mut PointCloud3D<P>) {
        if let Some(ref n) = (&self).left { n.to_pointcloud_3d(pc); }
        pc.push(self.val.clone());
        if let Some(ref n) = (&self).right { n.to_pointcloud_3d(pc); }
    }

    pub fn knearest(&self, search: &P, n: usize, pc: &mut PointCloud3D<P>) {
        if pc.len() < n || sqr_dist_3d(search, &self.val) < sqr_dist_3d(search, &**&pc.data[&pc.len() -1 ]) { //@todo reference weird
            pc.push(self.val.clone());
        }

        let comp = dimension_compare(search, &self.val, self.dimension);

        match comp {
            Some(res) => match res {
                Ordering::Less  => if let Some(ref node) = (&self).left { node.knearest(search, n, pc); },
                _               => if let Some(ref node) = (&self).right { node.knearest(search, n, pc); }
            },
            None => {}
        }

        sort_and_limit(pc, search, n);

        let (current_search, current_val) = match self.dimension {
            0 => (search.x(), self.val.x()),
            1 => (search.y(), self.val.y()),
            _ => (search.z(), self.val.z())
        };

        let distance_best = dist_3d(search, &**&pc.data[&pc.len() -1 ]); //@todo reference weird
        let border_left = current_search - distance_best;
        let border_right = current_search + distance_best;

        match comp {
            Some(res) => match res {
                Ordering::Less => if let Some(ref node) = (&self).right {
                    if pc.len() < n || border_right >= current_val {
                        node.knearest(search, n, pc);
                    }
                },
                Ordering::Greater => if let Some(ref node) = (&self).left {
                    if pc.len() < n || border_left <= current_val {
                        node.knearest(search, n, pc);
                    }
                },
                Ordering::Equal => {}
            },
            None => {}
        }

        sort_and_limit(pc, search, n);
    }

    pub fn in_sphere(&self, search: &P, radius: f64, pc: &mut PointCloud3D<P>) {
        if radius <= 0.0 { return; }

        if dist_3d(search, &self.val) <= radius {
            pc.push(self.val.clone());
        }

        if self.is_leaf() { return; }

        let comp = dimension_compare(search, &self.val, self.dimension);

        match comp {
            Some(res) => match res {
                Ordering::Less  => if let Some(ref node) = (&self).left { node.in_sphere(search, radius, pc); },
                _               => if let Some(ref node) = (&self).right { node.in_sphere(search, radius, pc); }
            },
            None => {}
        }

        let (current_search, current_val) = match self.dimension {
            0 => (search.x(), self.val.x()),
            1 => (search.y(), self.val.y()),
            _ => (search.z(), self.val.z())
        };

        let border_left = current_search - radius;
        let border_right = current_search + radius;



        match comp {
            Some(res) => match res {
                Ordering::Less => if let Some(ref node) = (&self).right {
                    if border_right >= current_val {
                        node.in_sphere(search, radius, pc);
                    }
                },
                Ordering::Greater => if let Some(ref node) = (&self).left {
                    if border_left <= current_val {
                        node.in_sphere(search, radius, pc);
                    }
                },
                Ordering::Equal => {}
            },
            None => {}
        }
    }

    pub fn in_box(&self, search: &P, x_size: f64, y_size: f64, z_size: f64, pc: &mut PointCloud3D<P>) {
        if x_size <= 0.0 || y_size <= 0.0 || z_size <= 0.0 { return; }

        if let (Some(dist_x), Some(dist_y), Some(dist_z)) = (dimension_dist(search, &self.val, 0), dimension_dist(search, &self.val, 1), dimension_dist(search, &self.val, 2)) {
            if dist_x <= 0.5 * x_size && dist_y <= 0.5 * y_size && dist_z <= 0.5 * z_size {
                pc.push(self.val.clone());
            }

            if self.is_leaf()  { return; }

            let comp = dimension_compare(search, &self.val, self.dimension);

            match comp {
                Some(res) => match res {
                    Ordering::Less  => if let Some(ref node) = (&self).left { node.in_box(search, x_size, y_size, z_size, pc); },
                    _               => if let Some(ref node) = (&self).right { node.in_box(search, x_size, y_size, z_size, pc); }
                },
                None => {}
            }

            let (current_search, current_val, current_size) = match self.dimension {
                0 => (search.x(), self.val.x(), x_size),
                1 => (search.y(), self.val.y(), y_size),
                _ => (search.z(), self.val.z(), z_size)
            };

            let border_left = current_search - 0.5 * current_size;
            let border_right = current_search + 0.5 * current_size;

            match comp {
                Some(res) => match res {
                    Ordering::Less => if let Some(ref node) = (&self).right {
                        if border_right >= current_val {
                            node.in_box(search, x_size, y_size, z_size, pc);
                        }
                    },
                    Ordering::Greater => if let Some(ref node) = (&self).left {
                        if border_left <= current_val {
                            node.in_box(search, x_size, y_size, z_size, pc);
                        }
                    },
                    Ordering::Equal => {}
                },
                None => {}
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
