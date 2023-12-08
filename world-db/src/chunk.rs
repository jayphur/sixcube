

use core_obj::{Pos, Voxel, Runtime};
use world_protocol::{message::VoxelMsg, VisitorRead, VisitorRespond, VisitorApply, VisitorRegistry};

use crate::{CHUNK_SIZE, LocalPos};

#[derive(Debug, Clone)]
pub(crate) struct Chunk<R> 
where 
R: Runtime, 
{
    voxels: [[[Option<Voxel<R>>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    contains_attr: Vec< R::AttrType  >, // a mess bro
    visitors_needed: Vec<u16>,

    cw_pos: Pos,
}

impl<R> super::ChunkTrait<R> for Chunk<R> 
where 
R: Runtime, 
{


    fn read_phase<'a, V>(&self, registry: &V, map: &crate::Map<R>) 
    where V: VisitorRegistry<'a, R, crate::Map<R>> 
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
    where V: VisitorRegistry<'a, R, crate::Map<R>> 
    {
        todo!()
    }

    fn apply_phase<'a, V>(&mut self, registry: &V) 
    where V: VisitorRegistry<'a, R, crate::Map<R>> 
    {
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
                            vec.push(Pos(x, y, z));
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
            visitors_needed: Vec::new(),
            cw_pos,
        }
    }

    fn get_type(&self, pos: LocalPos) -> Option<R::VoxelType> {
        todo!()
    }

    fn tell(&self, pos: LocalPos, msg: VoxelMsg<R>) {
        todo!()
    }
}

