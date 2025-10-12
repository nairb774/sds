use std::iter::Peekable;
use std::num::NonZeroIsize;

use crate::update::Update;

pub fn diff_iter<T, I1, I2>(iter1: I1, iter2: I2) -> DiffIter<T, I1::IntoIter, I2::IntoIter>
where
    T: Ord,
    I1: IntoIterator<Item = (T, usize)>,
    I2: IntoIterator<Item = (T, usize)>,
{
    DiffIter {
        iter1: iter1.into_iter().peekable(),
        iter2: iter2.into_iter().peekable(),
    }
}

pub struct DiffIter<T, I1, I2>
where
    T: Ord,
    I1: Iterator<Item = (T, usize)>,
    I2: Iterator<Item = (T, usize)>,
{
    iter1: Peekable<I1>,
    iter2: Peekable<I2>,
}

impl<T, I1, I2> Iterator for DiffIter<T, I1, I2>
where
    T: Ord,
    I1: Iterator<Item = (T, usize)>,
    I2: Iterator<Item = (T, usize)>,
{
    type Item = Update<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.iter1.peek(), self.iter2.peek()) {
                (Some((item1, _)), Some((item2, _))) => {
                    use std::cmp::Ordering::*;
                    match item1.cmp(item2) {
                        Less => {
                            let (item, count) = self.iter1.next().unwrap();
                            let count_isize =
                                isize::try_from(count).expect("count value too large for isize");
                            let diff = NonZeroIsize::new(-count_isize).unwrap();
                            return Some(Update { item, diff });
                        }
                        Greater => {
                            let (item, count) = self.iter2.next().unwrap();
                            let count_isize =
                                isize::try_from(count).expect("count value too large for isize");
                            let diff = NonZeroIsize::new(count_isize).unwrap();
                            return Some(Update { item, diff });
                        }
                        Equal => {
                            let (item1, count1) = self.iter1.next().unwrap();
                            let (_item2, count2) = self.iter2.next().unwrap();
                            let count1_isize =
                                isize::try_from(count1).expect("count1 value too large for isize");
                            let count2_isize =
                                isize::try_from(count2).expect("count2 value too large for isize");
                            let diff = count2_isize - count1_isize;
                            if let Some(diff) = NonZeroIsize::new(diff) {
                                return Some(Update { item: item1, diff });
                            }
                        }
                    }
                }
                (Some(_), None) => {
                    let (item, count) = self.iter1.next().unwrap();
                    let count_isize =
                        isize::try_from(count).expect("count value too large for isize");
                    let diff = NonZeroIsize::new(-count_isize).unwrap();
                    return Some(Update { item, diff });
                }
                (None, Some(_)) => {
                    let (item, count) = self.iter2.next().unwrap();
                    let count_isize =
                        isize::try_from(count).expect("count value too large for isize");
                    let diff = NonZeroIsize::new(count_isize).unwrap();
                    return Some(Update { item, diff });
                }
                (None, None) => return None,
            }
        }
    }
}
