use std::hash::Hash;
use std::sync::Arc;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Name(Arc<String>);
impl From<String> for Name{
    fn from(value: String) -> Self {
        Self(Arc::new(value))
    }
}
impl From<Arc<String>> for Name{
    fn from(value: Arc<String>) -> Self {
        Self(value)
    }
}
impl From<&str> for Name{
    fn from(value: &str) -> Self {
        Self(Arc::new(value.to_string()))
    }
}
impl Hash for Name{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ref().hash(state)
    }
}

#[cfg(test)]
mod test{
    use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}};

    use crate::Name;
    #[test]
    fn hashing_leads_to_eq(){
        let name1: Name = "red".into();
        let name2: Name = "red".into();
        let name3: Name = "purple".into();
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