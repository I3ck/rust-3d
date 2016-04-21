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


fn main() {
    let p = Point{x: 1.0, y: 2.0, z: 3.0};
    println!("Gello! {}", p);
}
