use std::time::Duration;

use crate::Outcome;

pub trait Node<C> {
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome;
    fn reset(&mut self);
}
