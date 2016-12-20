use std::fmt;

use traits::is_2d::Is2D;
use traits::is_moveable_2d::IsMoveable2D;
use traits::is_buildable_2d::IsBuildable2D;
use traits::is_editable_2d::IsEditable2D;
use traits::has_bounding_box_2d::HasBoundingBox2D;
use point_2d::{Point2D};
use functions::dist_2d;



pub struct PointCloud2D<P> where
    P: IsEditable2D {

    pub data: Vec<Box<P>>
}

impl<P> PointCloud2D<P> where
    P: IsEditable2D{

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

    pub fn push(&mut self, p: P) {
        self.data.push(Box::new(p));
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn path_length(&self) -> f64 { //@todo also implement for 3d   (could be defined more generic for both)
        let mut length : f64 = 0.0;
        if self.data.len() < 2 { return length; }

        for i in 0..self.data.len() {
            length += dist_2d(&*self.data[i], &*self.data[i-1]);
        }
        length
    }

    pub fn center(&self) -> Option<P> { //@todo missing test //@todo rename to center of gravity
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
}

impl<P> IsMoveable2D for PointCloud2D<P> where P: IsEditable2D + IsMoveable2D {
    fn move_by(&mut self, x: f64, y: f64) {
        for p in &mut self.data {
            p.move_by(x, y);
        }
    }
}

impl<P> HasBoundingBox2D for PointCloud2D<P> where P: IsEditable2D {
    fn bounding_box(&self) -> Option<(Point2D, Point2D)> {
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

impl<P> fmt::Display for PointCloud2D<P> where
    P: IsEditable2D + fmt::Display {

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
