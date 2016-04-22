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

    fn bbox(&self) -> Option<(Point, Point)> {
        if self.len() < 2 {
            return None;
        }

        let mut minx = self.data[0].x;
        let mut miny = self.data[0].y;
        let mut minz = self.data[0].z;
        let mut maxx = self.data[0].x;
        let mut maxy = self.data[0].y;
        let mut maxz = self.data[0].z;

        for p in &self.data {
            if p.x < minx { minx = p.x; }
            if p.y < miny { miny = p.y; }
            if p.z < minz { minz = p.z; }
            if p.x > maxx { maxx = p.x; }
            if p.y > maxy { maxy = p.y; }
            if p.z > maxz { maxz = p.z; }
        }

        return Some((Point{x: minx, y: miny, z: minz}, Point{x: maxx, y: maxy, z: maxz}));
    }
}

//------------------------------------------------------------------------------

struct CompressedPoint { ///@todo u16 templated
    unitsx: u16,
    unitsy: u16,
    unitsz: u16
}

struct CompressedPointCloud {
    start: Point,
    unitsizex: f64,
    unitsizey: f64,
    unitsizez: f64,
    data: Vec<CompressedPoint>
}

impl CompressedPointCloud {
    fn new(pc: &PointCloud) -> CompressedPointCloud {
        let (pmin, pmax) = pc.bbox().expect("Can't compress PointCloud with less than two points");

        let rangex = (pmax.x - pmin.x).abs();
        let rangey = (pmax.y - pmin.y).abs();
        let rangez = (pmax.z - pmin.z).abs();

        let unitsizex = rangex / (u16::max_value() as f64);
        let unitsizey = rangey / (u16::max_value() as f64);
        let unitsizez = rangez / (u16::max_value() as f64);

        let mut data = Vec::new();

        for p in &pc.data {
            let distx = p.x - pmin.x;
            let disty = p.y - pmin.y;
            let distz = p.z - pmin.z;

            data.push(CompressedPoint{
                unitsx: (distx / unitsizex) as u16,
                unitsy: (disty / unitsizey) as u16,
                unitsz: (distz / unitsizez) as u16
            })
        }
        return CompressedPointCloud{start: pmin, unitsizex: unitsizex, unitsizey: unitsizey, unitsizez: unitsizez, data: data};
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
    println!("center : {}", pc.center().expect("Can't calculate center of empty path"));

    let (pmin, pmax) = pc.bbox().expect("Can't calculate bounding box with less than two elemts");

    println!("min : {}", pmin);
    println!("max : {}", pmax);

    let compressed = CompressedPointCloud::new(&pc);

}
