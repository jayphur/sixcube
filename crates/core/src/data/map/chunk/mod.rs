use sc_prelude::*;

#[derive(Default, Debug)]
pub(crate) struct Chunk<V, const S: usize>{
    _v: PhantomData<V>,
}