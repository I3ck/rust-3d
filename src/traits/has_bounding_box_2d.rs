use traits::is_2d::Is2D;
use point_2d::Point2D;

pub trait HasBoundingBox2D : Is2D {
    fn bounding_box(&self) -> Option<(Point2D, Point2D)>;

    fn min_pos(&self) -> Option<(Point2D)> {
        match self.bounding_box() {
            None => None,
            Some((min, _)) => Some(min)
        }
    }

    fn max_pos(&self) -> Option<(Point2D)> {
        match self.bounding_box() {
            None => None,
            Some((_, max)) => Some(max)
        }
    }

    fn is_inside<B>(&self, other: &B) -> Option<bool> where
        B: HasBoundingBox2D {

        if let (Some(bbthis), Some(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Some(
                   minthis.x() > minother.x()
                && minthis.y() > minother.y()
                && maxthis.x() < maxother.x()
                && maxthis.y() < maxother.y()
            );
        }
        None
    }

    fn contains<P>(&self, other: &P) -> Option<bool> where
        P: Is2D {

        if let Some(bbthis) = self.bounding_box() {
            let (minthis, maxthis) = bbthis;

            return Some(
                   other.x() > minthis.x()
                && other.x() < maxthis.x()
                && other.y() > minthis.y()
                && other.y() < maxthis.y()
            );
        }
        None
    }

    fn has_inside<B>(&self, other: &B) -> Option<bool> where
        B: HasBoundingBox2D {

        if let (Some(bbthis), Some(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            return Some(
                   minthis.x() < minother.x()
                && minthis.y() < minother.y()
                && maxthis.x() > maxother.x()
                && maxthis.y() > maxother.y()
            );
        }
        None
    }

    fn collides_with<B>(&self, other: &B) -> Option<bool> where
        B: HasBoundingBox2D {

        if let (Some(bbthis), Some(bbother)) = (self.bounding_box(), other.bounding_box()) {
            let (minthis, maxthis) = bbthis;
            let (minother, maxother) = bbother;

            let (xsizethis, ysizethis) = (
                (minthis.x() - maxthis.x()).abs(),
                (minthis.y() - maxthis.y()).abs());

            let (xsizeother, ysizeother) = (
                (minother.x() - maxother.x()).abs(),
                (minother.y() - maxother.y()).abs());

            return Some(
                   2.0 * self.x() - other.x() < (xsizethis + xsizeother)
                && 2.0 * self.y() - other.y() < (ysizethis + ysizeother)
            );
        }
        None
    }
}
