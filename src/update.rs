#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Update<T> {
    Add(T),
    Remove(T),
}
