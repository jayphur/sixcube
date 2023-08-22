use crate::{obj::voxel::{Voxel, VoxelStaticType, VoxelDynType}, r#type::StaticType};

//TODO: move a lot of this enum's functionality to a new generic Type<Obj>. (Type is using a associated type rn)

#[derive(Debug)]
pub enum VoxelTypePtr{ 
    Static(&'static dyn VoxelStaticType),
    Dyn(Box<dyn VoxelDynType>),
}
impl VoxelStaticType for VoxelTypePtr{

}
impl StaticType for VoxelTypePtr{
    type Obj = Voxel;

    fn name(&'static self) -> &crate::Name {
        todo!()
    }

    fn new_obj<'a>(&'a self) -> Self::Obj {
        Voxel{
            my_type: self.clone()
        }
    }
}
impl Clone for VoxelTypePtr{
    fn clone(&self) -> Self {
        #[allow(suspicious_double_ref_op)]
        match self {
            Self::Static(ptr) => {
                Self::Static(ptr.clone()) // suspicious_double_ref_op!! (i think its fine)
            },
            Self::Dyn(t) => {
                Self::Dyn(t.clone_to_box())
            },
        } 
    }
}