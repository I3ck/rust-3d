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

//! Modulecontaining the HasBoundingBox2D trait for types which might have a bounding box

use result::*;
use traits::is_2d::*;
use traits::is_buildable_2d::*;
use point_2d::*;
use bounding_box_2d::*;
use positive::*;

/// HasBoundingBox2D is a trait for types which might have a bounding box
pub trait HasBoundingBox2D {
    /// Should return the bounding box as a pair of two points. The first point should be the minimum for all coordinates, the second the maximum for all coordinates
    fn bounding_box(&self) -> Result<BoundingBox2D>;

    /// Returns the minimum position of the bounding box
    fn min_pos(&self) -> Result<Point2D> {
        let bb = self.bounding_box()?;
        Ok(bb.min)
    }

    /// Returns the maximum position of the bounding box
    fn max_pos(&self) -> Result<Point2D> {
        let bb = self.bounding_box()?;
        Ok(bb.max)
    }

    /// Returns the size the bounding box within the x-dimension
    fn size_x(&self) -> Result<Positive> {
        let bb = self.bounding_box()?;
        Positive::new((bb.max.x() - bb.min.x()).abs())
    }

    /// Returns the size the bounding box within the y-dimension
    fn size_y(&self) -> Result<Positive> {
        let bb = self.bounding_box()?;
        Positive::new((bb.max.y() - bb.min.y()).abs())
    }

    /// Returns the center of the bounding box
    fn center_bb(&self) -> Result<Point2D> {
        let bb = self.bounding_box()?;
        Ok(*Point2D::build(bb.min.x() + (bb.max.x() - bb.min.x()) / 2.0,
                           bb.min.y() + (bb.max.y() - bb.min.y()) / 2.0))
    }

    /// Tests whether this bounding box is within the other
    fn is_inside<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox2D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = (bbthis.min, bbthis.max);
            let (minother, maxother) = (bbother.min, bbother.max);

            return Ok(
                   minthis.x() > minother.x()
                && minthis.y() > minother.y()
                && maxthis.x() < maxother.x()
                && maxthis.y() < maxother.y()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    /// Tests whether this bounding box contains a position
    fn contains<P>(&self, other: &P) -> Result<bool> where
        Self: Sized, P: Is2D {

        if let Ok(bbthis) = self.bounding_box() {
            let (minthis, maxthis) = (bbthis.min, bbthis.max);

            return Ok(
                   other.x() > minthis.x()
                && other.x() < maxthis.x()
                && other.y() > minthis.y()
                && other.y() < maxthis.y()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    /// Tests whether this bounding box contains the other
    fn has_inside<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox2D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = (bbthis.min, bbthis.max);
            let (minother, maxother) = (bbother.min, bbother.max);

            return Ok(
                   minthis.x() < minother.x()
                && minthis.y() < minother.y()
                && maxthis.x() > maxother.x()
                && maxthis.y() > maxother.y()
            );
        }
        Err(ErrorKind::BoundingBoxMissing)
    }

    /// Tests whether this bounding box and the other overlap in any way
    fn collides_with<B>(&self, other: &B) -> Result<bool> where
        Self: Sized, B: HasBoundingBox2D {

        if let (Ok(bbthis), Ok(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = (bbthis.min, bbthis.max);
            let (minother, maxother) = (bbother.min, bbother.max);

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
