use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct StringDictionary{
    list: Vec<String>,
}
impl StringDictionary{
    pub fn find_or_maybe_push(&mut self, key: &String) -> usize{
        match self.list.iter().position(|string| *string == *key){
            Some(pos) => pos,
            None => {
                self.list.push(key.clone());
                self.list.len() - 1
            },
        }
    }
    pub fn get(&self, index: usize) -> Option<&String>{
        self.list.get(index)
    }
    pub fn inner(&self) -> &Vec<String>{
        &self.list
    }
}