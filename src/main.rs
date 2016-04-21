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

    fn center(&self) -> Option<Point> {
        let size = self.len();

        if size < 1 {
            return None;
        }

        let sizef = size as f64;

        let mut sumx: f64 = 0.0;
        let mut sumy: f64 = 0.0;
        let mut sumz: f64 = 0.0;

        for p in &self.data {
            sumx += p.x;
            sumy += p.y;
            sumz += p.z;
        }

        return Some(Point {
            x: (sumx / sizef),
            y: (sumy / sizef),
            z: (sumz / sizef)
        })

    }
}


fn main() {
    let p = Point::new();
    let p2 = Point{x: 100.0, y: 200.0, z: 400.0};
    println!("Gello! {}", p);

    let mut pc = PointCloud::new();

    println!("len : {}", pc.len());
    pc.push(p);
    println!("len : {}", pc.len());

    pc.push(p2);
    println!("center : {}", pc.center().unwrap())

}
