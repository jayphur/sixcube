use crate::{obj::{dim::Dim, voxel::Voxel}, pos::GlobalPos, Seed};
use sc_prelude::*;

/// Includes a dimension.
#[derive(Debug)]
pub struct Map<M: MapData<Dim>>{
    data: M,
    dim: Dim,
}
impl<M: MapData<Dim>> Map<M>{
    fn set_seed(&mut self, seed: Seed){
        self.data.set_seed(seed);
    }
    fn get(&self, pos: GlobalPos) -> Option<&Voxel> {
        self.data.get(pos)
    }
    fn get_mut(&mut self, pos: GlobalPos) -> &mut Option<Voxel> {
        self.data.get_mut(pos, &self.dim)
    }
    fn get_mut_weak(&mut self, pos: GlobalPos) -> Option<&mut Voxel> {
        self.data.get_mut_weak(pos, &self.dim)
    }
}
/// A map stores the voxels/chunks(?) in a dimension. 
/// This is the data structure that holds the voxels. 
pub trait MapData<D: DimTrait>: Debug{
    fn set_seed(&mut self, seed: Seed);
    fn get(&self, pos: GlobalPos) -> Option<&Voxel>;
    fn get_mut(&mut self, pos: GlobalPos, dim: &D) -> &mut Option<Voxel>;
    fn get_mut_weak(&mut self, pos: GlobalPos, dim: &D) -> Option<&mut Voxel>;
}
pub trait DimTrait: Debug{
    fn gen_at(&self, seed: Seed, pos: GlobalPos) -> Result<Voxel>;
}