use core_obj::{Pos, Runtime};
use parking_lot::{RwLock, RwLockWriteGuard};
use rustc_hash::FxHashMap;
use world_protocol::{Visitor, VisitorRegistry, VoxelMut};

use crate::{CHUNK_SIZE, Pos16, Map, CHUNK_SIZE_I32};

#[derive(Debug)]
pub(crate) struct Chunk<R> 
where 
R: Runtime, 
{
    pub cw_pos: Pos16,
    pub data: RwLock<ChunkData<R>>,
    pub msg_tx: flume::Sender<ChunkMsg>
}
#[derive(Debug, Clone)]

pub struct ChunkData<R:Runtime>{ //TODO: special case for empty chunk?
    msg_rx: flume::Receiver<ChunkMsg>,
    voxels: [[[Option<StoredVoxel<R>>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    data: FxHashMap<LocalPos, ()>, //TODO: () is the data stored in the voxel
    visitors_list: Vec<u16>, //TODO: Nothing fills this atm
}
#[derive(Debug, Default, Clone, Copy)]
struct StoredVoxel<R:Runtime>{
        type_id: R::VoxelType,
        has_data: bool,
    
}

impl<R> Chunk<R> 
where 
R: Runtime, 
{
    pub fn update<'a, 'v, V>(&self, registry: &V, mut data: RwLockWriteGuard<'a, ChunkData<R>>, map: &Map<R>) 
    where V: VisitorRegistry<'v, R, crate::Map<R>> 
    {
        for visitor in registry.get_visitor(data.visitors_list.as_slice()){
            let pos16 = self.cw_pos;
            let origin = (pos16.0 as i32 *CHUNK_SIZE_I32,pos16.1 as i32 *CHUNK_SIZE_I32,pos16.2 as i32 *CHUNK_SIZE_I32);
            for pos_usize in all_pos::<usize>(){
                let pos = Pos::from_usize(pos_usize);
                let pos = Pos(origin.0 + pos.0,origin.1 + pos.1,origin.2 + pos.2);
                let Some(stored_vox) = &mut data.voxels[pos_usize.2][pos_usize.1][pos_usize.0] else {continue;};
                let vox = VoxelMut{
                    my_type: &mut stored_vox.type_id,
                };
                visitor.visit(
                        pos, 
                        vox, 
                        |pos|{
                            todo!()
                        }, 
                        |pos|{
                            todo!()
                        }
                    );
            }
        }
    }

    pub  fn new(cw_pos: Pos) -> Self {
        todo!()
    }

    pub fn get_type(&self, pos: crate::LocalPos) -> Option<R::VoxelType> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalPos{

}

#[derive(Debug)]
pub enum ChunkMsg{

}

fn all_pos<T>() -> impl Iterator<Item=(T,T,T)>
where
T: From<usize>
{
    (0..CHUNK_SIZE).into_iter()
        .flat_map(|x|{
            (0..CHUNK_SIZE)
                .into_iter()
                .flat_map(move |y|{
                    (0..CHUNK_SIZE)
                        .into_iter()
                        .map(move |z|{
                            (T::from(x),T::from(y),T::from(z))
                        })
                })
        })
}