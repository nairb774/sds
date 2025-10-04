use crate::set::Set;
use crate::update::Update;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub struct BTreeSetStore<T> {
    set: BTreeSet<T>,
}

impl<T> BTreeSetStore<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for BTreeSetStore<T> {
    fn default() -> Self {
        BTreeSetStore {
            set: BTreeSet::new(),
        }
    }
}

impl<T> Set<T> for BTreeSetStore<T>
where
    T: Ord + Clone,
{
    type Iter<'a> = std::collections::btree_set::Iter<'a, T> where T: 'a;

    fn apply_changes(&mut self, changes: Vec<Update<T>>) {
        for change in changes {
            match change {
                Update::Add(value) => {
                    self.set.insert(value);
                }
                Update::Remove(value) => {
                    self.set.remove(&value);
                }
            }
        }
    }

    fn iter<'a>(&'a self) -> Self::Iter<'a> {
        self.set.iter()
    }
}

impl<T> FromIterator<T> for BTreeSetStore<T>
where
    T: Ord,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        BTreeSetStore {
            set: BTreeSet::from_iter(iter),
        }
    }
}