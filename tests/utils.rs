use std::num::NonZeroIsize;

use sds::update::Update;
use sds::utils::diff_iter;

#[test]
fn test_diff_iter() {
    let a = vec![('A', 2), ('B', 1), ('D', 3)];
    let b = vec![('A', 1), ('C', 4), ('D', 3)];

    let diff: Vec<_> = diff_iter(a, b).collect();

    assert_eq!(
        diff,
        vec![
            Update {
                item: 'A',
                diff: NonZeroIsize::new(-1).unwrap()
            },
            Update {
                item: 'B',
                diff: NonZeroIsize::new(-1).unwrap()
            },
            Update {
                item: 'C',
                diff: NonZeroIsize::new(4).unwrap()
            },
        ]
    );
}

#[test]
fn test_diff_iter_additions() {
    let a = vec![];
    let b = vec![('A', 1), ('B', 2)];

    let diff: Vec<_> = diff_iter(a, b).collect();

    assert_eq!(
        diff,
        vec![
            Update {
                item: 'A',
                diff: NonZeroIsize::new(1).unwrap()
            },
            Update {
                item: 'B',
                diff: NonZeroIsize::new(2).unwrap()
            },
        ]
    );
}

#[test]
fn test_diff_iter_removals() {
    let a = vec![('A', 1), ('B', 2)];
    let b = vec![];

    let diff: Vec<_> = diff_iter(a, b).collect();

    assert_eq!(
        diff,
        vec![
            Update {
                item: 'A',
                diff: NonZeroIsize::new(-1).unwrap()
            },
            Update {
                item: 'B',
                diff: NonZeroIsize::new(-2).unwrap()
            },
        ]
    );
}

#[test]
fn test_diff_iter_no_changes() {
    let a = vec![('A', 1), ('B', 2)];
    let b = vec![('A', 1), ('B', 2)];

    let diff: Vec<_> = diff_iter(a, b).collect();

    assert!(diff.is_empty());
}

#[test]
fn test_diff_iter_empty() {
    let a: Vec<(char, usize)> = vec![];
    let b: Vec<(char, usize)> = vec![];

    let diff: Vec<_> = diff_iter(a, b).collect();

    assert!(diff.is_empty());
}