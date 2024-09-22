#![feature(future_join)]

use std::future::join;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use itertools::Itertools;
use tokio::sync::RwLock;
use prelude::*;
use world_db::disk::{EncodedChunk, MapFile};
use world_db::map::fake::test_chunk;
use world_db::map::{ChunkData, Map};
use world_db::{ChunkPos, MapTrait};

#[test]
fn encode_decode() -> Result<()>{
	let data = test_chunk(124532);
	let encoded = EncodedChunk::encode(&data);
	assert_eq!(data, encoded.decode()?);
	let data = test_chunk(3);
	let encoded = EncodedChunk::encode(&data);
	assert_eq!(data, encoded.decode()?);
	let data = ChunkData::default();
	let encoded = EncodedChunk::encode(&data);
	assert_eq!(data, encoded.decode()?);
	Ok(())
}

#[tokio::test]
async fn write_chunks() -> Result<()> {
	use tokio::sync::Mutex;
	let file = tempfile::TempDir::new()?;
	let map = Map::new();

	//testing one
	let data_pos = [
		(test_chunk(11), ChunkPos(1, 1, 1)),
		(test_chunk(11), ChunkPos(0, 0, 0)),
		(test_chunk(11), ChunkPos(i16::MAX, i16::MAX, i16::MAX)),
		(test_chunk(11), ChunkPos(7, -24, 4)),
		(test_chunk(11), ChunkPos(-i16::MAX, -i16::MAX, -i16::MAX)),
	];

	{ //write to in memory map
		async fn write(map: &Map, pos: &ChunkPos, chunk: &ChunkData) {
			let mut write = map.write(*pos).await.unwrap();
			write.set_data(chunk.clone());
		}
		let futures = data_pos
			.iter()
			.map(|(data, pos)| write(&map, pos, data))
			.collect_vec();
		futures::future::join_all(futures).await;
	}


	let path = Arc::new(file.path().to_path_buf());
	let map_file = RwLock::new(MapFile::init(path).await?);
	//write to in disk map
	{
		async fn write(map: &Map, map_file: &RwLock<MapFile>, pos: &ChunkPos) {
			let mut map_file = map_file.write().await;
			map_file.write(*pos, &map.read(*pos).await.unwrap().unwrap().encode()).await.unwrap();
		}
		let futures = data_pos
			.iter()
			.map(|(data, pos)| write(&map, &map_file, pos))
			.collect_vec();
		futures::future::join_all(futures).await;
	}
	let mut data = Mutex::new(vec![ChunkData::default(); data_pos.len()]);
	//read to into data
	{
		async fn read(map_file: &RwLock<MapFile>, data: &Mutex<Vec<ChunkData>>, pos: &ChunkPos, index: usize) {
			*data.lock().await.get_mut(index).unwrap() =
				map_file.read().await.read(*pos).await.unwrap().unwrap().decode().unwrap();
		}
		let futures = data_pos
			.iter()
			.enumerate()
			.map(|(index, (_, pos))| read(&map_file, &data, pos, index))
			.collect_vec();
		futures::future::join_all(futures).await;
	}
	let data_read = data.lock().await;
	assert_eq!(data_read[0],data_pos[0].0);
	assert_eq!(data_read[1],data_pos[1].0);
	assert_eq!(data_read[2],data_pos[2].0);
	assert_eq!(data_read[3],data_pos[3].0);
	assert_eq!(data_read[4],data_pos[4].0);

	drop(file);
	Ok(())
}
