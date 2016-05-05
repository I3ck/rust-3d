use point::{Point};
use pointCloud::{PointCloud};
use functions::{center};
//@todo either merge Oct code or splot KdNode and Tree into seperate files

pub enum OctNode {
    Leaf(Point),
    Internal {
        tfl: Option<Box<OctNode>>, //top front left
        tfr: Option<Box<OctNode>>, //top front right
        tbl: Option<Box<OctNode>>, //top back left
        tbr: Option<Box<OctNode>>, //top back right
        bfl: Option<Box<OctNode>>, //bottom front left
        bfr: Option<Box<OctNode>>, //bottom front right
        bbl: Option<Box<OctNode>>, //bottom back left
        bbr: Option<Box<OctNode>>  //bottom back right
    }
}

impl OctNode {
    pub fn new(min: &Point, max: &Point, mut pc: Vec<Point>) -> OctNode {
        let mut pctfl = Vec::new(); //@todo rename all these to positive/negative x/y/z (also nodes in struct def)
        let mut pctfr = Vec::new();
        let mut pctbl = Vec::new();
        let mut pctbr = Vec::new();
        let mut pcbfl = Vec::new();
        let mut pcbfr = Vec::new();
        let mut pcbbl = Vec::new();
        let mut pcbbr = Vec::new();

        let middle = center(min, max);

        for p in pc { //@todo >= or <= ? //@todo define helper method and enum for this
            if p.x >= middle.x && p.Y >= middle.y && p.Z >= middle.Z {
                pctfl.push(p);
            }
            else if p.x < middle.x && p.Y >= middle.y && p.Z >= middle.Z {
                pctfr.push(p);
            }
            else if p.x >= middle.x && p.Y < middle.y && p.Z >= middle.Z {
                pctbl.push(p);
            }
            else if p.x < middle.x && p.Y < middle.y && p.Z >= middle.Z {
                pctbr.push(p);
            }
            if p.x >= middle.x && p.Y >= middle.y && p.Z < middle.Z {
                pcbfl.push(p);
            }
            else if p.x < middle.x && p.Y >= middle.y && p.Z < middle.Z {
                pcbfr.push(p);
            }
            else if p.x >= middle.x && p.Y < middle.y && p.Z < middle.Z {
                pcbbl.push(p);
            }
            else // if p.x < middle.x && p.Y < middle.y && p.Z < middle.Z {
                pcbbr.push(p);
            }
        }

        let tfl = match pctfl.len() {
            0 => None,
            _ => Some(Box::new(OctNode::new(TODO)))
        }


    }
}
