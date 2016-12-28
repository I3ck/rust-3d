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

use std::fmt;
use std::f64::consts;


pub mod traits;
pub mod functions;
pub mod point_2d;
pub mod point_3d;
pub mod norm_2d;
pub mod norm_3d;
pub mod plane_3d;
pub mod point_cloud_2d;
pub mod point_cloud_3d;
pub mod matrix4;
pub mod matrix4_pipe;
pub mod mesh_3d;
pub mod compressed_point_3d;
pub mod compressed_point_cloud_3d;
pub mod projection_to_plane;
pub mod kd_tree;
pub mod oc_node;
pub mod oc_tree;
pub mod view;

use point_2d::{Point2D};
use point_3d::{Point3D};
use point_cloud_2d::{PointCloud2D};
use point_cloud_3d::{PointCloud3D};
use compressed_point_cloud_3d::{CompressedPointCloud3D};
use kd_tree::{KdTree};
use oc_tree::{OcTree};
use traits::is_2d::Is2D;
use traits::is_3d::Is3D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_editable_2d::IsEditable2D;
use traits::is_editable_3d::IsEditable3D;
use traits::has_bounding_box_3d::HasBoundingBox3D;
use traits::is_tree_3d::IsTree3D;
use traits::is_oc_tree::IsOcTree;
use traits::is_kd_tree_3d::IsKdTree3D;
use traits::is_mesh_3d::IsMesh3D;
use traits::is_editable_mesh_3d::IsEditableMesh3D;
use mesh_3d::Mesh3D;
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

    let (pmin, pmax) = pc.bounding_box().expect("Can't calculate bounding box with less than two elemts");

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
                    let (lenA, lenB) = (extrusionA.len(), extrusionB.len());

                    //println!("extrusionA : {}", extrusionA);
                    //println!("extrusionB : {}", extrusionB);

                    let mut f = File::create("extrusionA.xyz").expect("Could not create file");
                    f.write_all(extrusionA.to_str().as_bytes()).expect("Could not write to file");

                    let mut f = File::create("extrusionB.xyz").expect("Could not create file");
                    f.write_all(extrusionB.to_str().as_bytes()).expect("Could not write to file");


                    let mut mesh = Mesh3D::<Point3D>::new();
                    //vertices of extrusionA
                    for p in extrusionA.data {
                        mesh.add_vertex(*p);
                    }
                    //vertices of extrusionB
                    for p in extrusionB.data {
                        mesh.add_vertex(*p);
                    }
                    mesh.add_vertex(*Point3D::new());
                    mesh.add_vertex(extrustionDir);

                    //faces with base on extrusionA
                    for i in 0..lenA-1 {
                        mesh.try_add_connection(i, i+1, lenA+i).expect("error adding connection in mesh1");
                    }
                    //faces with base on extrusionB
                    for i in 0..lenB-1 {
                        mesh.try_add_connection(lenB+i+1, lenB+i, i+1).expect("error adding connection in mesh2");
                    }

                    //END GENERAL MESHING PART
                    //extrusionA to origin
                    for i in 0..lenA-1 {
                        mesh.try_add_connection(i+1, i, lenA+lenB).expect("error adding connection in mesh3");
                    }
                    //extrusionB to origin
                    for i in 0..lenB-1 {
                        mesh.try_add_connection(lenB+i, lenB+i+1, lenA+lenB+1).expect("error adding connection in mesh4");
                    }

                    mesh.save_ply_ascii("extrusionMesh.ply");
                    mesh.save_stl_ascii("extrusionMesh.stl");
                }
            }
        }
    }
}
