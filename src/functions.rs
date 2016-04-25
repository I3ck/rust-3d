use structs::{Point, PointCloud, CompressedPoint, CompressedPointCloud};

pub fn center(p1: &Point, p2: &Point) -> Point {
    Point {
        x: (p1.x + (p2.x - p1.x) / 2.0),
        y: (p1.y + (p2.y - p1.y) / 2.0),
        z: (p1.z + (p2.z - p1.z) / 2.0)
    }
}
