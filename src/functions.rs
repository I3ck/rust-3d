use std::cmp::Ordering;

use point::{Point};
use pointCloud::{PointCloud};
use ocNode::{Direction};

pub fn center(p1: &Point, p2: &Point) -> Point {
    Point {
        x: (p1.x + (p2.x - p1.x) / 2.0),
        y: (p1.y + (p2.y - p1.y) / 2.0),
        z: (p1.z + (p2.z - p1.z) / 2.0)
    }
}

pub fn dist(p1: &Point, p2: &Point) -> f64 {
    sqr_dist(p1,p2).sqrt()
}

pub fn sqr_dist(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)
}

pub fn dimension_compare(lhs: &Point, rhs: &Point, dim: i8) -> Option<Ordering> {
    match dim {
        0 => lhs.x.partial_cmp(&rhs.x),
        1 => lhs.y.partial_cmp(&rhs.y),
        2 => lhs.z.partial_cmp(&rhs.z),
        _ => None
    }
}

pub fn dimension_dist(lhs: &Point, rhs: &Point, dim: i8) -> Option<f64> {
    match dim {
        0 => Some((lhs.x - rhs.x).abs()),
        1 => Some((lhs.y - rhs.y).abs()),
        2 => Some((lhs.z - rhs.z).abs()),
        _ => None
    }
}

pub fn sort_and_limit(mut pc: &mut PointCloud, search: &Point, maxSize: usize) {
    if pc.len() > maxSize {
        pc.data.sort_by(|a, b| sqr_dist(search, a).partial_cmp(&sqr_dist(search, b)).unwrap_or(Ordering::Equal));
        let mut result = Vec::new();
        for i in pc.data.iter().take(maxSize) {
            result.push(i.clone());
        }
        pc.data = result;

    }
}

pub fn calc_direction(reference: &Point, p: &Point) -> Direction {
    if p.x >= reference.x && p.y >= reference.y && p.z >= reference.z {
        Direction::PPP
    } else if p.x >= reference.x && p.y >= reference.y && p.z < reference.z {
        Direction::PPN
    } else if p.x >= reference.x && p.y < reference.y && p.z >= reference.z {
        Direction::PNP
    } else if p.x >= reference.x && p.y < reference.y && p.z < reference.z {
        Direction::PNN
    } else if p.x < reference.x && p.y >= reference.y && p.z >= reference.z {
        Direction::NPP
    } else if p.x < reference.x && p.y >= reference.y && p.z < reference.z {
        Direction::NPN
    } else if p.x >= reference.x && p.y < reference.y && p.z >= reference.z {
        Direction::NNP
    } else { //if p.x < reference.x && p.y < reference.y && p.z < reference.z {
        Direction::NNN
    }
}

pub fn calc_sub_min_max(dir: Direction, min: &Point, max: &Point) -> (Point, Point) { //@todo better name
    let middle = center(min, max);

    let px = max.x;
    let py = max.y;
    let pz = max.z;
    let nx = min.x;
    let ny = min.y;
    let nz = min.z;
    let mx = middle.x;
    let my = middle.y;
    let mz = middle.z;

    match dir {
        Direction::PPP => (middle,                         max.clone()),
        Direction::PPN => (Point{x: mx, y: my, z: nz},     Point{x: px, y: py, z: mz}),
        Direction::PNP => (Point{x: mx, y: ny, z: mz},     Point{x: px, y: my, z: pz}),
        Direction::PNN => (Point{x: mx, y: ny, z: nz},     Point{x: px, y: my, z: mz}),
        Direction::NPP => (Point{x: nx, y: my, z: mz},     Point{x: mx, y: py, z: pz}),
        Direction::NPN => (Point{x: nx, y: my, z: nz},     Point{x: mx, y: py, z: mz}),
        Direction::NNP => (Point{x: nx, y: ny, z: mz},     Point{x: mx, y: my, z: pz}),
        Direction::NNN => (min.clone(),                    middle)
    }
}

pub fn in_bb(p: &Point, min: &Point, max: &Point) -> bool {
    p.x >= min.x && p.x <= max.x &&
    p.y >= min.y && p.y <= max.y &&
    p.z >= min.z && p.z <= max.z
}
