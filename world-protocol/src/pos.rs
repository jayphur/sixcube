use core_obj::Pos;

use crate::CHUNK_SIZE_I32;

#[derive(Debug, Default, Clone, Copy, Hash)]
pub struct ChunkPos(pub i16,pub i16,pub i16);

impl ChunkPos {
    fn pos(self, local: ChunkLocalPos) -> Pos{
        local.pos(self)
    }
}

#[derive(Debug, Default, Clone, Copy, Hash)]
pub struct ChunkLocalPos(pub u8,pub u8,pub u8);

impl ChunkLocalPos {
    fn pos(self, chunk: ChunkPos) -> Pos{
        Pos(self.0 as i32,self.1 as i32,self.2 as i32)
        + Pos(self.0 as i32 * CHUNK_SIZE_I32,self.1 as i32 * CHUNK_SIZE_I32,self.2 as i32 * CHUNK_SIZE_I32)
    }
} // -128 -> 128

pub fn split(pos: Pos) -> (ChunkPos, ChunkLocalPos){
    (
        ChunkPos((pos.0 / CHUNK_SIZE_I32) as i16,(pos.1 / CHUNK_SIZE_I32) as i16,(pos.2 / CHUNK_SIZE_I32) as i16),
        ChunkLocalPos((pos.0 % CHUNK_SIZE_I32) as i8, (pos.1 % CHUNK_SIZE_I32) as i8,(pos.2 % CHUNK_SIZE_I32) as i8)
    )
}

#[cfg(test)]
mod test{

}