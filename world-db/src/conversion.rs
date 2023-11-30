use core_obj::Pos;
    use spatialtree::OctVec;

    use crate::{CHUNK_SIZE_I32, LocalPos};


const U32_MIDPOINT: u32 = 32768;

pub fn pos_to_oct_pos(pos: Pos) -> OctVec<u32>{
    OctVec::new([
        i32_to_u32(pos.x),
        i32_to_u32(pos.y),
        i32_to_u32(pos.z),
    ], 0)
}
pub fn oct_pos_to_pos(pos: OctVec<u32>) -> Pos{
    Pos::new(u32_to_i32(pos.pos[0]), u32_to_i32(pos.pos[1]), u32_to_i32(pos.pos[2]))
}
pub fn i32_to_u32(val: i32) -> u32{
    if val.is_negative(){
        val.abs() as u32 
    } else {
        val as u32 + U32_MIDPOINT
    }
}
pub fn pos_to_local_pos(pos: Pos) -> LocalPos{
    LocalPos::new((pos.x%CHUNK_SIZE_I32) as usize, (pos.y%CHUNK_SIZE_I32) as usize, (pos.z%CHUNK_SIZE_I32) as usize)
}

pub fn u32_to_i32(val: u32) -> i32{
    use std::cmp::Ordering;
    match val.cmp(&U32_MIDPOINT){
        Ordering::Less => {
            -(val as i32)
        },
        Ordering::Equal => {
            0
        },
        Ordering::Greater => {
            (val - U32_MIDPOINT) as i32
        },
    }
}

#[cfg(test)]
mod test_conv{

#[test]
fn there_and_back(){
    let start: i32 = 31785;
    assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));

    let start: i32 = -3185;
    assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));

    let start: i32 = -3;
    assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));
    
    let start: i32 = 0;
    assert_eq!(start, super::u32_to_i32(super::i32_to_u32(start)));
}
}
