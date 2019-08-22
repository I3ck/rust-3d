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

//! OcTree https://en.wikipedia.org/wiki/Octree

use std::collections::HashSet;
use std::iter::IntoIterator;

use prelude::*;
use functions::{center_3d};

#[derive (Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// OcTree https://en.wikipedia.org/wiki/Octree
pub struct OcTree<P> where
    P: Is3D {

    root: Option<OcNode<P>>,
    bb: BoundingBox3D
}

impl<P> IsTree3D<P> for OcTree<P> where
    P: IsBuildable3D + Clone + Default {

    fn size(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref node) => node.size()
        }
    }

    fn to_pointcloud(&self) -> PointCloud3D<P> {
        self.collect(-1)
    }

    fn build(&mut self, pc: PointCloud3D<P>) -> Result<()> {
        self.bb = pc.bounding_box()?;
        let mut unique_data = Vec::new();
        let mut set = HashSet::new();
        for p in pc.data {
            set.insert(p);
        }

        unique_data.extend(set.into_iter());
        self.root = Some(OcNode::new(&self.bb, unique_data)?);

        Ok(())
    }
}

impl<P> IsOcTree<P> for OcTree<P> where
    P: IsBuildable3D + Clone + Default {

    //@todo rewrite or make new method which returns cog instead of stopping recursion
    fn collect(&self,  maxdepth: i8) -> PointCloud3D<P> {
        let mut result = PointCloud3D::new();
        if let Some(ref node) = self.root {
            node.collect(0, maxdepth, &mut result);
        }
        result
    }
}

#[derive (Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
/// OcNode, which is a single node used within OcTree
enum OcNode<P> where
    P: Is3D {

    Leaf(P),
    Node(Internal<P>)
}

#[derive (Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
struct Internal<P> where
    P: Is3D { // naming : p == positive, n == negative ||| xyz   => pnp => x positive, y negative, z positive direction from center

    ppp: Option<Box<OcNode<P>>>,
    ppn: Option<Box<OcNode<P>>>,
    pnp: Option<Box<OcNode<P>>>,
    pnn: Option<Box<OcNode<P>>>,
    npp: Option<Box<OcNode<P>>>,
    npn: Option<Box<OcNode<P>>>,
    nnp: Option<Box<OcNode<P>>>,
    nnn: Option<Box<OcNode<P>>>
}

enum Direction {
    PPP,
    PPN,
    PNP,
    PNN,
    NPP,
    NPN,
    NNP,
    NNN
}

/*
/// Calculates the direction of one point to another in terms of an enum
pub fn calc_direction<P>(reference: &Point3D, p: &Point3D) -> Direction where
    P: Is3D {

    if p.x() >= reference.x() && p.y() >= reference.y() && p.z() >= reference.z() {
        Direction::PPP
    } else if p.x() >= reference.x() && p.y() >= reference.y() && p.z() < reference.z() {
        Direction::PPN
    } else if p.x() >= reference.x() && p.y() < reference.y() && p.z() >= reference.z() {
        Direction::PNP
    } else if p.x() >= reference.x() && p.y() < reference.y() && p.z() < reference.z() {
        Direction::PNN
    } else if p.x() < reference.x() && p.y() >= reference.y() && p.z() >= reference.z() {
        Direction::NPP
    } else if p.x() < reference.x() && p.y() >= reference.y() && p.z() < reference.z() {
        Direction::NPN
    } else if p.x() >= reference.x() && p.y() < reference.y() && p.z() >= reference.z() {
        Direction::NNP
    } else { //if p.x() < reference.x() && p.y() < reference.y() && p.z() < reference.z() {
        Direction::NNN
    }
}
*/

impl<P> OcNode<P> where
    P: Is3D {
    /// Returns the size of the oc node
    pub fn size(&self) -> usize {
        match self {
            OcNode::Leaf(_) => 1,
            OcNode::Node(internal) => {
                let mut result: usize = 0;
                if let Some(ref n) = internal.ppp { result += n.size(); }
                if let Some(ref n) = internal.ppn { result += n.size(); }
                if let Some(ref n) = internal.pnp { result += n.size(); }
                if let Some(ref n) = internal.pnn { result += n.size(); }
                if let Some(ref n) = internal.npp { result += n.size(); }
                if let Some(ref n) = internal.npn { result += n.size(); }
                if let Some(ref n) = internal.nnp { result += n.size(); }
                if let Some(ref n) = internal.nnn { result += n.size(); }
                result
            }
        }
    }
}

impl<P> OcNode<P> where
    P: IsBuildable3D + Clone {
    /// Creates a new OcNode from a min and max position and the data it should hold
    pub fn new(bb: &BoundingBox3D, pc: Vec<P>) -> Result<OcNode<P>> {
        if pc.len() == 1 { return Ok(OcNode::Leaf(pc[0].clone())); };
        let mut pcppp = Vec::new();
        let mut pcppn = Vec::new();
        let mut pcpnp = Vec::new();
        let mut pcpnn = Vec::new();
        let mut pcnpp = Vec::new();
        let mut pcnpn = Vec::new();
        let mut pcnnp = Vec::new();
        let mut pcnnn = Vec::new();

        let bbppp = Self::calc_sub_min_max(Direction::PPP, bb)?;
        let bbppn = Self::calc_sub_min_max(Direction::PPN, bb)?;
        let bbpnp = Self::calc_sub_min_max(Direction::PNP, bb)?;
        let bbpnn = Self::calc_sub_min_max(Direction::PNN, bb)?;
        let bbnpp = Self::calc_sub_min_max(Direction::NPP, bb)?;
        let bbnpn = Self::calc_sub_min_max(Direction::NPN, bb)?;
        let bbnnp = Self::calc_sub_min_max(Direction::NNP, bb)?;
        let bbnnn = Self::calc_sub_min_max(Direction::NNN, bb)?;

        for p in pc {
            if        bbppp.contains(&p) {
                pcppp.push(p);
            } else if bbppn.contains(&p) {
                pcppn.push(p);
            } else if bbpnp.contains(&p) {
                pcpnp.push(p);
            } else if bbpnn.contains(&p) {
                pcpnn.push(p);
            } else if bbnpp.contains(&p) {
                pcnpp.push(p);
            } else if bbnpn.contains(&p) {
                pcnpn.push(p);
            } else if bbnnp.contains(&p) {
                pcnnp.push(p);
            } else if bbnnn.contains(&p) {
                pcnnn.push(p);
            }
        }

        let ppp = Self::build_subnode(pcppp, &bbppp);
        let ppn = Self::build_subnode(pcppn, &bbppn);
        let pnp = Self::build_subnode(pcpnp, &bbpnp);
        let pnn = Self::build_subnode(pcpnn, &bbpnn);
        let npp = Self::build_subnode(pcnpp, &bbnpp);
        let npn = Self::build_subnode(pcnpn, &bbnpn);
        let nnp = Self::build_subnode(pcnnp, &bbnnp);
        let nnn = Self::build_subnode(pcnnn, &bbnnn);

        let result: Internal<P> = Internal {
            ppp: ppp,
            ppn: ppn,
            pnp: pnp,
            pnn: pnn,
            npp: npp,
            npn: npn,
            nnp: nnp,
            nnn: nnn
        };

        Ok(OcNode::Node(result))
    }
    /// Calculates the min and max values of sub nodes of an OcTree
    fn calc_sub_min_max(dir: Direction, bb: &BoundingBox3D) -> Result<BoundingBox3D> {

        let (min, max) = (bb.min_p(), bb.max_p());
        let middle = center_3d(&min, &max);

        let px = max.x();
        let py = max.y();
        let pz = max.z();
        let nx = min.x();
        let ny = min.y();
        let nz = min.z();
        let mx = middle.x();
        let my = middle.y();
        let mz = middle.z();

        //@todo get rid of unwrapping
        match dir {
            Direction::PPP => BoundingBox3D::new(&middle,                   &max),
            Direction::PPN => BoundingBox3D::new(&Point3D::new(mx, my, nz), &Point3D::new(px, py, mz)),
            Direction::PNP => BoundingBox3D::new(&Point3D::new(mx, ny, mz), &Point3D::new(px, my, pz)),
            Direction::PNN => BoundingBox3D::new(&Point3D::new(mx, ny, nz), &Point3D::new(px, my, mz)),
            Direction::NPP => BoundingBox3D::new(&Point3D::new(nx, my, mz), &Point3D::new(mx, py, pz)),
            Direction::NPN => BoundingBox3D::new(&Point3D::new(nx, my, nz), &Point3D::new(mx, py, mz)),
            Direction::NNP => BoundingBox3D::new(&Point3D::new(nx, ny, mz), &Point3D::new(mx, my, pz)),
            Direction::NNN => BoundingBox3D::new(&min.clone(),              &middle)
        }
    }
    /// Creates a child node
    fn build_subnode(pc: Vec<P>,bb: &BoundingBox3D) -> Option<Box<OcNode<P>>> {
        match pc.len() {
            0 => None,
            _ => match OcNode::new(bb, pc) {
                Err(_) => None,
                Ok(x)  => Some(Box::new(x))
            }
        }
    }
}

impl<P> OcNode<P> where
    P: IsBuildable3D + Clone + Default {
    /// Collects all points within the node until a certain depth
    pub fn collect(&self, depth: i8, maxdepth: i8, pc: &mut PointCloud3D<P>) {
        let only_collect_centers = maxdepth >= 0 && depth > maxdepth;
        match self {
            &OcNode::Leaf(ref p) => pc.push(p.clone()),

            &OcNode::Node(ref internal) => {
                if let Some(ref n) = internal.ppp {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.ppn {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.pnp {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.pnn {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.npp {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.npn {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.nnp {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.nnn {
                    Self::collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
            }
        }
    }
    /// Depending on a flag either returns the child points or the center of gravity
    fn collect_center_or_all(n: &OcNode<P>, only_collect_centers: bool, depth: i8, maxdepth: i8, pc: &mut PointCloud3D<P>) {
        if only_collect_centers {
            let mut sub_pc = PointCloud3D::new();
            n.collect(depth+1, maxdepth, &mut sub_pc);
            if let Ok(c) = sub_pc.center_of_gravity() {
                let mut p = P::default();
                p.from(&c);
                pc.push(p);
            }
        } else {
            n.collect(depth+1, maxdepth, pc);
        }
    }
}
