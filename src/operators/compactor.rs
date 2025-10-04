use crate::update::Update;
use std::collections::HashMap;
use std::hash::Hash;
use std::num::NonZeroIsize;

pub fn compact<T, I>(input: I) -> Vec<Update<T>>
where
    T: Eq + Hash + Clone,
    I: IntoIterator<Item = Update<T>>,
{
    let mut counts = HashMap::new();
    for update in input.into_iter() {
        *counts.entry(update.item).or_insert(0) += update.diff.get();
    }

    counts
        .into_iter()
        .filter_map(|(item, diff)| NonZeroIsize::new(diff).map(|diff| Update { item, diff }))
        .collect()
}
