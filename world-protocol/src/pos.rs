use serde::{Deserialize, Serialize};

use core_obj::Pos;

use crate::CHUNK_SIZE_I32;

/// Position based on chunks
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct ChunkPos(pub i16,pub i16,pub i16);

impl ChunkPos {
    fn pos(self, local: ChunkLocalPos) -> Pos{
        local.pos(self)
    }
}

/// Position Local to a chunk
#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
        ChunkLocalPos((pos.0 % CHUNK_SIZE_I32) as u8, (pos.1 % CHUNK_SIZE_I32) as u8,(pos.2 % CHUNK_SIZE_I32) as u8)
    )
}

#[cfg(test)]
mod test{

}