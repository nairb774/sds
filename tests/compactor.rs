use sds::operators::compactor::compact;
use sds::update::Update;
use std::collections::HashSet;
use std::num::NonZeroIsize;

// Helper to create NonZeroIsize for tests, panicking if the value is zero.
fn nz(i: isize) -> NonZeroIsize {
    NonZeroIsize::new(i).unwrap()
}

// Helper to compare slices of Updates as sets, ignoring order.
fn assert_set_eq(a: &[Update<i32>], b: &[Update<i32>]) {
    let a_set: HashSet<_> = a.iter().cloned().collect();
    let b_set: HashSet<_> = b.iter().cloned().collect();
    assert_eq!(a_set, b_set);
}

#[test]
fn test_compactor_simple_cancellation() {
    let updates = vec![Update::add(1), Update::remove(1)];
    let compacted = compact(updates);
    assert_set_eq(&compacted, &[]);
}

#[test]
fn test_compactor_reversed_cancellation() {
    let updates = vec![Update::remove(1), Update::add(1)];
    let compacted = compact(updates);
    assert_set_eq(&compacted, &[]);
}

#[test]
fn test_compactor_multi_add() {
    let updates = vec![Update::add(1), Update::add(1), Update::remove(1)];
    let compacted = compact(updates);
    assert_set_eq(&compacted, &[Update { item: 1, diff: nz(1) }]);
}

#[test]
fn test_compactor_multi_remove() {
    let updates = vec![Update::add(1), Update::remove(1), Update::remove(1)];
    let compacted = compact(updates);
    assert_set_eq(&compacted, &[Update { item: 1, diff: nz(-1) }]);
}

#[test]
fn test_compactor_multiple_items() {
    let updates = vec![
        Update::add(1),
        Update::add(2),
        Update::remove(1),
        Update::add(3),
        Update::add(2),
        Update::remove(2),
    ];
    let compacted = compact(updates);
    assert_set_eq(
        &compacted,
        &[Update { item: 2, diff: nz(1) }, Update { item: 3, diff: nz(1) }],
    );
}

#[test]
fn test_compactor_no_cancellations() {
    let updates = vec![
        Update { item: 1, diff: nz(1) },
        Update { item: 2, diff: nz(1) },
        Update { item: 3, diff: nz(-1) },
    ];
    let compacted = compact(updates.clone());
    assert_set_eq(&compacted, &updates);
}

#[test]
fn test_compactor_empty_input() {
    let updates: Vec<Update<i32>> = vec![];
    let compacted = compact(updates);
    assert_set_eq(&compacted, &[]);
}

#[test]
fn test_compactor_complex_counts() {
    let updates = vec![
        Update { item: 1, diff: nz(2) },
        Update { item: 1, diff: nz(-1) },
        Update { item: 2, diff: nz(-3) },
        Update { item: 2, diff: nz(2) },
        Update { item: 3, diff: nz(5) },
        Update { item: 3, diff: nz(-5) },
    ];
    let compacted = compact(updates);
    assert_set_eq(
        &compacted,
        &[
            Update { item: 1, diff: nz(1) },
            Update { item: 2, diff: nz(-1) },
        ],
    );
}