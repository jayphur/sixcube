macro_rules! dynamic_static_trait_ptr {
    ($trait_name:ident) => {
        paste::paste! {
            dyn_clone::clone_trait_object!($trait_name);
            #[derive(Debug, Clone)]
            pub enum [<$trait_name Ptr>] {
                Static(&'static (dyn $trait_name + Send + Sync)),
                Dynamic(Box<(dyn $trait_name + Send)>),
            }
            impl [<$trait_name Ptr>] {
                #[inline]
                pub fn as_ref(&self) -> &dyn $trait_name {
                    match self{
                        Self::Static(s) => {
                            s.clone()
                        },
                        Self::Dynamic(b) => {
                            (*b).as_ref()
                        } 
                    }
                }
                #[inline]
                pub fn try_mut(&mut self) -> Option<&mut dyn $trait_name> {
                    match self{
                        Self::Static(_) => {
                            None
                        },
                        Self::Dynamic(b) => {
                            Some((*b).as_mut())
                        } 
                    }
                }
            }
            impl crate::trait_ptr::TraitPtr for [<$trait_name Ptr>] {
                // whole load of nothing
            }
            impl From<&'static (dyn $trait_name + Send + Sync)> for [<$trait_name Ptr>]{
                #[inline]
                fn from(value: &'static (dyn $trait_name + Send + Sync)) -> Self{
                    Self::Static(value)
                }
            }
            impl From<Box<(dyn $trait_name + Send)>> for [<$trait_name Ptr>]{
                fn from(value: Box<(dyn $trait_name + Send)>) -> Self{
                    Self::Dynamic(value)
                }
            }
        }
    };
}
macro_rules! static_trait_ptr {
    ($trait_name:ident) => {
        paste::paste! {
            dyn_clone::clone_trait_object!($trait_name);
            #[derive(Debug, Clone)]
            pub struct [<$trait_name Ptr>] (&'static (dyn $trait_name + Send + Sync));
            impl [<$trait_name Ptr>] {
                #[inline]
                pub fn as_ref(&self) -> &dyn $trait_name {
                    self.0
                }
            }
            impl crate::trait_ptr::TraitPtr for [<$trait_name Ptr>] {
                // whole load of nothing
            }
            impl From<&'static (dyn $trait_name + Send + Sync)> for [<$trait_name Ptr>]{
                #[inline]
                fn from(value: &'static (dyn $trait_name + Send + Sync)) -> Self{
                    Self(value)
                }
            }
        }
    };
}
use sc_prelude::Debug;
/// a lot of the trait pointer stuff isn't really trait compatible. 
/// this more exists as a way of having trait pointers in generics.
pub trait TraitPtr: Clone + Debug{

} 