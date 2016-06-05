use traits::has_position_2d::HasPosition2D;

pub trait Is2D {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn clone(&self) -> Self;

    fn pos(&self) -> (f64, f64) {
        ( self.x(), self.y() )
    }

    fn dot<P>(&self, other: &P) -> f64 where P: HasPosition2D {
        self.x() * other.x() + self.y() * other.y()
    }

    fn cross<P>(&self, other: &P) -> f64 where P: HasPosition2D {
        self.x() * other.y() - self.y() * other.x()
    }

    fn abs(&self) -> f64 {
        (self.x()).powi(2) + (self.y()).powi(2)
    }

    fn rad_to<P>(&self, other: &P) -> f64 where P: HasPosition2D {
        (other.y() - self.y()).atan2(other.x() - self.x())
    }

    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();

        sx + " " + &sy
    }
}
