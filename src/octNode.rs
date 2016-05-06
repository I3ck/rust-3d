use point::{Point};
use pointCloud::{PointCloud};
use functions::{center};
//@todo either merge Oct code or splot KdNode and Tree into seperate files

pub enum OctNode {
    Leaf(Point),
    Internal { // naming : p == positive, n == negative ||| xyz   => pnp => x positive, y negative, z positive direction from center
        ppp: Option<Box<OctNode>>,
        ppn: Option<Box<OctNode>>,
        pnp: Option<Box<OctNode>>,
        pnn: Option<Box<OctNode>>,
        npp: Option<Box<OctNode>>,
        npn: Option<Box<OctNode>>,
        nnp: Option<Box<OctNode>>,
        nnn: Option<Box<OctNode>>
    }
}

impl OctNode {
    pub fn new(min: &Point, max: &Point, mut pc: Vec<Point>) -> OctNode {
        let mut pcppp = Vec::new();
        let mut pcppn = Vec::new();
        let mut pcpnp = Vec::new();
        let mut pcpnn = Vec::new();
        let mut pcnpp = Vec::new();
        let mut pcnpn = Vec::new();
        let mut pcnnp = Vec::new();
        let mut pcnnn = Vec::new();

        let middle = center(min, max);

        for p in pc { //@todo >= or <= ? //@todo define helper method and enum for this
            if p.x >= middle.x && p.Y >= middle.y && p.Z >= middle.Z {
                pcppp.push(p);
            } else if p.x >= middle.x && p.Y >= middle.y && p.Z < middle.Z {
                pcppn.push(p);
            } else if p.x >= middle.x && p.Y < middle.y && p.Z >= middle.Z {
                pcpnp.push(p);
            } else if p.x >= middle.x && p.Y < middle.y && p.Z < middle.Z {
                pcpnn.push(p);
            } else if p.x < middle.x && p.Y >= middle.y && p.Z >= middle.Z {
                pcnpp.push(p);
            } else if p.x < middle.x && p.Y >= middle.y && p.Z < middle.Z {
                pcnpn.push(p);
            } else if p.x >= middle.x && p.Y < middle.y && p.Z < middle.Z {
                pcnnp.push(p);
            } else { //if p.x < middle.x && p.Y < middle.y && p.Z < middle.Z {
                pcnnn.push(p);
            }
        }

        let ppp = match pcppp.len() {
            0 => None,
            _ => Some(Box::new(OctNode::new()))
        }


    }
}
