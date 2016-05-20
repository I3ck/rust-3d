use std::fmt;

use traits::{IsMoveable, HasPosition};
use point::{Point};



pub struct PointCloud<P> where P: HasPosition {
    pub data: Vec<Box<P>>
}

impl<P> PointCloud<P> where P: HasPosition{
    pub fn new() -> PointCloud<P> {
        PointCloud{data: Vec::new()}
    }

    pub fn parse(text: String) -> Option<PointCloud<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud::new();
        for line in lines {
            match P::parse(String::from(line)) { //@todo must be templated too
                Some(p) => pc.push(*p),
                None => {}
            }
        }
        if pc.len() == 0 { return None; }
        Some(pc)
    }

    pub fn to_str(&self) -> String {
        let mut result = String::new();

        for p in &self.data {
            result = result + &p.to_str() + "\n";
        }

        result
    }

    pub fn clone(&self) -> PointCloud<P> {
        let mut data: Vec<Box<P>>;
        data = Vec::new();

        for p in &self.data {
            data.push(Box::new((*p).clone()));
        }

        PointCloud { data: data }
    }

//------------------------------------------------------------------------------

    pub fn push(&mut self, p: P) {
        self.data.push(Box::new(p));
    }

//------------------------------------------------------------------------------

    pub fn len(&self) -> usize {
        self.data.len()
    }

//------------------------------------------------------------------------------

    pub fn center(&self) -> Option<P> {
        let size = self.len();

        if size < 1 {
            return None;
        }

        let sizef = size as f64;

        let mut sumx: f64 = 0.0;
        let mut sumy: f64 = 0.0;
        let mut sumz: f64 = 0.0;

        for p in &self.data {
            sumx += p.x();
            sumy += p.y();
            sumz += p.z();
        }

        return Some(*P::build(
            (sumx / sizef),
            (sumy / sizef),
            (sumz / sizef)
        ))
    }

//------------------------------------------------------------------------------

    pub fn bbox(&self) -> Option<(Point, Point)> { //@todo return P ?
        if self.len() < 2 {
            return None;
        }

        let mut minx = self.data[0].x();
        let mut miny = self.data[0].y();
        let mut minz = self.data[0].z();
        let mut maxx = self.data[0].x();
        let mut maxy = self.data[0].y();
        let mut maxz = self.data[0].z();

        for p in &self.data {
            if p.x() < minx { minx = p.x(); }
            if p.y() < miny { miny = p.y(); }
            if p.z() < minz { minz = p.z(); }
            if p.x() > maxx { maxx = p.x(); }
            if p.y() > maxy { maxy = p.y(); }
            if p.z() > maxz { maxz = p.z(); }
        }

        return Some((Point{x: minx, y: miny, z: minz}, Point{x: maxx, y: maxy, z: maxz}));
    }
}

impl<P> IsMoveable for PointCloud<P> where P: HasPosition + IsMoveable {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

//------------------------------------------------------------------------------

impl<P> fmt::Display for PointCloud<P> where P: HasPosition + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.data {
            match p.fmt(f) {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
            match f.write_str("\n") {
                Ok(_) => (),
                Err(err) => return Err(err)
            }
        }
        return Ok(());
    }
}
