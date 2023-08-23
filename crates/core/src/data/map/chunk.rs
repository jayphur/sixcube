use ndarray::Array3;
use sc_prelude::*;

#[derive(Debug)]
pub(crate) struct Chunk<V: Default + Debug + Clone, const SIZE: usize> {
    voxels: Array3<V>,
}
impl<V: Default + Debug + Clone, const SIZE: usize> Default for Chunk<V, SIZE> {
    fn default() -> Self {
        Self {
            voxels: Array3::<V>::default((SIZE, SIZE, SIZE)),
        }
    }
}
