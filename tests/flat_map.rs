use sds::operator::Operator;
use sds::operators::flat_map::FlatMap;
use sds::update::Update;

#[test]
fn test_flat_map_operator() {
    // Map an i32 to an iterator of i32s (e.g., n -> [n, n+1])
    let mut flat_map_op = FlatMap::new(|x: i32| (x..x + 2));

    let changes = vec![Update::add(1), Update::remove(3)];
    let output = flat_map_op.process_changes(&changes);

    assert_eq!(
        output,
        vec![
            Update::add(1),
            Update::add(2),
            Update::remove(3),
            Update::remove(4)
        ]
    );

    // Test with a function that returns an empty iterator
    let mut flat_map_op_empty = FlatMap::new(|_x: i32| std::iter::empty::<i32>());
    let changes = vec![Update::add(10)];
    let output = flat_map_op_empty.process_changes(&changes);
    assert!(output.is_empty());

    // Test with an empty input
    let changes: Vec<Update<i32>> = vec![];
    let output = flat_map_op.process_changes(&changes);
    assert!(output.is_empty());
}
