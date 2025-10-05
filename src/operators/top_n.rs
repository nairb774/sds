use crate::operator::Operator;
use crate::update::Update;
use std::collections::BTreeMap;

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

            let entry = self.all_items.entry(item);
            let key = entry.key().clone();
            let count = entry.or_insert(0);

            if diff > 0 {
                *count += diff as usize;
            } else {
                *count = count.saturating_sub((-diff) as usize);
            }

            if *count == 0 {
                self.all_items.remove(&key);
            }
        }

        let old_top_items = self.top_items.clone();
        let mut new_top_items = BTreeMap::new();
        let mut n_collected = 0;

        // Determine the new set of top N items.
        for (item, &count) in self.all_items.iter().rev() {
            if n_collected >= self.n {
                break;
            }
            let can_take = self.n - n_collected;
            let taking = std::cmp::min(can_take, count);
            new_top_items.insert(item.clone(), taking);
            n_collected += taking;
        }
        self.top_items = new_top_items;

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
