use world_db;

#[cfg(feature = "disk")]
mod disk_tests{
	use std::sync::Arc;

	use world_db::disk::{EncodedChunk, MapFile};
	use world_db::map::fake::test_chunk;
	use world_db::map::{ChunkData, Map};
	use world_db::{ChunkPos, MapTrait};

	#[test]
	fn encode_decode(){
		let data = test_chunk(124532);
		let encoded = EncodedChunk::encode(&data);
		assert_eq!(data, encoded.decode().unwrap());
		let data = test_chunk(3);
		let encoded = EncodedChunk::encode(&data);
		assert_eq!(data, encoded.decode().unwrap());
		let data = ChunkData::default();
		let encoded = EncodedChunk::encode(&data);
		assert_eq!(data, encoded.decode().unwrap());
	}

	#[tokio::test]
	async fn write_single_chunk() {
		let file = tempfile::TempDir::new().unwrap();
		let data = test_chunk(8935);
		let map = Map::new();
		let pos = ChunkPos(1,1,1);
		{
			let mut write = map.write(pos).await.unwrap();
			write.set_data(data.clone());
 		}
		let path = Arc::new(file.path().to_path_buf());
		let map_file: MapFile = MapFile::init(path).await.unwrap();
		let encoded = map.read(pos).await.unwrap().unwrap().encode();
		map_file.write(pos,&encoded).await.unwrap();
		let data = map_file.read(pos).await.unwrap().unwrap().decode().unwrap();
		assert_eq!(data, data);
		drop(file)
	}
}