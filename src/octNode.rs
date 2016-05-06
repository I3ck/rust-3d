use point::{Point};
use pointCloud::{PointCloud};
use functions::{center, calcSubMinMax};
//@todo either merge Oct code or splot KdNode and Tree into seperate files

pub enum OctNode {
    Leaf(Point),
    Node(Internal)
}

struct Internal { // naming : p == positive, n == negative ||| xyz   => pnp => x positive, y negative, z positive direction from center
    ppp: Option<Box<OctNode>>,
    ppn: Option<Box<OctNode>>,
    pnp: Option<Box<OctNode>>,
    pnn: Option<Box<OctNode>>,
    npp: Option<Box<OctNode>>,
    npn: Option<Box<OctNode>>,
    nnp: Option<Box<OctNode>>,
    nnn: Option<Box<OctNode>>
}

pub enum Direction { //@todo rename //@todo private?
    PPP,
    PPN,
    PNP,
    PNN,
    NPP,
    NPN,
    NNP,
    NNN
}

impl OctNode {
    pub fn new(min: &Point, max: &Point, mut pc: Vec<Point>) -> OctNode {
        if pc.len() == 1 { return OctNode::Leaf(pc[0].clone()); }

        let mut pcppp = Vec::new();
        let mut pcppn = Vec::new();
        let mut pcpnp = Vec::new();
        let mut pcpnn = Vec::new();
        let mut pcnpp = Vec::new();
        let mut pcnpn = Vec::new();
        let mut pcnnp = Vec::new();
        let mut pcnnn = Vec::new();

        let middle = center(min, max);

        //@todo if pc empty => node
        for p in pc { //@todo >= or <= ? //@todo define helper method and enum for this
            if p.x >= middle.x && p.y >= middle.y && p.z >= middle.z {
                pcppp.push(p);
            } else if p.x >= middle.x && p.y >= middle.y && p.z < middle.z {
                pcppn.push(p);
            } else if p.x >= middle.x && p.y < middle.y && p.z >= middle.z {
                pcpnp.push(p);
            } else if p.x >= middle.x && p.y < middle.y && p.z < middle.z {
                pcpnn.push(p);
            } else if p.x < middle.x && p.y >= middle.y && p.z >= middle.z {
                pcnpp.push(p);
            } else if p.x < middle.x && p.y >= middle.y && p.z < middle.z {
                pcnpn.push(p);
            } else if p.x >= middle.x && p.y < middle.y && p.z < middle.z {
                pcnnp.push(p);
            } else { //if p.x < middle.x && p.y < middle.y && p.z < middle.z {
                pcnnn.push(p);
            }
        }

        let ppp = match pcppp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::PPP, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcppp)))
            }
        };

        let ppn = match pcppn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::PPN, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcppn)))
            }
        };

        let pnp = match pcpnp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::PNP, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcpnp)))
            }
        };

        let pnn = match pcpnn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::PNN, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcpnn)))
            }
        };

        let npp = match pcnpp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::NPP, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcnpp)))
            }
        };

        let npn = match pcnpn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::NPN, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcnpn)))
            }
        };

        let nnp = match pcnnp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::NNP, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcnnp)))
            }
        };

        let nnn = match pcnnn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = calcSubMinMax(Direction::NNN, min, max);
                Some(Box::new(OctNode::new(&newMin, &newMax, pcnnn)))
            }
        };

        let mut result: Internal = Internal {
            ppp: ppp,
            ppn: ppn,
            pnp: pnp,
            pnn: pnn,
            npp: npp,
            npn: npn,
            nnp: nnp,
            nnn: nnn
        };

        OctNode::Node(result)
    }



}
