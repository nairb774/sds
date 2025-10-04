use sds::operator::Operator;
use sds::operators::filter::Filter;
use sds::update::Update;

#[test]
fn test_stateless_filter_operator() {
    let mut filter = Filter::new(|x: &i32| *x % 2 == 0);

    // A mix of even and odd numbers for both Add and Remove
    let changes = vec![
        Update::Add(1),
        Update::Add(2),
        Update::Remove(3),
        Update::Remove(4),
        Update::Add(6),
    ];
    let output = filter.process_changes(&changes);

    // Only the updates with even numbers should pass through
    assert_eq!(
        output,
        vec![Update::Add(2), Update::Remove(4), Update::Add(6)]
    );

    // Test with no items matching the predicate
    let changes = vec![Update::Add(1), Update::Remove(3)];
    let output = filter.process_changes(&changes);
    assert!(output.is_empty());

    // Test with an empty input
    let changes: Vec<Update<i32>> = vec![];
    let output = filter.process_changes(&changes);
    assert!(output.is_empty());
}