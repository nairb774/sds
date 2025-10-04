use crate::update::Update;

pub trait Set<T> {
    type Iter<'a>: Iterator<Item = &'a T>
    where
        Self: 'a,
        T: 'a;

    fn apply_changes(&mut self, changes: Vec<Update<T>>);
    fn iter<'a>(&'a self) -> Self::Iter<'a>;
}