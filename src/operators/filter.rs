use crate::operator::Operator;
use crate::update::Update;
use std::marker::PhantomData;

pub struct Filter<T, F>
where
    F: Fn(&T) -> bool,
{
    predicate: F,
    _phantom: PhantomData<T>,
}

impl<T, F> Filter<T, F>
where
    F: Fn(&T) -> bool,
{
    pub fn new(predicate: F) -> Self {
        Self {
            predicate,
            _phantom: PhantomData,
        }
    }
}

impl<T, F> Operator<T, T> for Filter<T, F>
where
    F: Fn(&T) -> bool,
{
    fn process_changes(&mut self, input_changes: Vec<Update<T>>) -> Vec<Update<T>> {
        input_changes
            .into_iter()
            .filter(|change| (self.predicate)(change.item()))
            .collect()
    }
}
