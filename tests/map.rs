use sds::operator::Operator;
use sds::operators::map::Map;
use sds::update::Update;

#[test]
fn test_map_operator() {
    // Map i32 to String
    let mut map_op = Map::new(|x: i32| x.to_string());

    let changes = vec![Update::Add(1), Update::Remove(2), Update::Add(3)];
    let output = map_op.process_changes(&changes);

    assert_eq!(
        output,
        vec![
            Update::Add("1".to_string()),
            Update::Remove("2".to_string()),
            Update::Add("3".to_string())
        ]
    );

    // Test with an empty input
    let changes: Vec<Update<i32>> = vec![];
    let output = map_op.process_changes(&changes);
    assert!(output.is_empty());
}