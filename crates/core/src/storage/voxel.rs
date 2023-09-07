use serde::Serialize;

use crate::storage::error::StorageErr;
use crate::{obj::voxel::Voxel, types::TypePtrList};
use super::{WorldStorage, Package, Unpackage};
use sc_prelude::*;

impl Package<Voxel> for WorldStorage{
    type Packaged = PackagedVoxel;

    fn package(&mut self, this: Voxel) -> Self::Packaged {
        PackagedVoxel{
            data: None,
            type_id_index: self.voxel_type_ids.find_or_maybe_push(this.my_type.as_ref().type_id_name()),
        }
    }
}

#[derive(Serialize)]
pub struct PackagedVoxel{
    data: Option<String>,
    type_id_index: usize,
}

impl Unpackage<PackagedVoxel> for WorldStorage{
    type Output = Voxel;

    fn unpackage<'a>(&'a mut self, i: PackagedVoxel, list: &'a TypePtrList) -> Result<Self::Output> {
        let Some(name) = self.voxel_type_ids.get(i.type_id_index) else {
            return Err(StorageErr::TypeIdMissing.into());
        };
        let Some(type_ptr) = list.get_voxel(name) else {
            return Err(StorageErr::TypePtrMissing.into());
        };
        let mut instance = type_ptr.as_ref().instance();
        if let Some(data) = i.data{
            match instance.try_mut(){
                Some(mut_inst) => mut_inst.load_string(data),
                None => {
                    eprintln!("Ditching data \"{data}\" in Packaged voxel due to Voxel type ony supporting static instances",);
                },
            };
        }
        Ok(Voxel{
            my_type: instance,
        })        
    }
    
}