extern crate core;

use std::f64;
use std::fmt;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use self::core::str::FromStr;

use traits::{MoveAble};
use functions::{sqr_dist};

#[derive (PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Point {}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point::new();
        sqr_dist(&origin, self).partial_cmp(&sqr_dist(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl Point {
    pub fn new() -> Point {
        Point{x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn parse(text: String) -> Option<Point> {
        let split = text.split(" ");
        let words = split.collect::<Vec<&str>>();
        match words.len() {
            3 => {
                let mut p = Point::new();
                match f64::from_str(words[0]) {
                    Err(_) => return None,
                    Ok(x) => p.x = x
                };
                match f64::from_str(words[1]) {
                    Err(_) => return None,
                    Ok(y) => p.y = y
                };
                match f64::from_str(words[2]) {
                    Err(_) => return None,
                    Ok(z) => p.z = z
                };
                Some(p)
            },
            _ => None
        }
    }
    //@todo make trait
    pub fn to_str(&self) -> String {
        let sx: String = self.x.to_string();
        let sy: String = self.y.to_string();
        let sz: String = self.z.to_string();

        sx + " " + &sy + " " + &sz
    }
    pub fn clone(&self) -> Point { //@todo use trait?
        Point { x: self.x, y: self.y, z: self.z }
    }
}

impl MoveAble for Point {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
