use std::fmt;

use traits::is2D::Is2D;
use traits::isMoveable2D::IsMoveable2D;
use traits::hasPosition2D::HasPosition2D;
use traits::hasEditablePosition2D::HasEditablePosition2D;
use point2D::{Point2D};



pub struct PointCloud2D<P> where P: HasEditablePosition2D {
    pub data: Vec<Box<P>>
}

impl<P> PointCloud2D<P> where P: HasEditablePosition2D{
    pub fn new() -> PointCloud2D<P> {
        PointCloud2D{data: Vec::new()}
    }

    pub fn parse(text: String) -> Option<PointCloud2D<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud2D::new();
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

    pub fn clone(&self) -> PointCloud2D<P> {
        let mut data: Vec<Box<P>>;
        data = Vec::new();

        for p in &self.data {
            data.push(Box::new((*p).clone()));
        }

        PointCloud2D { data: data }
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

        for p in &self.data {
            sumx += p.x();
            sumy += p.y();
        }

        return Some(*P::build(
            (sumx / sizef),
            (sumy / sizef)
        ))
    }

//------------------------------------------------------------------------------

    pub fn bbox(&self) -> Option<(Point2D, Point2D)> { //@todo return P ?
        if self.len() < 2 {
            return None;
        }

        let mut minx = self.data[0].x();
        let mut miny = self.data[0].y();
        let mut maxx = self.data[0].x();
        let mut maxy = self.data[0].y();

        for p in &self.data {
            if p.x() < minx { minx = p.x(); }
            if p.y() < miny { miny = p.y(); }
            if p.x() > maxx { maxx = p.x(); }
            if p.y() > maxy { maxy = p.y(); }
        }

        return Some((Point2D{x: minx, y: miny}, Point2D{x: maxx, y: maxy}));
    }
}

impl<P> IsMoveable2D for PointCloud2D<P> where P: HasEditablePosition2D + IsMoveable2D {
    fn move_by(&mut self, x: f64, y: f64) {
        for p in &mut self.data {
            p.move_by(x, y);
        }
    }
}

//------------------------------------------------------------------------------

impl<P> fmt::Display for PointCloud2D<P> where P: HasEditablePosition2D + fmt::Display {
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
