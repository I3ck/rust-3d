use traits::has_position_3d::HasPosition3D;
use point_3d::Point3D;

pub trait HasBoundingBox3D : HasPosition3D {
    fn bounding_box(&self) -> Option<(Point3D, Point3D)>;
    //@todo below methods can be implemented in here
    fn min_pos(&self) -> Option<(Point3D)>;
    fn max_pos(&self) -> Option<(Point3D)>;
    fn is_inside<B>(&self, other: &B) -> bool where B: HasBoundingBox3D;
    fn contains<P>(&self, other: &P) -> bool where P: HasPosition3D;
    fn contains_fully<B>(&self, other: &B) -> bool where B: HasBoundingBox3D;
    fn collides_with<B>(&self, other: &B) -> bool where B: HasBoundingBox3D;
}
