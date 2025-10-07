use crate::operator::Operator;
use crate::update::Update;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::mem;

pub struct TopN<T>
where
    T: Ord + Clone,
{
    n: usize,
    all_items: BTreeMap<T, usize>,
    top_items: BTreeMap<T, usize>,
}

impl<T> TopN<T>
where
    T: Ord + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            all_items: BTreeMap::new(),
            top_items: BTreeMap::new(),
        }
    }
}

impl<T> Operator<T, T> for TopN<T>
where
    T: Ord + Clone,
{
    fn process_changes(&mut self, input_changes: Vec<Update<T>>) -> Vec<Update<T>> {
        if self.n == 0 {
            return Vec::new();
        }

        // Update the counts of all items based on the input changes.
        for change in input_changes {
            let Update { item, diff } = change;
            let diff = diff.get();

            match self.all_items.entry(item) {
                Entry::Occupied(mut entry) => {
                    let count = entry.get_mut();
                    if diff > 0 {
                        *count += diff as usize;
                    } else {
                        *count = count.saturating_sub((-diff) as usize);
                    }
                    if *count == 0 {
                        entry.remove();
                    }
                }
                Entry::Vacant(entry) => {
                    if diff > 0 {
                        entry.insert(diff as usize);
                    }
                }
            }
        }

        let old_top_items = mem::take(&mut self.top_items);

        let mut n_collected = 0;

        // Determine the new set of top N items.
        for (item, &count) in self.all_items.iter().rev() {
            if n_collected >= self.n {
                break;
            }
            let can_take = self.n - n_collected;
            let taking = std::cmp::min(can_take, count);
            self.top_items.insert(item.clone(), taking);
            n_collected += taking;
        }

        let mut output_changes = Vec::new();

        // Calculate additions: items in new_top_items but not in old_top_items, or with a higher count.
        for (item, &new_count) in &self.top_items {
            let old_count = old_top_items.get(item).cloned().unwrap_or(0);
            if new_count > old_count {
                for _ in 0..(new_count - old_count) {
                    output_changes.push(Update::add(item.clone()));
                }
            }
        }

        // Calculate removals: items in old_top_items but not in new_top_items, or with a lower count.
        for (item, &old_count) in &old_top_items {
            let new_count = self.top_items.get(item).cloned().unwrap_or(0);
            if old_count > new_count {
                for _ in 0..(old_count - new_count) {
                    output_changes.push(Update::remove(item.clone()));
                }
            }
        }

        output_changes
    }
}
