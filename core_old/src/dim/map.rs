use super::chunk::Chunk;
use super::DimMap;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Map<const S: usize, const VOL: usize> {
    map: SLIVec<SLIVec<SLIVec<Chunk<S, VOL>>>>,
}
impl<const S: usize, const VOL: usize> DimMap for Map<S, VOL> {}

use sli_vec::SignedLazyImpliedVec as SLIVec;
mod sli_vec {
    ///
    #[derive(Debug, Default)]
    pub(super) struct SignedLazyImpliedVec<T: Default + Sized> {
        pos: Vec<Option<T>>,
        neg: Vec<Option<T>>,
    }
    impl<T: Default + Sized> SignedLazyImpliedVec<T> {
        /// Assumes that the Vecs are full, this is only a performance thing.
        pub fn get_all(&self) -> Vec<(i32, &T)> {
            let neg = self.neg.iter().enumerate().filter_map(|(i, t)| {
                if let Some(t) = t {
                    Some((i as i32 * -1, t))
                } else {
                    None
                }
            });
            let pos = self.pos.iter().enumerate().filter_map(|(i, t)| {
                if let Some(t) = t {
                    Some((i as i32, t))
                } else {
                    None
                }
            });
            neg.chain(pos).collect()
        }
        /// Assumes that the Vecs are full, this is only a performance thing.
        pub fn get_all_mut(&mut self) -> Vec<(i32, &mut T)> {
            let neg = self.neg.iter_mut().enumerate().filter_map(|(i, t)| {
                if let Some(t) = t {
                    Some((i as i32 * -1, t))
                } else {
                    None
                }
            });
            let pos = self.pos.iter_mut().enumerate().filter_map(|(i, t)| {
                if let Some(t) = t {
                    Some((i as i32, t))
                } else {
                    None
                }
            });
            neg.chain(pos).collect()
        }
        pub fn get(&self, index: i32) -> &Option<T> {
            if index.is_positive() || index == 0 {
                self.pos.get(index as usize)
            } else {
                self.neg.get(index.abs() as usize - 1)
            }
            .unwrap_or(&None)
        }
        pub fn get_mut(&mut self, index: i32) -> Option<&mut T> {
            if index.is_positive() || index == 0 {
                self.pos.get_mut(index as usize)?.as_mut()
            } else {
                self.neg.get_mut(index.abs() as usize - 1)?.as_mut()
            }
        }
        pub fn ensure(&mut self, len: i32) {
            if len.is_positive() || len == 0 {
                let len = len as usize;
                if len >= self.pos.len() {
                    self.pos.resize_with(len, || None)
                }
            } else {
                let len = len.abs() as usize - 1;
                if len >= self.neg.len() {
                    self.neg.resize_with(len, || None)
                }
            }
        }
    }
}
