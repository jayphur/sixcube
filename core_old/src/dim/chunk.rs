use std::fmt::Debug;

use crate::{voxel::Voxel, LocalPos};

#[derive(Debug, Clone, Default)]
pub struct Chunk<const S: usize, const VOL: usize> {
    vox: Array3d<S, Voxel>,
}
impl<const S: usize, const VOL: usize> Chunk<S, VOL> {
    /// Solid as in just one voxel.
    fn is_solid(&self) -> bool {
        todo!()
    }
    /// No voxels aka just "air" voxel.
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn pos_1d_list() -> [&'static LocalPos; VOL] {
        todo!()
    }

    fn voxels_1d_list(&self) -> [(&'static LocalPos, &Voxel); VOL] {
        todo!()
    }
    fn voxels_mut_1d_list(&mut self) -> [(&'static LocalPos, &mut Voxel); VOL] {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Array3d<const S: usize, T: Debug>([[[T; S]; S]; S]);

impl<const S: usize, T: Default + Debug + Clone> Default for Array3d<S, T> {
    fn default() -> Self {
        let default = T::default();
        todo!()
    }
}
