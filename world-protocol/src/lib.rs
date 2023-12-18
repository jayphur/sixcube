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

use std::path::Path;

use async_trait::async_trait;
use core_obj::*;
use prelude::*;

#[async_trait]
pub trait Map<R>
where
    R: Runtime,
    Self: Sized,
{    
    type UpdateListener: UpdateListener<R>;
    fn new_listener(&self) -> Self::UpdateListener;
    ///Read/write to existing file/make a new one
    async fn init(path: &Path, runtime: &R) -> Result<Self>;
    async fn get_type(&self, pos: Pos, runtime: &R) -> Option<R::VoxelType>;

    /// Iter LOADED chunks.
    /// Isolated to chunk and will drain it's message queue and mutate itself.
    async fn update<'v, V>(&mut self, registry: &V, runtime: &R) where V: VisitorRegistry<'v, R, Self>;
}

#[async_trait]
pub trait UpdateListener<R: Runtime>{
    async fn rx_async(&mut self) -> Result<Update<R>>;
    fn try_rx(&mut self) -> Result<Option<Update<R>>>;
    fn rx_blocking(&mut self) -> Result<Update<R>>;
}

#[derive(Debug, Clone, Copy)]
pub struct Update<R: Runtime>{
    pub pos: Pos,
    pub voxel_type: R::VoxelType,
}

pub trait Visitor<R, M> 
where R: Runtime, M: Map<R> 
{
    fn predicate<'a>(&'a self) -> VisitingPredicate<'a,R>;
    fn visit<'a, V: VoxelMut<'a, R>>(&self, pos: Pos, voxel_mut: V);
}
pub trait VoxelMut<'a, R: Runtime>{
    fn get_my_type(&self) -> &R::VoxelType;
    fn set_my_type(&mut self, val: R::VoxelType);
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