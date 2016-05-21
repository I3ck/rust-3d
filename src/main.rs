use std::fmt;


mod traits;
mod functions;
mod point2D;
mod point3D;
mod pointCloud3D;
mod compressedPoint3D;
mod compressedPointCloud3D;
mod kdTree;
mod ocNode;
mod ocTree;

use point3D::{Point3D};
use pointCloud3D::{Point3DCloud3D};
use compressedPoint3D::{CompressedPoint3D};
use compressedPointCloud3D::{CompressedPoint3DCloud3D};
use kdTree::{KdTree};
use ocTree::{OcTree};
use traits::{IsMoveable3D, HasPosition3D, IsTree3D, IsOcTree, IsKdTree3D};


//io
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//------------------------------------------------------------------------------

fn main() {
    let p = *Point3D::new();
    let p2 = *Point3D::build(100.0, 200.0, 400.0);
    let mut pCenter = Point3D::new();
    functions::center(&p, &p2, &mut pCenter);

    let mut pc = Point3DCloud3D::new();

    println!("len : {}", pc.len());
    pc.push(p);
    println!("len : {}", pc.len());

    pc.push(p2);
    println!("center : {}", pc.center().expect("Can't calculate center of empty path"));

    let (pmin, pmax) = pc.bbox().expect("Can't calculate bounding box with less than two elemts");

    println!("min : {}", pmin);
    println!("max : {}", pmax);

    let compressed = CompressedPoint3DCloud3D::<u8>::compress(&pc).expect("Could not compress!");
    let decompressed = compressed.decompress::<Point3D>().expect("Could not decompress!");

    println!("{}", decompressed.data[0]);
    println!("{}", decompressed.data[1]);



    println!("pCenter : {}", pCenter);
    println!("pc :\n {}", pc);

    let mut tree = KdTree::new();
    tree.build(pc);

    println!("tree.size() : {}", tree.size());

    let pcFromTree = tree.to_pointcloud();

    println!("pcFromTree :\n {}", pcFromTree);

    let nearest = tree.knearest(&Point3D{x: 10.0,y: 199.0,z: 350.0}, 1);

    println!("single nearest to 100/199/350 : {}", nearest);







    let path = Path::new("exampledata.xyz");
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

            match Point3DCloud3D::parse(String::from(s)) {
                None => {
                    println!("failed to parse pc data!");
                },
                Some(pc) => {
                    println!("parsed len : {}", pc.len());

                    let mut kdTree = KdTree::new();
                    kdTree.build(pc.clone());
                    println!("kdTree.size() : {}", tree.size());
                    let nearestTen = kdTree.knearest(&Point3D{x: 9.0,y: 56.0,z: 0.0}, 10);
                    println!("nearest ten to 9/56/0 : {}", nearestTen);

                    let mut ocTree = OcTree::new();
                    ocTree.build(pc);
                    println!("could create octree");

                    println!("ocTree.size() : {}", ocTree.size());

                    let collect0 = ocTree.collect(0);
                    println!("collect 0 : {}", collect0);

                    let collect1 = ocTree.collect(1);
                    println!("collect 1 : {}", collect1);

                    let collect2 = ocTree.collect(2);
                    println!("collect 2 : {}", collect2);

                    let collect = ocTree.collect(1);
                    println!("collect: {}", collect);

                    let mut f = File::create("collect.xyz").expect("Could not create file");
                    f.write_all(collect.to_str().as_bytes()).expect("Could not write to file");
                }
            }
        }
    }
}
