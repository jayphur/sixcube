use std::fmt::Debug;

pub type Pos = vector3d::Vector3d<i32>;


#[derive(Debug, Default, Clone)]
pub struct Voxel<T, D>
where
T: TypeId, 
D: Data
{
    pub type_id: T,
    pub data: D,
}



#[derive(Debug, Default, Clone)]
pub struct Dim<T, D, Map>
where
T: TypeId, 
D: Data,
Map:,
{
    pub type_id: T,
    pub data: D,
    pub map: Map,
}


#[derive(Debug, Default, Clone)]
pub struct World<Dim>{
    pub dims: Vec<Dim>,
}

/// Assuming there is no mixing type id of different objects by faith (and checking)
pub trait TypeId: PartialEq + Copy + Clone + Debug + Send + Sync{
    type AttrId: AttrId;
    fn my_obj(&self) -> &ObjStruct;
}
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum ObjStruct{
    Voxel,
    Dim,
}

pub trait ActionId: PartialEq + Copy + Clone + Send + Sync{

}

pub trait AttrId: PartialEq + Copy + Clone{
    fn default_inner(&self) -> Attr<Self>;
}

pub struct Action<Id: ActionId>{
    id: Id,
    args: ActionArgs,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ActionArgs{
    #[default]
    Unset,
    Boolean(bool),
    U8(u8),
    String(String),
}

#[derive(Debug, Default, Clone)]
pub struct Attr<Id: AttrId>{
    my_type: Id,
    val: AttrValue
}


#[derive(Debug, Default, Clone, PartialEq)]
pub enum AttrValue{
    #[default]
    Unset,
    Boolean(bool),
    U8(u8),
    String(String),
}

pub trait Data: Default + Debug + Clone + Send + Sync{

}


/// Protocol for defining types 
pub trait TypeDefiner<T: TypeId>{
    /// The voxels this struct defined.
    fn voxels_def(&self) -> &[T];
    /// The dims this struct defined.
    fn dim_def(&self) -> &[T];
}

/// Protocol for defining attributes 
pub trait AttrDefiner<A: AttrId>{
    /// The voxels this struct defined.
    fn attr_def(&self) -> &[A];
}

pub mod fake_types{
    use crate::{TypeId, AttrId, Data, ActionId};

    #[derive(Default,Debug,PartialEq, Eq, Clone, Copy)]
    pub struct FakeAttr(u8);
    impl AttrId for FakeAttr{
        fn default_inner(&self) -> crate::Attr<Self> {
            todo!()
        }
    }

    #[derive(Default,Debug,PartialEq, Eq, Clone, Copy)]
    pub struct FakeTypeId(u8);
    impl TypeId for FakeTypeId{
        
        fn my_obj(&self) -> &crate::ObjStruct {
            todo!()
        }

        type AttrId = FakeAttr;

    }

    #[derive(Default,Debug,PartialEq, Eq, Clone, Copy)]
    pub struct FakeData(u8);
    impl Data for FakeData{
        
    }

    #[derive(Default,Debug,PartialEq, Eq, Clone, Copy)]
    pub struct FakeAction(u8);
    impl ActionId for FakeAction{

    }
}