use sds::collections::btree::BTreeSetStore;
use sds::set::Set;
use sds::update::Update;

#[test]
fn test_btree_set_store_apply_changes() {
    let mut store = BTreeSetStore::new();

    let changes = vec![
        Update::add(1),
        Update::add(2),
        Update::add(3),
        Update::add(2), // Duplicate add
    ];
    store.apply_changes(changes);

    let mut items: Vec<_> = store.iter().cloned().collect();
    items.sort();
    assert_eq!(items, vec![1, 2, 3]);

    let changes = vec![Update::remove(2), Update::remove(4)]; // Remove existing and non-existing
    store.apply_changes(changes);

    let mut items: Vec<_> = store.iter().cloned().collect();
    items.sort();
    assert_eq!(items, vec![1, 3]);
}

#[test]
fn test_btree_set_store_from_iter() {
    let store: BTreeSetStore<_> = (0..5).collect();
    let items: Vec<_> = store.iter().cloned().collect();
    assert_eq!(items, vec![0, 1, 2, 3, 4]);
}
