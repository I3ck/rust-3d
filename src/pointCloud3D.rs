use std::fmt;

use traits::{Is3D, IsMoveable3D, HasPosition3D, HasEditablePosition3D};
use point3D::{Point3D};



pub struct PointCloud3D<P> where P: HasEditablePosition3D {
    pub data: Vec<Box<P>>
}

impl<P> PointCloud3D<P> where P: HasEditablePosition3D{
    pub fn new() -> PointCloud3D<P> {
        PointCloud3D{data: Vec::new()}
    }

    pub fn parse(text: String) -> Option<PointCloud3D<P>> {
        let lines = text.split("\n");

        let mut pc = PointCloud3D::new();
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

    pub fn clone(&self) -> PointCloud3D<P> {
        let mut data: Vec<Box<P>>;
        data = Vec::new();

        for p in &self.data {
            data.push(Box::new((*p).clone()));
        }

        PointCloud3D { data: data }
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

    pub fn bbox(&self) -> Option<(Point3D, Point3D)> { //@todo return P ?
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

        return Some((Point3D{x: minx, y: miny, z: minz}, Point3D{x: maxx, y: maxy, z: maxz}));
    }
}

impl<P> IsMoveable3D for PointCloud3D<P> where P: HasEditablePosition3D + IsMoveable3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

//------------------------------------------------------------------------------

impl<P> fmt::Display for PointCloud3D<P> where P: HasEditablePosition3D + fmt::Display {
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
