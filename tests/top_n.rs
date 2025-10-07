use sds::operator::Operator;
use sds::operators::top_n::TopN;
use sds::update::Update;

fn assert_updates_eq(mut a: Vec<Update<i32>>, mut b: Vec<Update<i32>>) {
    a.sort();
    b.sort();
    assert_eq!(a, b);
}

#[test]
fn test_top_n_initial_fill() {
    let mut top_n = TopN::new(3);
    let changes = vec![
        Update::add(1),
        Update::add(5),
        Update::add(3),
        Update::add(2),
        Update::add(4),
    ];
    let output = top_n.process_changes(changes);
    assert_updates_eq(output, vec![Update::add(3), Update::add(4), Update::add(5)]);
}

#[test]
fn test_top_n_less_than_n_elements() {
    let mut top_n = TopN::new(5);
    let changes = vec![Update::add(1), Update::add(2), Update::add(3)];
    let output = top_n.process_changes(changes);
    assert_updates_eq(output, vec![Update::add(1), Update::add(2), Update::add(3)]);
}

#[test]
fn test_top_n_add_new_top_element() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![Update::add(1), Update::add(2), Update::add(3)]);

    let changes = vec![Update::add(4)];
    let output = top_n.process_changes(changes);
    assert_updates_eq(output, vec![Update::add(4), Update::remove(1)]);
}

#[test]
fn test_top_n_add_non_top_element() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![Update::add(2), Update::add(3), Update::add(4)]);

    let changes = vec![Update::add(1)];
    let output = top_n.process_changes(changes);
    assert!(output.is_empty());
}

#[test]
fn test_top_n_remove_top_element() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![
        Update::add(1),
        Update::add(2),
        Update::add(3),
        Update::add(4),
    ]);

    let changes = vec![Update::remove(3)];
    let output = top_n.process_changes(changes);
    assert_updates_eq(output, vec![Update::add(1), Update::remove(3)]);
}

#[test]
fn test_top_n_remove_non_top_element() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![
        Update::add(1),
        Update::add(2),
        Update::add(3),
        Update::add(4),
    ]);

    let changes = vec![Update::remove(1)];
    let output = top_n.process_changes(changes);
    assert!(output.is_empty());
}

#[test]
fn test_top_n_with_n_zero() {
    let mut top_n = TopN::new(0);
    let changes = vec![Update::add(1), Update::add(2), Update::add(3)];
    let output = top_n.process_changes(changes);
    assert!(output.is_empty());
}

#[test]
fn test_top_n_empty_input() {
    let mut top_n: TopN<i32> = TopN::new(3);
    let changes = vec![];
    let output = top_n.process_changes(changes);
    assert!(output.is_empty());
}

#[test]
fn test_top_n_remove_to_below_n() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![Update::add(1), Update::add(2), Update::add(3)]);
    let changes = vec![Update::remove(2)];
    let output = top_n.process_changes(changes);
    assert_updates_eq(output, vec![Update::remove(2)]);
}

#[test]
fn test_top_n_with_duplicates() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![Update::add(1), Update::add(5), Update::add(5)]);
    let output = top_n.process_changes(vec![Update::add(4)]);

    assert_updates_eq(output, vec![Update::add(4), Update::remove(1)]);
}

#[test]
fn test_top_n_remove_duplicate() {
    let mut top_n = TopN::new(2);
    top_n.process_changes(vec![Update::add(5), Update::add(5), Update::add(4)]);
    let output = top_n.process_changes(vec![Update::remove(5)]);
    assert_updates_eq(output, vec![Update::add(4), Update::remove(5)]);
}

#[test]
fn test_top_n_remove_last_duplicate() {
    let mut top_n = TopN::new(3);
    top_n.process_changes(vec![
        Update::add(3),
        Update::add(3),
        Update::add(2),
        Update::add(1),
    ]);
    let output = top_n.process_changes(vec![Update::remove(3)]);
    assert_updates_eq(output, vec![Update::add(1), Update::remove(3)]);
}
