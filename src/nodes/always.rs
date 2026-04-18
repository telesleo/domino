use std::time::Duration;

use crate::{Node, Outcome};

pub struct Always<F> {
    function: F,
}

impl<F> Always<F> {
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<C, F> Node<C> for Always<F>
where
    F: FnMut(),
{
    fn tick(&mut self, _delta: Duration, _context: &mut C) -> Outcome {
        (self.function)();
        Outcome::pending()
    }

    fn reset(&mut self) {}
}

#[macro_export]
macro_rules! always {
    ($function:expr) => {
        $crate::nodes::Always::new($function)
    };
}
