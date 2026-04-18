use std::time::Duration;

use crate::{Node, Outcome};

pub struct Once<F> {
    function: F,
}

impl<F> Once<F> {
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<C, F> Node<C> for Once<F>
where
    F: FnMut(),
{
    fn tick(&mut self, delta: Duration, _context: &mut C) -> Outcome {
        (self.function)();
        Outcome::success(delta)
    }

    fn reset(&mut self) {}
}

#[macro_export]
macro_rules! once {
    ($function:expr) => {
        $crate::nodes::Once::new($function)
    };
}
