pub trait Point {
    fn set_x(&mut self, val: f64);
    fn set_y(&mut self, val: f64);
    fn set_z(&mut self, val: f64);

    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_z(&self) -> f64;
}

pub trait MoveAble {
    fn move_by(&mut self, x: f64, y: f64, z: f64);
}
