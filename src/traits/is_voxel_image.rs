pub trait IsVoxelImage<T> {
    fn size_x(&self) -> usize;
    fn size_y(&self) -> usize;
    fn size_z(&self) -> usize;

    fn voxel(&self, x: usize, y: usize, z: usize) -> Option<T>;
}
