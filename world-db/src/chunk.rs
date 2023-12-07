

use core_obj::{Pos, Voxel};
use world_protocol::{message::VoxelMsg, VisitorRead, VisitorRespond, VisitorApply, VisitorRegistry};

use crate::{CHUNK_SIZE, LocalPos};

#[derive(Debug, Clone)]
pub(crate) struct Chunk<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    voxels: [[[Option<Vox>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    contains_attr: Vec<  <Vox as Voxel>::AttrType  >, // a mess bro
    visitors_needed: Vec<u16>,
    messages: (flume::Sender<(LocalPos,VoxelMsg<Vox>)>, flume::Receiver<(LocalPos,VoxelMsg<Vox>)>),
    cw_pos: Pos,
}

impl<Vox> super::ChunkTrait<Vox> for Chunk<Vox> 
where 
Vox: core_obj::Voxel + Send + Sync, 
{
    fn get_type(&self, pos: LocalPos) -> Option<<Vox as Voxel>::Type> {
        todo!()
    }
    fn tell(&self, pos: LocalPos, msg: VoxelMsg<Vox>) {
        todo!()
    }


    fn new(cw_pos: Pos) -> Self {
        prelude::lazy_static! {
            static ref ALL_POS: Vec<Pos> = {
                let size = CHUNK_SIZE as i32;
                let mut vec: Vec<_> = Vec::with_capacity(CHUNK_SIZE.pow(3) as usize);
                for x in 0..size {
                    for y in 0..size {
                        for z in 0..size {
                            vec.push(Pos::new(x, y, z));
                        }
                    }
                }
                vec
            };
        }
        let arr_1d: [_; CHUNK_SIZE] = std::array::from_fn(|_| None);
        let arr_2d: [[_; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_1d.clone());
        let arr_3d: [[[_; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] =
            std::array::from_fn(|_| arr_2d.clone());

        Self {
            voxels: arr_3d,
            contains_attr: Vec::new(),
            messages: flume::unbounded(),
            visitors_needed: Vec::new(),
            cw_pos,
        }
    }

    fn read_phase<'a, V>(&self, registry: &V, map: &crate::Map<Vox>) 
    where V: VisitorRegistry<'a, Vox,crate::Map<Vox>> 
    {
        registry.get_read(self.visitors_needed.as_slice())
            .for_each(|visitor|{
                for plane in &self.voxels{
                    for row in plane{
                        for vox in row{
                            if let Some(vox) = vox{
                                visitor.visit(self.cw_pos, vox, map);
                            }
                        }
                    }
                }
            })
    }

    fn respond_phase<'a, V>(&mut self, registry: &V) 
    where V: VisitorRegistry<'a, Vox,crate::Map<Vox>> 
    {
        todo!()
    }

    fn apply_phase<'a, V>(&mut self, registry: &V) 
    where V: VisitorRegistry<'a, Vox,crate::Map<Vox>> 
    {
        todo!()
    }
}

