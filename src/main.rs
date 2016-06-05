use std::fmt;
use std::f64::consts;


mod traits;
mod functions;
mod point_2d;
mod point_3d;
mod norm_2d;
mod norm_3d;
mod plane_3d;
mod point_cloud_2d;
mod point_cloud_3d;
mod matrix4;
mod matrix4_pipe;
mod compressed_point_3d;
mod compressed_point_cloud_3d;
mod projection_to_plane;
mod kd_tree;
mod oc_node;
mod oc_tree;

use point_2d::{Point2D};
use point_3d::{Point3D};
use point_cloud_2d::{PointCloud2D};
use point_cloud_3d::{PointCloud3D};
use compressed_point_3d::{CompressedPoint3D};
use compressed_point_cloud_3d::{CompressedPointCloud3D};
use kd_tree::{KdTree};
use oc_tree::{OcTree};
use traits::is_moveable_3d::IsMoveable3D;
use traits::is_2d::Is2D;
use traits::is_3d::Is3D;
use traits::has_position_2d::HasPosition2D;
use traits::has_position_3d::HasPosition3D;
use traits::has_editable_position_2d::HasEditablePosition2D;
use traits::has_editable_position_3d::HasEditablePosition3D;
use traits::is_tree_3d::IsTree3D;
use traits::is_oc_tree::IsOcTree;
use traits::is_kd_tree_3d::IsKdTree3D;
use functions::{extrude, center};

use std::cmp::Ordering;

//io
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


//------------------------------------------------------------------------------

fn main() {
    let p = *Point3D::new();
    let p2 = *Point3D::build(100.0, 200.0, 400.0);
    let pCenter = center(&p, &p2);

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
            //print!("{} contains:\n{}", display, s);

            match PointCloud3D::parse(String::from(s)) {
                None => {
                    //println!("failed to parse pc data!");
                },
                Some(pc) => {
                    //println!("parsed len : {}", pc.len());

                    let mut kd_tree = KdTree::new();
                    kd_tree.build(pc.clone());
                    //println!("kd_tree.size() : {}", tree.size());
                    let nearestTen = kd_tree.knearest(&Point3D{x: 9.0,y: 56.0,z: 0.0}, 10);
                    //println!("nearest ten to 9/56/0 : {}", nearestTen);

                    let mut oc_tree = OcTree::new();
                    oc_tree.build(pc);
                    //println!("could create octree");

                    //println!("oc_tree.size() : {}", oc_tree.size());

                    let collect0 = oc_tree.collect(0);
                    //println!("collect 0 : {}", collect0);

                    let collect1 = oc_tree.collect(1);
                    //println!("collect 1 : {}", collect1);

                    let collect2 = oc_tree.collect(2);
                    //println!("collect 2 : {}", collect2);

                    let collect = oc_tree.collect(1);
                    //println!("collect: {}", collect);

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
            //print!("{} contains:\n{}", display2, s2);

            match PointCloud2D::<Point2D>::parse(String::from(s2)) {
                None => {
                    //println!("failed to parse pc data!");
                },
                Some(mut pc2) => {
                    //println!("parsed len : {}", pc2.len());
                    let origin = *Point2D::new();
                    pc2.data.sort_by(|ref a, ref b| { //@todo improve, really messy this way
                        let c = (***a).clone();
                        let d = (***b).clone();
                    (origin.rad_to(&c)).partial_cmp(&origin.rad_to(&d)).unwrap_or(Ordering::Equal)
                    });

                    let z = 54;
                    let mut pc3 = PointCloud2D::<Point2D>::new();
                    let step = 2.0 * consts::PI / (z as f64);
                    for i in 0..z {
                        let mut pcClone = pc2.clone();
                        for p in pcClone.data {
                            let mut clone = p.clone();
                            clone.rotate((i as f64) * step, &origin);
                            pc3.push(clone);
                        }
                    }
                    pc2 = pc3;

                    let extrustionDir = *Point3D::build(0.0, 0.0, 7.0);
                    let (extrusionA, extrusionB) = extrude::<Point2D, Point3D>(&pc2.data, &extrustionDir);

                    //println!("extrusionA : {}", extrusionA);
                    //println!("extrusionB : {}", extrusionB);

                    let mut f = File::create("extrusionA.xyz").expect("Could not create file");
                    f.write_all(extrusionA.to_str().as_bytes()).expect("Could not write to file");

                    let mut f = File::create("extrusionB.xyz").expect("Could not create file");
                    f.write_all(extrusionB.to_str().as_bytes()).expect("Could not write to file");



                    //testing .ply
                    let mut f = File::create("extrusionMesh.ply").expect("Could not create file");

                    let (lenA, lenB) = (extrusionA.len(), extrusionB.len());
                    let nVertices = lenA + lenB + 2;
                    let nFaces = 2*(lenA-1) + 2*(lenB-1);

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

                    //vertices of extrusionA
                    for p in extrusionA.data {
                        f.write_all((p.to_str() + "\n").as_bytes());
                    }
                    //vertices of extrusionB
                    for p in extrusionB.data {
                        f.write_all((p.to_str() + "\n").as_bytes());
                    }
                    f.write_all((Point3D::new().to_str() + "\n").as_bytes());
                    f.write_all((extrustionDir.to_str() + "\n").as_bytes());
                    //faces with base on extrusionA
                    for i in 0..lenA-1 {
                        f.write_all(("3 ".to_string() + &(i).to_string() + " " + &(i+1).to_string() + " " + &(lenA+i).to_string() + "\n").as_bytes());
                    }
                    //faces with base on extrusionB
                    for i in 0..lenB-1 {
                        f.write_all(("3 ".to_string() + &(lenB+i+1).to_string() + " " + &(lenB+i).to_string() + " " + &(i+1).to_string() + "\n").as_bytes());
                    }

                    //END GENERAL MESHING PART
                    //extrusionA to origin
                    for i in 0..lenA-1 {
                        f.write_all(("3 ".to_string() + &(i+1).to_string() + " " + &(i).to_string() + " " + &(lenA+lenB).to_string() + "\n").as_bytes());
                    }
                    //extrusionB to origin
                    for i in 0..lenB-1 {
                        f.write_all(("3 ".to_string() + &(lenB+i).to_string() + " " + &(lenB+i+1).to_string() + " " + &(lenA+lenB+1).to_string() + "\n").as_bytes());
                    }


                }
            }
        }
    }
}
