use std::{hash::Hash, ops::Deref};
use std::sync::Arc;

use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeId(Arc<String>);
impl PartialEq<String> for TypeId{
    fn eq(&self, other: &String) -> bool {
        *self.0.as_ref() == *other
    }
}
impl PartialEq<TypeId> for String{
    fn eq(&self, other: &TypeId) -> bool {
        self == other
    }
}
impl<'a> PartialEq<TypeId> for &'a String{
    fn eq(&self, other: &TypeId) -> bool {
        self == other
    }
}
impl From<String> for TypeId{
    fn from(value: String) -> Self {
        Self(Arc::new(value))
    }
}
impl From<Arc<String>> for TypeId{
    fn from(value: Arc<String>) -> Self {
        Self(value)
    }
}
impl From<&str> for TypeId{
    fn from(value: &str) -> Self {
        Self(Arc::new(value.to_string()))
    }
}
impl Into<String> for TypeId{
    fn into(self) -> String {
        self.0.deref().clone()
    }
}
impl Hash for TypeId{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().hash(state)
    }
}
impl Serialize for TypeId{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        serializer.serialize_str(self.0.as_str())
    }
}
impl TypeId{
    pub fn string<'a>(&'a self) -> &'a String{
        self.0.deref()
    }
}

#[cfg(test)]
mod test{
    use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

    use crate::TypeId;
    #[test]
    fn hashing_leads_to_eq(){
        let name1: TypeId = "red".into();
        let name2: TypeId = "red".into();
        let name3: TypeId = "purple".into();
        let mut hasher = DefaultHasher::new();
        name1.hash(&mut hasher);
        let name1_hash = hasher.finish();
        let mut hasher = DefaultHasher::new();
        name2.hash(&mut hasher);
        let name2_hash = hasher.finish();
        let mut hasher = DefaultHasher::new();
        name3.hash(&mut hasher);
        let name3_hash = hasher.finish();

        assert_eq!(name1_hash, name2_hash);
        assert_ne!(name2_hash, name3_hash);
        assert_ne!(name1_hash, name3_hash);
    }
}