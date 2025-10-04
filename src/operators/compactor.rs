use crate::update::Update;
use std::collections::HashMap;
use std::hash::Hash;

pub fn compact<T, I>(input: I) -> Vec<Update<T>>
where
    T: Eq + Hash + Clone,
    I: IntoIterator<Item = Update<T>>,
{
    let mut counts = HashMap::new();
    for update in input.into_iter() {
        match update {
            Update::Add(item) => *counts.entry(item).or_insert(0) += 1,
            Update::Remove(item) => *counts.entry(item).or_insert(0) -= 1,
        }
    }

    let mut output = Vec::new();
    for (item, count) in counts.drain() {
        if count > 0 {
            for _ in 0..count {
                output.push(Update::Add(item.clone()));
            }
        } else if count < 0 {
            for _ in 0..-count {
                output.push(Update::Remove(item.clone()));
            }
        }
    }
    output
}