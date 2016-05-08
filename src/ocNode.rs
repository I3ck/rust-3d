use point::{Point};
use pointCloud::{PointCloud};
use functions::{center, calcSubMinMax, calcDirection, inBB};
//@todo either merge Oct code or split KdNode and Tree into seperate files

pub enum OcNode {
    Leaf(Point),
    Node(Internal)
}

struct Internal { // naming : p == positive, n == negative ||| xyz   => pnp => x positive, y negative, z positive direction from center
    ppp: Option<Box<OcNode>>,
    ppn: Option<Box<OcNode>>,
    pnp: Option<Box<OcNode>>,
    pnn: Option<Box<OcNode>>,
    npp: Option<Box<OcNode>>,
    npn: Option<Box<OcNode>>,
    nnp: Option<Box<OcNode>>,
    nnn: Option<Box<OcNode>>
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

impl OcNode {
    pub fn new(min: &Point, max: &Point, mut pc: Vec<Point>) -> OcNode {
        if pc.len() == 1 { return OcNode::Leaf(pc[0].clone()); };
        let mut pcppp = Vec::new();
        let mut pcppn = Vec::new();
        let mut pcpnp = Vec::new();
        let mut pcpnn = Vec::new();
        let mut pcnpp = Vec::new();
        let mut pcnpn = Vec::new();
        let mut pcnnp = Vec::new();
        let mut pcnnn = Vec::new();

        let bbppp = calcSubMinMax(Direction::PPP, min, max);
        let bbppn = calcSubMinMax(Direction::PPN, min, max);
        let bbpnp = calcSubMinMax(Direction::PNP, min, max);
        let bbpnn = calcSubMinMax(Direction::PNN, min, max);
        let bbnpp = calcSubMinMax(Direction::NPP, min, max);
        let bbnpn = calcSubMinMax(Direction::NPN, min, max);
        let bbnnp = calcSubMinMax(Direction::NNP, min, max);
        let bbnnn = calcSubMinMax(Direction::NNN, min, max);

        for p in pc {
            if inBB(&p, &bbppp.0, &bbppp.1) {
                pcppp.push(p);
            } else if inBB(&p, &bbppn.0, &bbppn.1) {
                pcppn.push(p);
            } else if inBB(&p, &bbpnp.0, &bbpnp.1) {
                pcpnp.push(p);
            } else if inBB(&p, &bbpnn.0, &bbpnn.1) {
                pcpnn.push(p);
            } else if inBB(&p, &bbnpp.0, &bbnpp.1) {
                pcnpp.push(p);
            } else if inBB(&p, &bbnpn.0, &bbnpn.1) {
                pcnpn.push(p);
            } else if inBB(&p, &bbnnp.0, &bbnnp.1) {
                pcnnp.push(p);
            } else if inBB(&p, &bbnnn.0, &bbnnn.1) {
                pcnnn.push(p);
            }
        }

        let ppp = match pcppp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbppp;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcppp)))
            }
        };

        let ppn = match pcppn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbppn;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcppn)))
            }
        };

        let pnp = match pcpnp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbpnp;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcpnp)))
            }
        };

        let pnn = match pcpnn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbpnn;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcpnn)))
            }
        };

        let npp = match pcnpp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbnpp;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcnpp)))
            }
        };

        let npn = match pcnpn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbnpn;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcnpn)))
            }
        };

        let nnp = match pcnnp.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbnnp;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcnnp)))
            }
        };

        let nnn = match pcnnn.len() {
            0 => None,
            _ => {
                let (newMin, newMax) = bbnnn;
                Some(Box::new(OcNode::new(&newMin, &newMax, pcnnn)))
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

        OcNode::Node(result)
    }
}
