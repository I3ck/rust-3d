/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

//! Module containing the HasBoundingBox2D trait for types which might have a bounding box

use result::*;
use traits::is_2d::*;
use traits::is_buildable_2d::*;
use point_2d::*;

pub trait HasBoundingBox2D {
    fn bounding_box(&self) -> Result<(Point2D, Point2D)>;

    fn min_pos(&self) -> Result<Point2D> {
        let (min,_) = try!(self.bounding_box());
        Ok(min)
    }

    fn max_pos(&self) -> Result<Point2D> {
        let (_,max) = try!(self.bounding_box());
        Ok(max)
    }

    fn size_x(&self) -> Result<f64> { //@todo change signature to return a Positive
        let (min, max) = try!(self.bounding_box());
        Ok((max.x() - min.x()).abs())
    }

    fn size_y(&self) -> Result<f64> {
        let (min, max) = try!(self.bounding_box());
        Ok((max.y() - min.y()).abs())
    }

    fn center_bb(&self) -> Result<Point2D> {
        let (min, max) = try!(self.bounding_box());
        Ok(*Point2D::build(min.x() + (max.x() - min.x()) / 2.0,
                           min.y() + (max.y() - min.y()) / 2.0))
    }

    fn is_inside<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox2D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Ok(
                   minthis.x() > minother.x()
                && minthis.y() > minother.y()
                && maxthis.x() < maxother.x()
                && maxthis.y() < maxother.y()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    fn contains<P>(&self, other: &P) -> Result<bool> where
        Self: Sized, P: Is2D {

        if let Ok(bbthis) = self.bounding_box() {
            let (minthis, maxthis) = bbthis;

            return Ok(
                   other.x() > minthis.x()
                && other.x() < maxthis.x()
                && other.y() > minthis.y()
                && other.y() < maxthis.y()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    fn has_inside<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox2D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Ok(
                   minthis.x() < minother.x()
                && minthis.y() < minother.y()
                && maxthis.x() > maxother.x()
                && maxthis.y() > maxother.y()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    fn collides_with<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox2D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            let (xsizethis, ysizethis) = (
                (minthis.x() - maxthis.x()).abs(),
                (minthis.y() - maxthis.y()).abs());

            let (xsizeother, ysizeother) = (
                (minother.x() - maxother.x()).abs(),
                (minother.y() - maxother.y()).abs());

            let (xcenterthis, ycenterthis) = (
                (minthis.x() + maxthis.x() / 2.0),
                (minthis.y() + maxthis.y() / 2.0));

            let (xcenterother, ycenterother) = (
                (minother.x() + maxother.x() / 2.0),
                (minother.y() + maxother.y() / 2.0));

            return Ok(
                   2.0 * xcenterthis - xcenterother < (xsizethis + xsizeother)
                && 2.0 * ycenterthis - ycenterother < (ysizethis + ysizeother)
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }
}
