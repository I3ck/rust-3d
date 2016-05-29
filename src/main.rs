use std::fmt;


mod traits;
mod functions;
mod point2D;
mod point3D;
mod plane3D;
mod pointCloud2D;
mod pointCloud3D;
mod matrix4;
mod matrix4pipe;
mod compressedPoint3D;
mod compressedPointCloud3D;
mod projectionToPlane;
mod kdTree;
mod ocNode;
mod ocTree;

use point2D::{Point2D};
use point3D::{Point3D};
use pointCloud2D::{PointCloud2D};
use pointCloud3D::{PointCloud3D};
use compressedPoint3D::{CompressedPoint3D};
use compressedPointCloud3D::{CompressedPointCloud3D};
use kdTree::{KdTree};
use ocTree::{OcTree};
use traits::{IsMoveable3D, HasPosition2D, HasPosition3D, IsTree3D, IsOcTree, IsKdTree3D};
use functions::{extrude};


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

    let mut pc = PointCloud3D::new();

    println!("len : {}", pc.len());
    pc.push(p);
    println!("len : {}", pc.len());

    pc.push(p2);
    println!("center : {}", pc.center().expect("Can't calculate center of empty path"));

    let (pmin, pmax) = pc.bbox().expect("Can't calculate bounding box with less than two elemts");

    println!("min : {}", pmin);
    println!("max : {}", pmax);

    let compressed = CompressedPointCloud3D::<u8>::compress(&pc).expect("Could not compress!");
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

            match PointCloud3D::parse(String::from(s)) {
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

    let path2 = Path::new("exampledata.xy");
    let display2 = path2.display();

    let mut file2 = match File::open(&path2) {
        Err(why) => panic!("couldn't open {}: {}", display2,
                                                   Error::description(&why)),
        Ok(file2) => file2
    };

    let mut s2 = String::new();
    match file2.read_to_string(&mut s2) {
        Err(why) => panic!("couldn't read {}: {}", display2,
                                                   Error::description(&why)),
        Ok(_) => {
            print!("{} contains:\n{}", display2, s2);

            match PointCloud2D::<Point2D>::parse(String::from(s2)) {
                None => {
                    println!("failed to parse pc data!");
                },
                Some(pc2) => {
                    println!("parsed len : {}", pc2.len());

                    let extrustionDir = *Point3D::build(0.0, 0.0, 7.0);
                    let (extrusionA, extrusionB) = extrude::<Point2D, Point3D>(&pc2.data, &extrustionDir);

                    println!("extrusionA : {}", extrusionA);
                    println!("extrusionB : {}", extrusionB);

                    let mut f = File::create("extrusionA.xyz").expect("Could not create file");
                    f.write_all(extrusionA.to_str().as_bytes()).expect("Could not write to file");

                    let mut f = File::create("extrusionB.xyz").expect("Could not create file");
                    f.write_all(extrusionB.to_str().as_bytes()).expect("Could not write to file");



                    //testing .ply
                    let mut f = File::create("extrusionMesh.ply").expect("Could not create file");

                    let (lenA, lenB) = (extrusionA.len(), extrusionB.len());
                    let nVertices = lenA + lenB;
                    let nFaces = lenA + lenB - 2;

                    let nVertexString = "element vertex ".to_string() + &nVertices.to_string() + "\n";
                    let nFacesString = "element face ".to_string() + &nFaces.to_string() + "\n";


                    f.write_all(b"ply\n");
                    f.write_all(b"format ascii 1.0           { ascii/binary, format version number }\n");
                    f.write_all(b"comment made by Greg Turk  { comments keyword specified, like all lines }\n");
                    f.write_all(b"comment this file is a cube\n");
                    f.write_all(nVertexString.as_bytes());
                    f.write_all(b"property float x           { vertex contains float \"x\" coordinate }\n");
                    f.write_all(b"property float y           { y coordinate is also a vertex property }\n");
                    f.write_all(b"property float z           { z coordinate, too }\n");
                    f.write_all(nFacesString.as_bytes());
                    f.write_all(b"property list uchar int vertex_index { \"vertex_indices\" is a list of ints }\n");
                    f.write_all(b"end_header                 { delimits the end of the header }\n");

                    for p in extrusionA.data {
                        f.write_all((p.to_str() + "\n").as_bytes());
                    }
                    for p in extrusionB.data {
                        f.write_all((p.to_str() + "\n").as_bytes());
                    }
                    for i in 0..lenA-1 {
                        f.write_all(("3 ".to_string() + &i.to_string() + " " + &(i+1).to_string() + " " + &(lenA+i).to_string() + "\n").as_bytes());
                    }
                    for i in 0..lenB-1 {
                        f.write_all(("3 ".to_string() + &(lenB+i+1).to_string() + " " + &(lenB+i).to_string() + " " + &(i+1).to_string() + "\n").as_bytes());
                    }

                }
            }
        }
    }
}
