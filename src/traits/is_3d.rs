use traits::has_position_3d::HasPosition3D;

pub trait Is3D {
    fn x(&self) -> f64;

    fn y(&self) -> f64;

    fn z(&self) -> f64;

    fn clone(&self) -> Self;

    fn pos(&self) -> (f64, f64, f64) {
        ( self.x(), self.y(), self.z() )
    }

    fn dot<P>(&self, other: &P) -> f64 where
        P: Is3D {

        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<P,HP>(&self, other: &P) -> Box<HP> where
        P: Is3D,
        HP: HasPosition3D {

        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.z() * other.x() - self.x() * other.z();
        let z = self.x() * other.y() - self.y() * other.x();
        HP::build(x, y, z)
    }

    fn abs(&self) -> f64 {
        (self.x()).powi(2) + (self.y()).powi(2) + (self.z()).powi(2)
    }

    fn to_str(&self) -> String {
        let sx: String = self.x().to_string();
        let sy: String = self.y().to_string();
        let sz: String = self.z().to_string();

        sx + " " + &sy + " " + &sz
    }
}
