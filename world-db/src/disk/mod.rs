use std::{marker::PhantomData, path::Path, sync::Arc};
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::io::ErrorKind;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use itertools::Itertools;
use rustc_hash::FxHashMap;
use serde::Serialize;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::task::JoinSet;
use tokio_stream::{Stream, StreamExt};

use core_obj::fake::FakeRegistrar;
use core_obj::Registrar;
use prelude::*;
use world_protocol::pos::ChunkPos;

use crate::chunk::{ChunkData, SmallerChunk};
use crate::disk::region::RegionFile;
use crate::PosU;

pub mod rle;
mod region;

const REGION_DIRECTORY_NAME: &str = "regions";

pub struct MapFile<R: Registrar, P: AsRef<Path>>{
    pub(crate) path: P,
    pub(crate) region_dir: Arc<PathBuf>,
    __marker: PhantomData<R>, //What

}
impl<R: Registrar + 'static, P: AsRef<Path>> MapFile<R, P>{
    pub async fn init(path: P, registrar: &R) -> Result<Self>{
        let region_dir = Arc::new(path.as_ref().join("region"));
        match fs::create_dir_all(&*region_dir).await {
            Ok(_) => {}
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists{
                    return Err(err.into());
                }
            }
        }

        Ok(Self { 
            path,
            region_dir,
            __marker: PhantomData,
        })
    }
    ///batch read these ChunkPos, NOT preserving order.
    pub async fn read(&self, pos: impl Iterator<Item=ChunkPos>) -> Vec<Result<(ChunkPos,Option<Box<ChunkData<R>>>)>>{
        let mut join_set: JoinSet<Result<Vec<Result<(RegionPos, RegionLocalPos, Option<Box<ChunkData<R>>>)>>>> = JoinSet::new();
        for (r_pos, r_local_pos) in Self::break_down_c_pos(pos){
            let path= self.region_path(r_pos);
            join_set.spawn(async move {
                let file = OpenOptions::new().read(true).write(false).open(path).await?;
                let mut region_file: RegionFile<SmallerChunk<R>> = RegionFile::init(file).await?;

                let mut list = Vec::with_capacity(r_local_pos.len());

                for local in r_local_pos{
                    let read = region_file.read(local.into()).await;
                    let read = match read{
                        Ok(data) => {
                            let data = match data{
                                None => None,
                                Some(data) => Some(Box::from(data.to_data()))
                            };
                            Ok((r_pos, local, data))
                        }
                        Err(err) => Err(err.into()),
                    };
                    list.push(read)
                }
                Ok(list)
            });
        }
        let mut list = Vec::with_capacity(join_set.len());
        while let Some(result) = join_set.join_next().await{
            let set_list = match result{
                Ok(result) => {
                    match result{
                        Ok(set_list) => {
                            set_list
                        }
                        Err(err) => {
                            list.push(Err(err.into()));
                            continue;
                        }
                    }

                }
                Err(err) => {
                    list.push(Err(err.into()));
                    continue;
                }
            };
            for result in set_list{
                list.push(match result{
                    Ok((r_pos,r_local_pos,data)) => Ok((combine_region_pos(r_pos,r_local_pos,),data)),
                    Err(err) => Err(err.into())
                })
            }
        }
        list
    }
    ///Batch writes this iterator of chunk data and associated position.
    pub async fn write<D, I>(&self, pos_data: I) -> Result<(), Vec<ErrorStruct>>
    where
        I: Iterator<Item=(ChunkPos, D)>,
        D: Deref<Target=ChunkData<R>> + Send + Sync + 'static
    {
        let mut join_set: JoinSet<Result<Vec<ErrorStruct>>> = JoinSet::new();
        for (r_pos, list) in break_down_c_pos_with_data::<D>(pos_data){
            let path= self.region_path(r_pos);
            join_set.spawn(async move{
                let file = OpenOptions::new().read(true).write(true).create(true).open(&path).await
                    .with_context(||format!("Failed to open region file at {:?} during write operation for \"{:?}.\"", path, r_pos))?;
                let mut region_file: RegionFile<SmallerChunk<R>> = RegionFile::init(file).await
                    .context("Failed to initialize region file during write operation.")?;

                let mut errors: Vec<ErrorStruct> = Vec::new();

                for (pos, data) in list{
                    if let Err(err) = region_file.write(pos.into(), &SmallerChunk::new(&data)).await{
                        errors.push(err.into())
                    }
                }
                Ok(errors)
            });
        }
        let mut errors = Vec::new(); //TODO: with capacity?
        while let Some(result) = join_set.join_next().await{
            match result{
                Ok(result) => match result{
                    Ok(err_list) => errors.extend(err_list),
                    Err(err) => errors.push(err.into())
                },
                Err(err) => errors.push(err.into())
            }
        }
        if errors.len() != 0{
            Err(errors)
        } else {
            Ok(())
        }
    }

    /// Returns a vector of `LocalRegionPos` of the chunks and the associated `RegionPos` needed to contain these ChunkPos.
    fn break_down_c_pos(pos: impl Iterator<Item=ChunkPos>) -> Vec<((RegionPos,Vec<RegionLocalPos>))>{
        let mut list: HashMap<RegionPos, Vec<RegionLocalPos>,_> = FxHashMap::default();
        for pos in pos {
            if let Some(vec) = list.get_mut(&pos.into()){
                vec.push(pos.into())
            } else {
                list.insert(pos.into(), vec![pos.into()]);
            }
        }
        list.into_iter().collect_vec()
    }

    fn region_path(&self, pos: RegionPos) -> PathBuf{
        let mut path = self.region_dir.join(format!(",{},{},{}",pos.0,pos.1,pos.2));
        path.set_extension("dat");
        path
    }
}
/// Returns a vector of `LocalRegionPos` of the chunks and the associated `RegionPos` needed to contain these ChunkPos.
fn break_down_c_pos_with_data<T>(pos: impl Iterator<Item=(ChunkPos,T)>) -> Vec<((RegionPos,Vec<(RegionLocalPos,T)>))>{
    let mut list: HashMap<RegionPos, Vec<(RegionLocalPos,T)>,_> = FxHashMap::default();
    for (pos,t) in pos {
        if let Some(vec) = list.get_mut(&pos.into()){
            vec.push((pos.into(),t))
        } else {
            list.insert(pos.into(), vec![(pos.into(), t)]);
        }
    }
    list.into_iter().collect_vec()
}


///Pos of region in region space.
#[derive(Default, Debug, Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct RegionPos(pub i16,pub i16,pub i16);

impl From<ChunkPos> for RegionPos {
    fn from(value: ChunkPos) -> Self {
        let n = |val: i16| {
            if val.is_negative(){
                (val - 15)/16
            } else{
                val/16
            }
        };
        Self(
            n(value.0) as i16,
            n(value.1) as i16,
            n(value.2) as i16
        )
    }
}
///Pos local to a region
#[derive(Default, Debug, Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct RegionLocalPos(pub u16,pub u16,pub u16);

impl Into<PosU> for RegionLocalPos {
    fn into(self) -> PosU {
        PosU(self.0 as usize,self.1 as usize,self.2 as usize)
    }
}

impl From<ChunkPos> for RegionLocalPos {
    fn from(value: ChunkPos) -> Self {
        let n = |val: i16| {
            val.rem_euclid(16)
        };
        Self(
            n(value.0) as u16,
            n(value.1) as u16,
            n(value.2) as u16
        )    }
}

//TODO: Test for this function
fn combine_region_pos(pos: RegionPos, local_pos: RegionLocalPos) -> ChunkPos{
    let inverse = |val: i16| {
        val*16
    };
    ChunkPos(
        inverse(pos.0) + local_pos.0 as i16,
        inverse(pos.1) + local_pos.1 as i16,
        inverse(pos.2) + local_pos.2 as i16,
    )
}

#[cfg(test)]
mod tests {
    use core_obj::fake::{FakeRegistrar, FakeVoxel};

    use super::*;

    #[test]
    fn read_round_trip() {

    }

    #[test]
    fn combine_region_pos_round_trip() {
        let chunk_pos = ChunkPos(0,0,0);
        assert_eq!(chunk_pos, combine_region_pos(chunk_pos.into(),chunk_pos.into()));
        let chunk_pos = ChunkPos(0,1,2);
        assert_eq!(chunk_pos, combine_region_pos(chunk_pos.into(),chunk_pos.into()));
        let chunk_pos = ChunkPos(-16,-16,-16);
        assert_eq!(RegionPos(-1,-1,-1), chunk_pos.into());
        assert_eq!(RegionLocalPos(0,0,0), chunk_pos.into());
        assert_eq!(chunk_pos, combine_region_pos(chunk_pos.into(),chunk_pos.into()));
        let chunk_pos = ChunkPos(-1,-1,-1);
        assert_eq!(RegionPos(-1,-1,-1), chunk_pos.into());
        assert_eq!(RegionLocalPos(15,15,15), chunk_pos.into());
        assert_eq!(chunk_pos, combine_region_pos(chunk_pos.into(),chunk_pos.into()));
        let chunk_pos = ChunkPos(200,-100,400);
        assert_eq!(chunk_pos, combine_region_pos(chunk_pos.into(),chunk_pos.into()));
    }

    #[test]
    fn break_down_c_pos() {
        let vec: Vec<ChunkPos> = vec![
            ChunkPos(0,0,0),    // R: 0,0,0
            ChunkPos(4,6,5),    // R: 0,0,0
            ChunkPos(15,14,15), // R: 0,0,0
            ChunkPos(0,0,15), // R: 0,0,0
            ChunkPos(16,0,0), // R: 1,0,0
            ChunkPos(0,16,0), // R: 0,1,0
            ChunkPos(0,0,16), // R: 0,0,1
            ChunkPos(-1,-4,-1), // R: -1,-1,-1
            ChunkPos(-1,-15,-14), // R: -1,-1,-1
            ChunkPos(-14,15,2), // R: -1,0,0
            ChunkPos(25,4,4),  // R: 1,0,0
        ];
        let vec = MapFile::<FakeRegistrar, &Path>::break_down_c_pos(vec.iter().map(|v|*v));
        assert_eq!(vec.len(), 6); // There are six regions needed for these requested chunks.
        let (_,list) = vec.iter().find(|&(r_pos,_)|{ *r_pos == RegionPos(0,0,0) }).unwrap();
        assert!(list.contains(&RegionLocalPos(0,0,0)));
        assert!(list.contains(&RegionLocalPos(4,6,5)));
        assert!(list.contains(&RegionLocalPos(0,0,15)));
        assert!(list.contains(&RegionLocalPos(15,14,15)));
        let (_,list) = vec.iter().find(|&(r_pos,_)|{ *r_pos == RegionPos(1,0,0) }).unwrap();
        assert!(list.contains(&RegionLocalPos(0,0,0)));
        assert!(list.contains(&RegionLocalPos(9,4,4)));
        let (_,list) = vec.iter().find(|&(r_pos,_)|{ *r_pos == RegionPos(-1,-1,-1) }).unwrap();
        assert!(list.contains(&RegionLocalPos(15,12,15)));
        assert!(list.contains(&RegionLocalPos(15,1,2)));
        let (_,list) = vec.iter().find(|&(r_pos,_)|{ *r_pos == RegionPos(-1,0,0) }).unwrap();
        assert!(list.contains(&RegionLocalPos(2,15,2)));
        // TODO: there are more that i should make but im lazy
    }

    #[test]
    fn chunk_pos_to_region_pos() {
        let region_pos: RegionPos = ChunkPos(0,0,0).into();
        assert_eq!(region_pos,RegionPos(0,0,0));

        let region_pos: RegionPos = ChunkPos(15,15,15).into();
        assert_eq!(region_pos,RegionPos(0,0,0));

        let region_pos: RegionPos = ChunkPos(-15,-15,-15).into();
        assert_eq!(region_pos,RegionPos(-1,-1,-1));

        let region_pos: RegionPos = ChunkPos(-17,-15,-15).into();
        assert_eq!(region_pos,RegionPos(-2,-1,-1));
    }

    #[test]
    fn region_local_pos() {
        let region_pos: RegionLocalPos = ChunkPos(0,0,0).into();
        assert_eq!(region_pos,RegionLocalPos(0,0,0));

        let region_pos: RegionLocalPos = ChunkPos(-1,0,0).into();
        assert_eq!(region_pos,RegionLocalPos(15,0,0));
    }

    #[tokio::test]
    async fn round_trip_map_file(){ //TODO: DO with voxel data as well
        let file = tempfile::TempDir::new().unwrap();
        let path = file.path();
        let map_file = MapFile::init(path, &FakeRegistrar::default()).await.unwrap();
        let mut chunks = FxHashMap::default();

        // Test data
        let mut data: ChunkData<FakeRegistrar> = ChunkData::default();
        *data.voxels.get_mut(PosU(1,2,3)) = Some(FakeVoxel(1));
        *data.voxels.get_mut(PosU(0,0,0)) = Some(FakeVoxel(1));
        *data.voxels.get_mut(PosU(12,15,2)) = Some(FakeVoxel(12));
        chunks.insert(ChunkPos(0,0,0), Arc::new(data));
        chunks.insert(ChunkPos(0,1,1),  Default::default());
        chunks.insert(ChunkPos(0,0,1), Default::default());
        chunks.insert(ChunkPos(-1,-1,1),  Default::default());
        chunks.insert(ChunkPos(-3948,-2349,3758),  Default::default());
        let mut data: ChunkData<FakeRegistrar> = ChunkData::default();
        *data.voxels.get_mut(PosU(0,0,0)) = Some(FakeVoxel(2));
        *data.voxels.get_mut(PosU(0,0,5)) = Some(FakeVoxel(3));
        *data.voxels.get_mut(PosU(11,0,4)) = Some(FakeVoxel(5));
        chunks.insert(ChunkPos(0,1,0), Arc::new(data));

        map_file.write(chunks.iter().map(|(k,v)|(*k,v.clone()))).await.expect("Writing Test failed with: {}");

        let read = map_file.read(chunks.iter().map(|(k,_)|*k)).await;
        for result in read{
            let (pos, chunk) = result.unwrap();
            let original = chunks.get(&pos).unwrap().deref();
            let read_chunk = chunk.with_context(||format!("There is no chunk {:?}",pos)).unwrap();
            //FIXME: writes to the correct (i think) region files, the files are not completely blank and "differ" (according to sdiff). However reading is bring back None...
            assert_eq!(*original, *read_chunk.deref());

        }
    }
}