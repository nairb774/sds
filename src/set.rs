use crate::update::Update;

pub trait Set<T> {
    fn apply_changes(&mut self, changes: Vec<Update<T>>);
    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}