use std::num::NonZeroIsize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Update<T> {
    pub item: T,
    pub diff: NonZeroIsize,
}

impl<T> Update<T> {
    pub fn add(item: T) -> Self {
        Update { item, diff: NonZeroIsize::new(1).unwrap() }
    }

    pub fn remove(item: T) -> Self {
        Update { item, diff: NonZeroIsize::new(-1).unwrap() }
    }

    pub fn map<U, F>(self, f: F) -> Update<U>
    where
        F: FnOnce(T) -> U,
    {
        Update {
            item: f(self.item),
            diff: self.diff,
        }
    }

    pub fn item(&self) -> &T {
        &self.item
    }
}