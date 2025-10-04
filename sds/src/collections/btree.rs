use crate::set::Set;
use crate::update::Update;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub struct BTreeSetStore<T: Ord + Clone> {
    set: BTreeSet<T>,
}

impl<T: Ord + Clone> BTreeSetStore<T> {
    pub fn new() -> Self {
        BTreeSetStore {
            set: BTreeSet::new(),
        }
    }
}

impl<T: Ord + Clone> Default for BTreeSetStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> Set<T> for BTreeSetStore<T> {
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

    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.set.iter())
    }
}

impl<T: Ord + Clone> FromIterator<T> for BTreeSetStore<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        BTreeSetStore {
            set: BTreeSet::from_iter(iter),
        }
    }
}