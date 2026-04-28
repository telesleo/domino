use std::time::Duration;

use crate::Outcome;

pub trait Node<C> {
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome;
    fn reset(&mut self);
}

impl<C, F> Node<C> for F
where
    F: FnMut(Duration, &mut C) -> Outcome,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        self(delta, context)
    }
    fn reset(&mut self) {}
}
