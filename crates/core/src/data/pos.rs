use sc_prelude::*;
use std::{
    error::Error,
    ops::{Div, Mul, Rem, Sub},
};

const CHUNK_SIZE: i16 = crate::CHUNK_SIZE as i16;

pub trait Pos<T: Copy + Clone + Debug + Default>: Copy + Clone + Default + Debug {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
    fn new(tuple: (T, T, T)) -> Self;
    #[inline]
    fn tuple<V: From<T>>(&self) -> (V, V, V) {
        (self.x().into(), self.y().into(), self.z().into())
    }
    #[inline]
    fn try_tuple<V: TryFrom<T>>(&self) -> Result<(V, V, V)>
    where
        <V as TryFrom<T>>::Error: Error + Send + Sync + 'static,
    {
        Ok((
            self.x().try_into()?,
            self.y().try_into()?,
            self.z().try_into()?,
        ))
    }
    #[inline(always)]
    fn mul<V: Mul<T, Output = T> + Copy>(&self, mul: V) -> Self {
        Self::new((mul * self.x(), mul * self.y(), mul * self.z()))
    }
    #[inline(always)]
    fn div<V: Copy>(&self, div: V) -> Self
    where
        T: Div<V, Output = T>,
    {
        Self::new((self.x() / div, self.y() / div, self.z() / div))
    }
    #[inline(always)]
    fn modulo<V: Copy>(&self, div: V) -> Self
    where
        T: Rem<V, Output = T>,
    {
        Self::new((self.x() % div, self.y() % div, self.z() % div))
    }
    #[inline(always)]
    fn sub(&self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self::new((self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()))
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct RelativePos(i16, i16, i16);
impl Pos<i16> for RelativePos {
    #[inline(always)]
    fn x(&self) -> i16 {
        self.0
    }
    #[inline(always)]
    fn y(&self) -> i16 {
        self.1
    }
    #[inline(always)]
    fn z(&self) -> i16 {
        self.2
    }
    #[inline(always)]
    fn new(tuple: (i16, i16, i16)) -> Self {
        Self(tuple.0, tuple.1, tuple.2)
    }
}
#[derive(Default, Debug, Clone, Copy)]
pub struct GlobalPos{
    chunk: (i16,i16,i16),
    relative: RelativePos,
}
impl GlobalPos {
    #[inline(always)]
    pub fn relative(&self) -> RelativePos {
        RelativePos::new(self.modulo(CHUNK_SIZE).tuple())
    }
    #[inline(always)]
    pub fn chunk(&self) -> (i16,i16,i16){
        self.chunk
    }
    #[inline(always)]
    pub fn new_from_parts(chunk: (i16,i16,i16), relative: RelativePos) -> Self{
        Self { chunk, relative }
    }
}

impl Pos<i16> for GlobalPos {
    #[inline(always)]
    fn x(&self) -> i16 {
        self.chunk.0*CHUNK_SIZE + self.relative.0
    }
    #[inline(always)]
    fn y(&self) -> i16 {
        self.chunk.1*CHUNK_SIZE + self.relative.1
    }
    #[inline(always)]
    fn z(&self) -> i16 {
        self.chunk.2*CHUNK_SIZE + self.relative.2
    }
    #[inline(always)]
    fn new(tuple: (i16, i16, i16)) -> Self {
        Self{
            chunk: (tuple.0 / CHUNK_SIZE, tuple.1 / CHUNK_SIZE, tuple.2 / CHUNK_SIZE),
            relative: RelativePos::new((tuple.0 % CHUNK_SIZE, tuple.1 % CHUNK_SIZE, tuple.2 % CHUNK_SIZE)),
        }
    }
}

///Like a global position but not chunk wise.
#[derive(Default, Debug, Clone, Copy)]
pub struct GlobalAbsPos{
    tuple: (i16,i16,i16)
}
impl Pos<i16> for GlobalAbsPos{
    fn x(&self) -> i16 {
        self.tuple.0
    }

    fn y(&self) -> i16 {
        self.tuple.1
    }

    fn z(&self) -> i16 {
        self.tuple.2
    }

    fn new(tuple: (i16, i16, i16)) -> Self {
        Self{
            tuple,
        }
    }
}
impl Into<GlobalPos> for GlobalAbsPos{
    #[inline(always)]
    fn into(self) -> GlobalPos {
        GlobalPos::new(self.tuple)
    }
}
impl From<GlobalPos> for GlobalAbsPos{
    #[inline]
    fn from(value: GlobalPos) -> Self {
        Self { tuple: (
                value.chunk.0 * CHUNK_SIZE + value.relative.0,
                value.chunk.1 * CHUNK_SIZE + value.relative.1,
                value.chunk.2 * CHUNK_SIZE + value.relative.2,
            ) 
        }
    }
}