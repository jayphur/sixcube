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

//Note: we're trying to avoid that dreaded &dyn. it's stinky, silly, annoying, and hard to work with. 

#![feature(return_position_impl_trait_in_trait)]

use core_obj::*;
use message::VoxelMsg;
use prelude::*;

pub mod message;

pub trait Map<Vox>
where
    Vox: Voxel,
    Self: Sized,
{    
    fn get_type(&self, pos: Pos) -> Option<Vox::Type>;
    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<Vox>);
    fn load(&mut self, pos: &[Pos]);

    /// Iter LOADED chunks and each can send messages.
    /// Cannot mutate, can only send messages and look at data.
    fn read_phase<'v, V>(&self, registry: &V) where V: VisitorRegistry<'v, Vox, Self>;

    /// Iter LOADED chunks and each can respond to messages.
    /// Cannot mutate chunk data (only respond), isolated to chunk and its msg queue.
    fn respond_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, Vox, Self>;


    /// Iter LOADED chunks.
    /// Isolated to chunk and will drain it's message queue and mutate itself.
    fn apply_phase<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, Vox, Self>;
}
#[derive(Debug)]
pub enum BoundsResult<T>{
    Ok(T),
    OutOfBounds,
}


pub trait VisitorRead<Vox, M> 
where Vox: Voxel, M: Map<Vox> 
{
    fn predicate<'a>(&'a self) -> VisitingPredicate<'a,Vox>;

    fn visit(&self, pos: Pos, vox: &Vox, map: &M);
}

pub trait VisitorRespond<Vox, M> 
where Vox: Voxel, M: Map<Vox> 
{
    fn predicate<'a>(&'a self) -> VisitingPredicate<'a,Vox>;

    fn visit<'a, I> (&'a self, pos: Pos, vox: &Vox, messages: I) 
    where Vox: 'a, I: Iterator<Item = &'a mut VoxelMsg<Vox>>;
}

pub trait VisitorApply<Vox, M> 
where Vox: Voxel, M: Map<Vox> 
{
    fn predicate<'a>(&'a self) -> VisitingPredicate<'a,Vox>;

    fn visit<'a, I> (&'a self, pos: Pos, vox: &mut Vox, messages: I) 
    where Vox: 'a, I: Iterator<Item = &'a mut VoxelMsg<Vox>>{}
}

pub trait VisitorRegistry<'i, Vox, M>: Sized 
where
Vox: Voxel,
M: Map<Vox>
{
    type ReadList<'a>: Iterator<Item=Self::Read>;
    type Read: VisitorRead<Vox, M>;

    type RespondList<'a>: Iterator<Item=Self::Respond>;
    type Respond: VisitorRespond<Vox, M>;

    type ApplyList<'a>: Iterator<Item=Self::Apply>;
    type Apply: VisitorApply<Vox, M>;

    fn get_read<'b>(&self, ids: &[u16])    -> Self::ReadList<'b>;
    fn get_respond<'b>(&self, ids: &[u16]) -> Self::RespondList<'b>;
    fn get_apply<'b>(&self, ids: &[u16])   -> Self::ApplyList<'b>;

    fn make_list<'a>(&self, info: Info<'a, Vox>) -> &'i [u16];

    //TODO: methods for adding visitors?? ACTUALLY, maybe not, that might not be of concern for this layer...
}

#[derive(Default, Debug, Clone)]
pub struct Info<'a, Vox: Voxel>{ //FIXME: maybe get a better name here??
    pub contains_attr: Option<&'a Vec<Vox::AttrType>>,
}

#[derive(Debug)]
pub struct VisitingPredicate<'a, Vox: Voxel>{
    attr: &'a [Vox::AttrType],
}