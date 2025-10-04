#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Update<T> {
    Add(T),
    Remove(T),
}

impl<T> Update<T> {
    pub fn as_ref(&self) -> Update<&T> {
        match self {
            Update::Add(item) => Update::Add(item),
            Update::Remove(item) => Update::Remove(item),
        }
    }

    pub fn map<U, F>(self, f: F) -> Update<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Update::Add(item) => Update::Add(f(item)),
            Update::Remove(item) => Update::Remove(f(item)),
        }
    }

    pub fn item(&self) -> &T {
        match self {
            Update::Add(item) => item,
            Update::Remove(item) => item,
        }
    }
}
