use std::fmt;


mod structs;
mod traits;
mod impls;
mod functions;
mod kdTree;

use structs::{Point, PointCloud, CompressedPoint, CompressedPointCloud};
use kdTree::{KdTree};
use traits::{MoveAble};


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

}
