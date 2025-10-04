use sds::operator::Operator;
use sds::operators::map::Map;
use sds::update::Update;

#[test]
fn test_map_operator() {
    // Map i32 to String
    let mut map_op = Map::new(|x: i32| x.to_string());

    let changes = vec![Update::add(1), Update::remove(2), Update::add(3)];
    let output = map_op.process_changes(changes);

    assert_eq!(
        output,
        vec![
            Update::add("1".to_string()),
            Update::remove("2".to_string()),
            Update::add("3".to_string())
        ]
    );

    // Test with an empty input
    let changes: Vec<Update<i32>> = vec![];
    let output = map_op.process_changes(changes);
    assert!(output.is_empty());
}

#[test]
fn test_map_operator_with_duplicates() {
    // A map function that can produce duplicate outputs (e.g., integer division)
    let mut map_op = Map::new(|x: i32| x / 2);

    let changes = vec![Update::add(2), Update::add(3), Update::remove(5)];
    let output = map_op.process_changes(changes);

    // The output should contain duplicates, as the map operator does not compact.
    // 2 -> 1, 3 -> 1, 5 -> 2
    assert_eq!(
        output,
        vec![Update::add(1), Update::add(1), Update::remove(2)]
    );
}
