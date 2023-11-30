//! # Plan
//! There are several "systems" aka visitors. 
//! That specify a pool of chunks/voxels that they want to visit.
//! 
//! - The map is divided into chunks of arbitrary size. (In order to be able to fine tune what the best size is with experimentation.)
//! 
//! Each visitor will visit each chunk via mutex 
//! and whenever it want to know stuff about voxels outside its chunk,
//! it can send a message to query another voxel.
//! 
//! We'll do message (and read), respond phase and apply phase.
//! 
//! ## Consistence
//! I want the updating to be consistent, 
//! the old method of ensuring this (I could think of...) 
//! was the two step *message & respond phase* and *apply phase*.
//! 
//! Do I think this aligns with the proposed ECS-ish system, maybe, but it's pretty complicated.
//! 
//! The issue is not just with the order at which each visitor is present, but also matter within each visitor.

#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use message::VoxelMsg;
use prelude::*;

pub mod message;

pub trait Map<Vox>: Debug
where
    Vox: Voxel,
    Self: Sized,
{
    type Chunk: Chunk<Vox>;
    
    fn get_type(&self, pos: Pos) -> Option<Vox::Type>;
    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<Vox>);
    fn load(&mut self, pos: &[Pos]);

    fn iter_chunks<F>(&self, f: F)
    where F: Fn(&Self::Chunk);

    /// Iter LOADED chunks and each can send messages.
    /// Cannot mutate, can only send messages and look at data.
    fn read_phase<'v, V>(&self, visitors: &'v [V])
    where V: 'v + Send + Sync + VisitorRead<Vox,Self>;

    /// Iter LOADED chunks and each can respond to messages.
    /// Cannot mutate chunk data (only respond), isolated to chunk and its msg queue.
    fn respond_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + VisitorRespond<Vox, Self>;

    /// Iter LOADED chunks.
    /// Isolated to chunk and will drain it's message queue and mutate itself.
    fn apply_phase<'v, V>(&mut self, visitors: &'v [V])
    where V: 'v + Send + Sync + VisitorApply<Vox, Self>;
}

/// I want to specify that positions here are global.
pub trait Chunk<Vox: Voxel>{
    fn msg(&self, pos: Pos, msg: VoxelMsg<Vox>);
    // some utility "getters". Don't use these for iteration.
    fn get_pos(&self, pos: Pos) -> BoundsResult<&Option<Vox>>;
    fn get_pos_mut(&mut self, pos: Pos) -> BoundsResult<&mut Option<Vox>>;

    fn read_phase<M, V>(&self, visitor: V) 
    where M: Map<Vox>, V: VisitorRead<Vox, M>;

    fn respond_phase<M, V>(&mut self, visitor: V) 
    where M: Map<Vox>, V: VisitorRespond<Vox, M>;

    fn apply_phase<M, V>(&mut self, visitor: V) 
    where M: Map<Vox>, V: VisitorApply<Vox, M>;
}

#[derive(Debug, Clone, Copy)]
pub struct ForEachPredicate{
    pub exists: bool,
}

#[derive(Debug)]
pub enum BoundsResult<T>{
    Ok(T),
    OutOfBounds,
}

pub trait VisitorRead<Vox: Voxel, Map: crate::Map<Vox>> {
    fn predicate_attr(&self) -> &[Vox::AttrType];

    fn visit(&self, pos: Pos, vox: &Vox, map: &Map);
}
pub trait VisitorRespond<Vox: Voxel, Map: crate::Map<Vox>> {
    fn predicate_attr(&self) -> &[Vox::AttrType];

    fn visit<'a, I> (&'a self, pos: Pos, vox: &Vox, messages: I) 
    where Vox: 'a, I: Iterator<Item = &'a mut VoxelMsg<Vox>>;
}
pub trait VisitorApply<Vox: Voxel, Map: crate::Map<Vox>> {
    fn predicate_attr(&self) -> &[Vox::AttrType];

    fn visit<'a, I> (&'a self, pos: Pos, vox: &mut Vox, messages: I) 
    where Vox: 'a, I: Iterator<Item = &'a mut VoxelMsg<Vox>>{}
}