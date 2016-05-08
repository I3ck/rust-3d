use std::fmt;


mod traits;
mod functions;
mod point;
mod pointCloud;
mod compressedPoint;
mod compressedPointCloud;
mod kdTree;
mod ocNode;
mod ocTree;

use point::{Point};
use pointCloud::{PointCloud};
use compressedPoint::{CompressedPoint};
use compressedPointCloud::{CompressedPointCloud};
use kdTree::{KdTree};
use ocTree::{OcTree};
use traits::{MoveAble};


//io
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//------------------------------------------------------------------------------

fn main() {
    let p = Point::new();
    let p2 = Point{x: 100.0, y: 200.0, z: 400.0};
    let pCenter = functions::center(&p, &p2);

    let mut pc = PointCloud::new();

    println!("len : {}", pc.len());
    pc.push(p);
    println!("len : {}", pc.len());

    pc.push(p2);
    println!("center : {}", pc.center().expect("Can't calculate center of empty path"));

    let (pmin, pmax) = pc.bbox().expect("Can't calculate bounding box with less than two elemts");

    println!("min : {}", pmin);
    println!("max : {}", pmax);

    let compressed = CompressedPointCloud::<u8>::compress(&pc).expect("Could not compress!");

    let decompressed = compressed.decompress().expect("Could not decompress!");

    println!("{}", decompressed.data[0]);
    println!("{}", decompressed.data[1]);



    println!("pCenter : {}", pCenter);
    println!("pc :\n {}", pc);

    let tree = KdTree::new(pc).expect("Could not parse tree!");

    println!("tree.size() : {}", tree.size());

    let pcFromTree = tree.toPointCloud();

    println!("pcFromTree :\n {}", pcFromTree);

    let nearest = tree.knearest(&Point{x: 10.0,y: 199.0,z: 350.0}, 1);

    println!("single nearest to 100/199/350 : {}", nearest);







    let path = Path::new("exampledata.tmp");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {
            print!("{} contains:\n{}", display, s);

            match PointCloud::parse(String::from(s)) {
                None => {
                    println!("failed to parse pc data!");
                },
                Some(pc) => {
                    println!("parsed len : {}", pc.len());

                    let kdTree = KdTree::new(pc.clone()).expect("Could not parse kdTree!");
                    println!("tree.size() : {}", tree.size());
                    let nearestTen = kdTree.knearest(&Point{x: 9.0,y: 56.0,z: 0.0}, 10);
                    println!("nearest ten to 9/56/0 : {}", nearestTen);

                    let ocTree = OcTree::new(pc).expect("Could not parse ocTree!");
                    println!("could create octree");
                }

            }
        }
    }
}
