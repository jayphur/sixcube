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

use std::marker::PhantomData;

use core_obj::*;
use prelude::*;

pub trait Map<R>
where
    R: Runtime,
    Self: Sized,
{    
    fn get_type(&self, pos: Pos) -> Option<R::VoxelType>;
    fn load(&mut self, pos: &[Pos]);

    /// Iter LOADED chunks.
    /// Isolated to chunk and will drain it's message queue and mutate itself.
    fn update<'v, V>(&mut self, registry: &V) where V: VisitorRegistry<'v, R, Self>;
}

pub trait Visitor<R, M> 
where R: Runtime, M: Map<R> 
{
    fn predicate<'a>(&'a self) -> VisitingPredicate<'a,R>;
    fn visit<'a, GetAttr,GetType>(&self, pos: Pos, vox: VoxelMut<'a, R>, get_attr: GetAttr, get_type: GetType)
    where
    GetAttr: Fn(Pos) -> Option<core_obj::Value>,
    GetType: Fn(Pos) -> Option<R::VoxelType>;
}
pub struct VoxelMut<'a, R: Runtime>{
    pub my_type: &'a mut R::VoxelType,
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