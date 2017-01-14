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

use traits::is_3d::Is3D;
use traits::is_buildable_3d::IsBuildable3D;
use point_3d::Point3D;

pub trait HasBoundingBox3D  {
    fn bounding_box(&self) -> Option<(Point3D, Point3D)>;

    fn min_pos(&self) -> Option<Point3D> {
        match self.bounding_box() {
            None => None,
            Some((min, _)) => Some(min)
        }
    }

    fn max_pos(&self) -> Option<Point3D> {
        match self.bounding_box() {
            None => None,
            Some((_, max)) => Some(max)
        }
    }

    fn size_x(&self) -> Option<f64> {
        match self.bounding_box() {
            None => None,
            Some((min, max)) => Some((max.x() - min.x()).abs())
        }
    }

    fn size_y(&self) -> Option<f64> {
        match self.bounding_box() {
            None => None,
            Some((min, max)) => Some((max.y() - min.y()).abs())
        }
    }

    fn size_z(&self) -> Option<f64> {
        match self.bounding_box() {
            None => None,
            Some((min, max)) => Some((max.z() - min.z()).abs())
        }
    }

    fn center_bb(&self) -> Option<Point3D> {
        match self.bounding_box() {
            None => None,
            Some((min, max)) => Some(*Point3D::build(min.x() + (max.x() - min.x()) / 2.0,
                                                     min.y() + (max.y() - min.y()) / 2.0,
                                                     min.z() + (max.z() - min.z()) / 2.0))
        }
    }


    fn is_inside<B>(&self, other: &B) -> Option<bool> where
        Self: Sized, B: HasBoundingBox3D {

        if let (Some(bbthis), Some(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Some(
                   minthis.x() > minother.x()
                && minthis.y() > minother.y()
                && minthis.z() > minother.z()
                && maxthis.x() < maxother.x()
                && maxthis.y() < maxother.y()
                && maxthis.z() < maxother.z()
            );
        }
        None
    }

    fn contains<P>(&self, other: &P) -> Option<bool> where
        Self: Sized, P: Is3D {

        if let Some(bbthis) = self.bounding_box() {
            let (minthis, maxthis) = bbthis;

            return Some(
                   other.x() > minthis.x()
                && other.x() < maxthis.x()
                && other.y() > minthis.y()
                && other.y() < maxthis.y()
                && other.z() > minthis.z()
                && other.z() < maxthis.z()
            );
        }
        None
    }

    fn has_inside<B>(&self, other: &B) -> Option<bool> where
        Self: Sized, B: HasBoundingBox3D {

        if let (Some(bbthis), Some(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Some(
                   minthis.x() < minother.x()
                && minthis.y() < minother.y()
                && minthis.z() < minother.z()
                && maxthis.x() > maxother.x()
                && maxthis.y() > maxother.y()
                && maxthis.z() > maxother.z()
            );
        }
        None
    }

    fn collides_with<B>(&self, other: &B) -> Option<bool> where
        Self: Sized, B: HasBoundingBox3D {

        if let (Some(bbthis), Some(bbother)) = (self.bounding_box(), other.bounding_box()) {
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

            return Some(
                   2.0 * xcenterthis - xcenterother < (xsizethis + xsizeother)
                && 2.0 * ycenterthis - ycenterother < (ysizethis + ysizeother)
                && 2.0 * zcenterthis - zcenterother < (zsizethis + zsizeother)
            );
        }
        None
    }
}
