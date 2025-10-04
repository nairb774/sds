use crate::operator::Operator;
use crate::update::Update;
use std::marker::PhantomData;

pub struct Map<In, Out, F>
where
    F: Fn(In) -> Out,
{
    f: F,
    _phantom: PhantomData<(In, Out)>,
}

impl<In, Out, F> Map<In, Out, F>
where
    F: Fn(In) -> Out,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

impl<In, Out, F> Operator<In, Out> for Map<In, Out, F>
where
    F: Fn(In) -> Out,
{
    fn process_changes(&mut self, input_changes: Vec<Update<In>>) -> Vec<Update<Out>> {
        input_changes
            .into_iter()
            .map(|change| change.map(|item| (self.f)(item)))
            .collect()
    }
}
