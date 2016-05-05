extern crate num;

use std::fmt;
use std::cmp;
use std::cmp::Ordering;

use self::num::traits::PrimInt;
use self::num::traits::Unsigned;

use structs::{Point, PointCloud, CompressedPoint, CompressedPointCloud, KdTree, KdNode};
use traits::{MoveAble};
use functions::{dist, sqr_dist, dimension_compare, dimension_dist, sort_and_limit};

//------------------------------------------------------------------------------

impl MoveAble for Point {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

//------------------------------------------------------------------------------

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//------------------------------------------------------------------------------

impl Point {
    pub fn new() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn clone(&self) -> Point { //@todo use trait?
        Point { x: self.x, y: self.y, z: self.z }
    }
}

//------------------------------------------------------------------------------
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

impl MoveAble for PointCloud {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        for p in &mut self.data {
            p.move_by(x, y, z);
        }
    }
}

//------------------------------------------------------------------------------

impl fmt::Display for PointCloud {
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

//------------------------------------------------------------------------------

impl PointCloud {
    pub fn new() -> PointCloud {
        PointCloud{data: Vec::new()}
    }

//------------------------------------------------------------------------------

    pub fn push(&mut self, p: Point) {
        self.data.push(p);
    }

//------------------------------------------------------------------------------

    pub fn len(&self) -> usize {
        self.data.len()
    }

//------------------------------------------------------------------------------

    pub fn center(&self) -> Option<Point> {
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

//------------------------------------------------------------------------------

    pub fn bbox(&self) -> Option<(Point, Point)> {
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
//------------------------------------------------------------------------------
//------------------------------------------------------------------------------

impl<T> CompressedPointCloud<T> where T: Unsigned + PrimInt {
    pub fn compress(pc: &PointCloud) -> Option<CompressedPointCloud<T>> {
        let (pmin, pmax) = match pc.bbox() {
            None        => return None,
            Some(res)   => res,
        };

        let rangex = (pmax.x - pmin.x).abs();
        let rangey = (pmax.y - pmin.y).abs();
        let rangez = (pmax.z - pmin.z).abs();

        let maxval = match T::max_value().to_f64() {
            None        => return None,
            Some(res)   => res,
        };

        let unitsizex = rangex / maxval;
        let unitsizey = rangey / maxval;
        let unitsizez = rangez / maxval;

        let mut data = Vec::new();

        for p in &pc.data {
            let distx = p.x - pmin.x;
            let disty = p.y - pmin.y;
            let distz = p.z - pmin.z;

            let unitsx = match T::from(distx / unitsizex) {
                None        => return None,
                Some(res)   => res
            };

            let unitsy = match T::from(disty / unitsizey) {
                None        => return None,
                Some(res)   => res
            };

            let unitsz = match T::from(distz / unitsizez) {
                None        => return None,
                Some(res)   => res
            };

            data.push(CompressedPoint{
                unitsx: unitsx,
                unitsy: unitsy,
                unitsz: unitsz
            })
        }
        return Some(CompressedPointCloud::<T>{start: pmin, unitsizex: unitsizex, unitsizey: unitsizey, unitsizez: unitsizez, data: data});
    }

//------------------------------------------------------------------------------

    pub fn decompress(&self) -> Option<PointCloud> {
        let mut pc = PointCloud::new();

        for p in &self.data {
            if let (Some(unitsxf), Some(unitsyf), Some(unitszf)) = (p.unitsx.to_f64(), p.unitsy.to_f64(), p.unitsz.to_f64()) {
                pc.push(Point{
                    x: self.start.x + (self.unitsizex * unitsxf),
                    y: self.start.y + (self.unitsizey * unitsyf),
                    z: self.start.z + (self.unitsizez * unitszf)
                });
            }
        }
        return Some(pc);
    }
}

impl KdTree {
    pub fn new(pc: PointCloud) -> Option<KdTree> {
        match pc.len() {
            0 => None,
            _ => Some(KdTree{root: KdNode::new(0, pc.data)})
        }
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn toPointCloud(&self) -> PointCloud {
        let mut result = PointCloud::new();
        self.root.toPointCloud(&mut result);
        result
    }

    pub fn knearest(&self, search: &Point, n: usize) -> PointCloud {
        let mut result = PointCloud::new();
        if n < 1 { return result; }
        self.root.knearest(search, n, &mut result);
        return result;
    }

    pub fn in_sphere(&self, search: &Point, radius: f64) -> PointCloud {
        let mut result = PointCloud::new();
        if radius <= 0.0 { return result; }
        self.root.in_sphere(search, radius, &mut result);
        return result;
    }

    pub fn in_box(&self, search: &Point, xSize: f64, ySize: f64, zSize: f64) -> PointCloud {
        let mut result = PointCloud::new();
        if xSize <= 0.0 || ySize <= 0.0 || zSize <= 0.0 { return result; }
        self.root.in_box(search, xSize, ySize, zSize, &mut result);
        return result;
    }

    pub fn nearest(&self, search: &Point) -> PointCloud { //@todo implemented on its own, since the code can be faster without vecs
        self.knearest(search, 1)
    }

}

impl KdNode {
    pub fn new(dim: i8, mut pc: Vec<Point>) -> KdNode {
        let dimension = dim % 2;
        let mut val = Point::new();
        if pc.len() == 1 {
            return KdNode {
                left: None,
                right: None,
                val: pc[0].clone(),
                dimension: dimension
            }
        }

        pc.sort_by(|a, b| match dimension {
            0 => a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal),
            1 => a.y.partial_cmp(&b.y).unwrap_or(Ordering::Equal),
            2 => a.z.partial_cmp(&b.z).unwrap_or(Ordering::Equal),
            _ => Ordering::Equal
        });
        let median = pc.len() / 2;
        let mut pcLeft = Vec::new();
        let mut pcRight = Vec::new();

        let mut val = Point::new();

        for (i, p) in pc.into_iter().enumerate() {
            if      i < median  { pcLeft.push(p); }
            else if i > median  { pcRight.push(p); }
            else                { val = p; }
        }

        let left = match pcLeft.len() {
            0 => None,
            _ => Some(Box::new(KdNode::new(dimension+1, pcLeft)))
        };

        let right = match pcRight.len() {
            0 => None,
            _ => Some(Box::new(KdNode::new(dimension+1, pcRight)))
        };

        KdNode {
            left: left,
            right: right,
            val: val,
            dimension: dimension
        }
    }

    pub fn size(&self) -> usize {
        let mut result: usize = 0;
        if let Some(ref n) = (&self).left { result += n.size(); }
        result += 1;
        if let Some(ref n) = (&self).right { result += n.size(); }
        result
    }

    pub fn toPointCloud(&self, pc: &mut PointCloud) {
        if let Some(ref n) = (&self).left { n.toPointCloud(pc); }
        pc.push(self.val.clone());
        if let Some(ref n) = (&self).right { n.toPointCloud(pc); }
    }

    pub fn knearest(&self, search: &Point, n: usize, pc: &mut PointCloud) {
        if pc.len() < n || sqr_dist(search, &self.val) < sqr_dist(search, &pc.data[&pc.len() -1 ]) {
            pc.push(self.val.clone());
        }

        let comp = dimension_compare(search, &self.val, self.dimension);

        match comp {
            Some(res) => match res {
                Ordering::Less  => if let Some(ref node) = (&self).left { node.knearest(search, n, pc); },
                _               => if let Some(ref node) = (&self).right { node.knearest(search, n, pc); }
            },
            None => {}
        }

        sort_and_limit(pc, search, n);

        let (currentSearch, currentVal) = match self.dimension {
            0 => (search.x, self.val.x),
            1 => (search.y, self.val.y),
            _ => (search.z, self.val.z)
        };

        let distanceBest = dist(search, &pc.data[&pc.len() -1 ]);
        let borderLeft = currentSearch - distanceBest;
        let borderRight = currentSearch + distanceBest;

        match comp {
            Some(res) => match res {
                Ordering::Less => if let Some(ref node) = (&self).right {
                    if pc.len() < n || borderRight >= currentVal {
                        node.knearest(search, n, pc);
                    }
                },
                Ordering::Greater => if let Some(ref node) = (&self).left {
                    if pc.len() < n || borderRight <= currentVal {
                        node.knearest(search, n, pc);
                    }
                },
                Ordering::Equal => {}
            },
            None => {}
        }

        sort_and_limit(pc, search, n);
    }

    pub fn in_sphere(&self, search: &Point, radius: f64, pc: &mut PointCloud) {
        if radius <= 0.0 { return; }

        if dist(search, &self.val) <= radius {
            pc.push(self.val.clone());
        }

        if self.is_leaf() { return; }

        let comp = dimension_compare(search, &self.val, self.dimension);

        match comp {
            Some(res) => match res {
                Ordering::Less  => if let Some(ref node) = (&self).left { node.in_sphere(search, radius, pc); },
                _               => if let Some(ref node) = (&self).right { node.in_sphere(search, radius, pc); }
            },
            None => {}
        }

        let (currentSearch, currentVal) = match self.dimension {
            0 => (search.x, self.val.x),
            1 => (search.y, self.val.y),
            _ => (search.z, self.val.z)
        };

        let borderLeft = currentSearch - radius;
        let borderRight = currentSearch + radius;



        match comp {
            Some(res) => match res {
                Ordering::Less => if let Some(ref node) = (&self).right {
                    if borderRight >= currentVal {
                        node.in_sphere(search, radius, pc);
                    }
                },
                Ordering::Greater => if let Some(ref node) = (&self).left {
                    if borderRight <= currentVal {
                        node.in_sphere(search, radius, pc);
                    }
                },
                Ordering::Equal => {}
            },
            None => {}
        }
    }

    pub fn in_box(&self, search: &Point, xSize: f64, ySize: f64, zSize: f64, pc: &mut PointCloud) {
        if xSize <= 0.0 || ySize <= 0.0 || zSize <= 0.0 { return; }

        if let (Some(distX), Some(distY), Some(distZ)) = (dimension_dist(search, &self.val, 0), dimension_dist(search, &self.val, 1), dimension_dist(search, &self.val, 2)) {
            if distX <= 0.5 * xSize && distY <= 0.5 * ySize && distZ <= 0.5 * zSize {
                pc.push(self.val.clone());
            }

            if self.is_leaf()  { return; }

            let comp = dimension_compare(search, &self.val, self.dimension);

            match comp {
                Some(res) => match res {
                    Ordering::Less  => if let Some(ref node) = (&self).left { node.in_box(search, xSize, ySize, zSize, pc); },
                    _               => if let Some(ref node) = (&self).right { node.in_box(search, xSize, ySize, zSize, pc); }
                },
                None => {}
            }

            let (currentSearch, currentVal, currentSize) = match self.dimension {
                0 => (search.x, self.val.x, xSize),
                1 => (search.y, self.val.y, ySize),
                _ => (search.z, self.val.z, zSize)
            };

            let borderLeft = currentSearch - 0.5 * currentSize;
            let borderRight = currentSearch + 0.5 * currentSize;

            match comp {
                Some(res) => match res {
                    Ordering::Less => if let Some(ref node) = (&self).right {
                        if borderRight >= currentVal {
                            node.in_box(search, xSize, ySize, zSize, pc);
                        }
                    },
                    Ordering::Greater => if let Some(ref node) = (&self).left {
                        if borderRight <= currentVal {
                            node.in_box(search, xSize, ySize, zSize, pc);
                        }
                    },
                    Ordering::Equal => {}
                },
                None => {}
            }
        }
    }



    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }


}
