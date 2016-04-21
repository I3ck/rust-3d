use std::fmt;

struct Point {
    x: f64,
    y: f64,
    z: f64
}

trait MoveAble {
    fn move_by(&mut self, p: Point);
}

impl MoveAble for Point {
    fn move_by(&mut self, p: Point) {
        self.x += p.x;
        self.y += p.y;
        self.z += p.z;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point {
    fn new() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }
}


struct PointCloud {
    data: Vec<Point>
}

impl PointCloud {
    fn new() -> PointCloud {
        PointCloud{data: Vec::new()}
    }

    fn push(&mut self, p: Point) {
        self.data.push(p);
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}


fn main() {
    let p = Point::new();
    println!("Gello! {}", p);

    let mut pc = PointCloud::new();

    println!("len : {}", pc.len());
    pc.push(p);
    println!("len : {}", pc.len());

}
