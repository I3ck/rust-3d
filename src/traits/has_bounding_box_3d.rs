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

use result::*;
use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use point_3d::Point3D;

pub trait HasBoundingBox3D  {
    fn bounding_box(&self) -> Result<(Point3D, Point3D)>;

    fn min_pos(&self) -> Result<Point3D> {
        let (min,_) = try!(self.bounding_box());
        Ok(min)
    }

    fn max_pos(&self) -> Result<Point3D> {
        let (_,max) = try!(self.bounding_box());
        Ok(max)
    }

    fn size_x(&self) -> Result<f64> {
        let (min, max) = try!(self.bounding_box());
        Ok((max.x() - min.x()).abs())
    }

    fn size_y(&self) -> Result<f64> {
        let (min, max) = try!(self.bounding_box());
        Ok((max.y() - min.y()).abs())
    }

    fn size_z(&self) -> Result<f64> {
        let (min, max) = try!(self.bounding_box());
        Ok((max.z() - min.z()).abs())
    }

    fn center_bb(&self) -> Result<Point3D> {
        let (min, max) = try!(self.bounding_box());
        Ok(*Point3D::build(min.x() + (max.x() - min.x()) / 2.0,
                           min.y() + (max.y() - min.y()) / 2.0,
                           min.z() + (max.z() - min.z()) / 2.0))
    }

    fn is_inside<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox3D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Ok(
                   minthis.x() > minother.x()
                && minthis.y() > minother.y()
                && minthis.z() > minother.z()
                && maxthis.x() < maxother.x()
                && maxthis.y() < maxother.y()
                && maxthis.z() < maxother.z()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    fn contains<P>(&self, other: &P) -> Result<bool> where
        Self: Sized, P: Is3D {

        if let Ok(bbthis) = self.bounding_box() {
            let (minthis, maxthis) = bbthis;

            return Ok(
                   other.x() > minthis.x()
                && other.x() < maxthis.x()
                && other.y() > minthis.y()
                && other.y() < maxthis.y()
                && other.z() > minthis.z()
                && other.z() < maxthis.z()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    fn has_inside<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox3D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Ok(
                   minthis.x() < minother.x()
                && minthis.y() < minother.y()
                && minthis.z() < minother.z()
                && maxthis.x() > maxother.x()
                && maxthis.y() > maxother.y()
                && maxthis.z() > maxother.z()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    fn collides_with<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox3D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            let (xsizethis, ysizethis, zsizethis) = (
                (minthis.x() - maxthis.x()).abs(),
                (minthis.y() - maxthis.y()).abs(),
                (minthis.z() - maxthis.z()).abs());

            let (xsizeother, ysizeother, zsizeother) = (
                (minother.x() - maxother.x()).abs(),
                (minother.y() - maxother.y()).abs(),
                (minother.z() - maxother.z()).abs());

            let (xcenterthis, ycenterthis, zcenterthis) = (
                (minthis.x() + maxthis.x() / 2.0),
                (minthis.y() + maxthis.y() / 2.0),
                (minthis.z() + maxthis.z() / 2.0));

            let (xcenterother, ycenterother, zcenterother) = (
                (minother.x() + maxother.x() / 2.0),
                (minother.y() + maxother.y() / 2.0),
                (minother.z() + maxother.z() / 2.0));

            return Ok(
                   2.0 * xcenterthis - xcenterother < (xsizethis + xsizeother)
                && 2.0 * ycenterthis - ycenterother < (ysizethis + ysizeother)
                && 2.0 * zcenterthis - zcenterother < (zsizethis + zsizeother)
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }
}
