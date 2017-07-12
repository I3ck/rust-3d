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

//! OcNode, which is a single node used within OcTree

//@todo clean up similar to pc code

use prelude::*;
use functions::{center};
//@todo either merge Oct code or split KdNode and Tree into seperate files
//@todo make all private or document

/// OcNode, which is a single node used within OcTree
pub enum OcNode<P> where
    P: IsEditable3D + IsBuildable3D {

    Leaf(P),
    Node(Internal<P>)
}

pub struct Internal<P> where
    P: IsEditable3D + IsBuildable3D { // naming : p == positive, n == negative ||| xyz   => pnp => x positive, y negative, z positive direction from center

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

//@todo refactor to work with IsBuildable3D?
/// Calculates the min and max values of sub nodes of an OcTree
fn calc_sub_min_max<P>(dir: Direction, min: &P, max: &P) -> (P, P) where //@todo move to OcTree
    P: IsBuildable3D + Clone { //@todo better name

    let middle = center(min, max);

    let px = max.x();
    let py = max.y();
    let pz = max.z();
    let nx = min.x();
    let ny = min.y();
    let nz = min.z();
    let mx = middle.x();
    let my = middle.y();
    let mz = middle.z();

    match dir {
        Direction::PPP => (*middle,               max.clone()),
        Direction::PPN => (*P::new(mx, my, nz),   *P::new(px, py, mz)),
        Direction::PNP => (*P::new(mx, ny, mz),   *P::new(px, my, pz)),
        Direction::PNN => (*P::new(mx, ny, nz),   *P::new(px, my, mz)),
        Direction::NPP => (*P::new(nx, my, mz),   *P::new(mx, py, pz)),
        Direction::NPN => (*P::new(nx, my, nz),   *P::new(mx, py, mz)),
        Direction::NNP => (*P::new(nx, ny, mz),   *P::new(mx, my, pz)),
        Direction::NNN => (min.clone(),           *middle)
    }
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

//@todo define somewhere else
fn collect_center_or_all<P>(n: &OcNode<P>, only_collect_centers: bool, depth: i8, maxdepth: i8, mut pc: &mut PointCloud3D<P>) where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone + Default {

    if only_collect_centers {
        let mut sub_pc = PointCloud3D::new();
        n.collect(depth+1, maxdepth, &mut sub_pc);
        if let Ok(c) = sub_pc.center_of_gravity() {
            let mut p = P::default();
            p.from(c);
            pc.push(p);
        }
    } else {
        n.collect(depth+1, maxdepth, pc);
    }
}

fn in_bb<P>(p: &P, min: &P, max: &P) -> bool where
    P: Is3D {

    match BoundingBox3D::new(min, max) {
        Err(_) => return false,
        Ok(x) => return x.contains(p)
    }
}

//@todo define somewhere else
fn build_subnode<P>(pc: Vec<P>,bb: (P, P)) -> Option<Box<OcNode<P>>> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone + Default {

    match pc.len() {
        0 => None,
        _ => {
            let (new_min, new_max) = bb;
            Some(Box::new(OcNode::new(&new_min, &new_max, pc)))
        }
    }
}


impl<P> OcNode<P> where
    P: IsEditable3D + IsBuildableND + IsBuildable3D + Clone + Default {

    pub fn new(min: &P, max: &P, pc: Vec<P>) -> OcNode<P> {
        if pc.len() == 1 { return OcNode::Leaf(pc[0].clone()); };
        let mut pcppp = Vec::new();
        let mut pcppn = Vec::new();
        let mut pcpnp = Vec::new();
        let mut pcpnn = Vec::new();
        let mut pcnpp = Vec::new();
        let mut pcnpn = Vec::new();
        let mut pcnnp = Vec::new();
        let mut pcnnn = Vec::new();

        let bbppp = calc_sub_min_max(Direction::PPP, min, max);
        let bbppn = calc_sub_min_max(Direction::PPN, min, max);
        let bbpnp = calc_sub_min_max(Direction::PNP, min, max);
        let bbpnn = calc_sub_min_max(Direction::PNN, min, max);
        let bbnpp = calc_sub_min_max(Direction::NPP, min, max);
        let bbnpn = calc_sub_min_max(Direction::NPN, min, max);
        let bbnnp = calc_sub_min_max(Direction::NNP, min, max);
        let bbnnn = calc_sub_min_max(Direction::NNN, min, max);

        for p in pc {
            if in_bb(&p, &bbppp.0, &bbppp.1) {
                pcppp.push(p);
            } else if in_bb(&p, &bbppn.0, &bbppn.1) {
                pcppn.push(p);
            } else if in_bb(&p, &bbpnp.0, &bbpnp.1) {
                pcpnp.push(p);
            } else if in_bb(&p, &bbpnn.0, &bbpnn.1) {
                pcpnn.push(p);
            } else if in_bb(&p, &bbnpp.0, &bbnpp.1) {
                pcnpp.push(p);
            } else if in_bb(&p, &bbnpn.0, &bbnpn.1) {
                pcnpn.push(p);
            } else if in_bb(&p, &bbnnp.0, &bbnnp.1) {
                pcnnp.push(p);
            } else if in_bb(&p, &bbnnn.0, &bbnnn.1) {
                pcnnn.push(p);
            }
        }

        let ppp = build_subnode(pcppp, bbppp);
        let ppn = build_subnode(pcppn, bbppn);
        let pnp = build_subnode(pcpnp, bbpnp);
        let pnn = build_subnode(pcpnn, bbpnn);
        let npp = build_subnode(pcnpp, bbnpp);
        let npn = build_subnode(pcnpn, bbnpn);
        let nnp = build_subnode(pcnnp, bbnnp);
        let nnn = build_subnode(pcnnn, bbnnn);

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

        OcNode::Node(result)
    }

    pub fn size(&self) -> usize {
        match self {
            &OcNode::Leaf(_) => 1,
            &OcNode::Node(ref internal) => {
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

//@todo define helpers here to simplify code (and in other areas)
    pub fn collect(&self, depth: i8, maxdepth: i8, pc: &mut PointCloud3D<P>) {
        let only_collect_centers = maxdepth >= 0 && depth > maxdepth; //@todo make this depend on a setting?
        match self {
            &OcNode::Leaf(ref p) => pc.push(p.clone()),

            &OcNode::Node(ref internal) => {
                if let Some(ref n) = internal.ppp {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.ppn {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.pnp {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.pnn {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.npp {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.npn {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.nnp {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
                if let Some(ref n) = internal.nnn {
                    collect_center_or_all(n, only_collect_centers, depth, maxdepth, pc);
                }
            }
        }
    }
}
