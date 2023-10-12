use db_protocol::update::Message;
use prelude::*;
use core_obj::{TypeId, Data, Voxel, Pos, AttrId};

use crate::{ChunkTrait, CwPos, CHUNK_SIZE_I32};
use super::CHUNK_SIZE;

#[derive(Debug)]
pub(crate) struct Chunk<T: TypeId,D: Data,M: Message>{
    voxels: [[[Option<Voxel<T,D>>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    messages: Vec<M>, //TODO: actual message system
}
impl<T: TypeId,D: Data,M: Message> ChunkTrait<T,D,M> for Chunk<T,D,M>{
    fn contains_attr(&self, attr: T::AttrId) -> bool {
        todo!()
    }

    fn tell(&self, pos: Pos, msg: M) {
        todo!()
    }

    fn get(&self, pos: Pos) -> &Option<Voxel<T,D>> {
        &self.voxels[pos.x as usize][pos.y as usize][pos.z as usize]
    }
    /// Where cw_pos defines (0,0) of the voxel, the "top left back" (or whatever)
    fn iter_voxel<'a>(&'a self, cw_pos: CwPos) -> impl Iterator<Item=(&Option<Voxel<T,D>>, Pos)>
    where D: 'a, T: 'a {
        self.voxels.iter().flat_map(move |p|{
            p.iter().flat_map(move |p|{
                p.iter()
            })
        }).zip(all_pos::<CHUNK_SIZE_I32>()).map(move |(vox, pos)|{
            (vox, pos + cw_pos)
        })
    }

    fn iter_voxel_mut<'a>(&'a mut self, cw_pos: CwPos) -> impl Iterator<Item=(&mut Option<Voxel<T,D>>, Pos)>
    where D: 'a, T: 'a {
        self.voxels.iter_mut().flat_map(move |p|{
            p.iter_mut().flat_map(move |p|{
                p.iter_mut()
            })
        }).zip(all_pos::<CHUNK_SIZE_I32>()).map(move |(vox, pos)|{
            (vox, pos + cw_pos)
        })
    }

}
impl<T: TypeId,D: Data,M: Message> Default for Chunk<T,D,M>{
    fn default() -> Self {
        let arr_1d: [_; CHUNK_SIZE] = std::array::from_fn(|_| None);
        let arr_2d: [[_; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_1d.clone());
        let arr_3d: [[[_; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = std::array::from_fn(|_| arr_2d.clone());
        
        Self { 
            voxels: arr_3d,
            messages: Vec::new(), 
        }
    }
}

lazy_static! {
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

fn all_pos<const S: i32>() -> impl Iterator<Item=Pos>{
    (0..S).flat_map(|x|{
            (0..S).flat_map(move |y|{
                (0..S).map(move |z|{
                    Pos::new(x, y, z)
                })
            })
        })
}

#[cfg(test)]
mod test{
    use crate::{ChunkTrait, chunk::all_pos, CHUNK_SIZE_I32};

    use super::Chunk;
    use core_obj::{fake_types::*, Pos, Voxel};
    use db_protocol::update::fake_types::FakeMessage;
    #[test]
    fn get_and_set(){
        let mut chunk: Chunk<FakeTypeId, FakeData, FakeMessage> = Chunk::default();
        let the_pos = Pos::new(1, 2, 3);
        for (voxel, pos) 
            in chunk.iter_voxel_mut(Pos::new(0,0,0)){
                if pos == the_pos{
                    *voxel = Some(Voxel{
                        ..Default::default()
                    })
                }
        }
        assert!(chunk.get(the_pos).is_some())
    }
}