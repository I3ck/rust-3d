pub trait IsMoveable2D { //@todo remove trait and impl in IsBuildable2D
    fn move_by(&mut self, x: f64, y: f64);
}
