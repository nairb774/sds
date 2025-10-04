use crate::update::Update;

pub trait Operator<In, Out> {
    fn process_changes(&mut self, input_changes: &[Update<In>]) -> Vec<Update<Out>>;
}