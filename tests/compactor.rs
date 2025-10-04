use sds::operators::compactor::compact;
use sds::update::Update;
use std::collections::HashSet;

fn run_compactor(input: Vec<Update<i32>>) -> Vec<Update<i32>> {
    compact(input)
}

fn assert_set_eq(a: &[Update<i32>], b: &[Update<i32>]) {
    let a_set: HashSet<_> = a.iter().cloned().collect();
    let b_set: HashSet<_> = b.iter().cloned().collect();
    assert_eq!(a_set, b_set);
}

#[test]
fn test_compactor_simple_cancellation() {
    let updates = vec![Update::Add(1), Update::Remove(1)];
    let compacted = run_compactor(updates);
    assert_set_eq(&compacted, &[]);
}

#[test]
fn test_compactor_reversed_cancellation() {
    let updates = vec![Update::Remove(1), Update::Add(1)];
    let compacted = run_compactor(updates);
    assert_set_eq(&compacted, &[]);
}

#[test]
fn test_compactor_multi_add() {
    let updates = vec![Update::Add(1), Update::Add(1), Update::Remove(1)];
    let compacted = run_compactor(updates);
    assert_set_eq(&compacted, &[Update::Add(1)]);
}

#[test]
fn test_compactor_multi_remove() {
    let updates = vec![Update::Add(1), Update::Remove(1), Update::Remove(1)];
    let compacted = run_compactor(updates);
    assert_set_eq(&compacted, &[Update::Remove(1)]);
}

#[test]
fn test_compactor_multiple_items() {
    let updates = vec![
        Update::Add(1),
        Update::Add(2),
        Update::Remove(1),
        Update::Add(3),
        Update::Add(2),
        Update::Remove(2),
    ];
    let compacted = run_compactor(updates);
    assert_set_eq(&compacted, &[Update::Add(2), Update::Add(3)]);
}

#[test]
fn test_compactor_no_cancellations() {
    let updates = vec![Update::Add(1), Update::Add(2), Update::Remove(3)];
    let compacted = run_compactor(updates.clone());
    assert_set_eq(&compacted, &updates);
}

#[test]
fn test_compactor_empty_input() {
    let updates: Vec<Update<i32>> = vec![];
    let compacted = run_compactor(updates);
    assert_set_eq(&compacted, &[]);
}