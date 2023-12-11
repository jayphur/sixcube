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

pub trait Map<R>
where
    R: Runtime,
    Self: Sized,
{    
    fn get_type(&self, pos: Pos) -> Option<R::VoxelType>;
    fn msg_voxel(&self, pos: Pos, msg: VoxelMsg<R>);
    fn load(&mut self, pos: &[Pos]);

    /// Iter LOADED chunks.
    /// Isolated to chunk and will drain it's message queue and mutate itself.
    fn update<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, R, Self>;
}

pub trait Visitor<R, M> 
where R: Runtime, M: Map<R> 
{
    fn predicate<'a>(&'a self) -> VisitingPredicate<'a,R>;

    fn visit(&self, pos: Pos, vox: &Voxel<R>, map: &M);
}

pub trait VisitorRegistry<'i, R, M>: Sized + Send + Sync
where
R: Runtime,
M: Map<R>
{
    type VisitorList<'a>: Iterator<Item=Self::Visitor>;
    type Visitor: Visitor<R, M>;

    fn get_visitor<'b>(&self, ids: &[u16])    -> Self::VisitorList<'b>;

    fn make_list<'a>(&self, info: Predicates<'a, R>) -> &'i [u16];

    //TODO: methods for adding visitors?? ACTUALLY, maybe not, that might not be of concern for this layer...
}

#[derive(Default, Debug, Clone)]
pub struct Predicates<'a, R: Runtime>{
    pub contains_voxels: Option<&'a Vec<R::VoxelType>>,
}

#[derive(Debug)]
pub struct VisitingPredicate<'a, R: Runtime>{
    attr: &'a [R::AttrType],
}