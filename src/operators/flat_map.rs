use crate::operator::Operator;
use crate::update::Update;
use std::marker::PhantomData;

pub struct FlatMap<In, Out, F, I>
where
    F: Fn(In) -> I,
    I: IntoIterator<Item = Out>,
{
    f: F,
    _phantom: PhantomData<(In, Out, I)>,
}

impl<In, Out, F, I> FlatMap<In, Out, F, I>
where
    F: Fn(In) -> I,
    I: IntoIterator<Item = Out>,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _phantom: PhantomData,
        }
    }
}

impl<In, Out, F, I> Operator<In, Out> for FlatMap<In, Out, F, I>
where
    In: Clone,
    F: Fn(In) -> I,
    I: IntoIterator<Item = Out>,
{
    fn process_changes(&mut self, input_changes: &[Update<In>]) -> Vec<Update<Out>> {
        input_changes
            .iter()
            .flat_map(|change| {
                (self.f)(change.item().clone())
                    .into_iter()
                    .map(move |item| change.as_ref().map(|_| item))
            })
            .collect()
    }
}