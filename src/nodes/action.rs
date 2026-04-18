use std::time::Duration;

use crate::{Node, Outcome};

pub struct Action<F> {
    function: F,
}

impl<F> Action<F> {
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<C, F> Node<C> for Action<F>
where
    F: FnMut(Duration, &mut C) -> Outcome,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        (self.function)(delta, context)
    }

    fn reset(&mut self) {}
}

#[macro_export]
macro_rules! action {
    ($function:expr) => {
        $crate::nodes::Action::new($function)
    };
}
