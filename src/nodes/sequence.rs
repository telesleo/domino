use std::time::Duration;

use crate::{Node, Outcome, Status};

pub struct Sequence<L, R> {
    left: L,
    right: R,
    left_done: bool,
}

impl<L, R> Sequence<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            left_done: false,
        }
    }
}

impl<C, A, B> Node<C> for Sequence<A, B>
where
    A: Node<C>,
    B: Node<C>,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        if self.left_done {
            return self.right.tick(delta, context);
        }

        match self.left.tick(delta, context) {
            Outcome { status: Status::Success, remaining } => {
                self.left_done = true;
                self.right.tick(remaining, context)
            }
            outcome => outcome,
        }
    }

    fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
        self.left_done = false;
    }
}

#[macro_export]
macro_rules! sequence {
    ($head:expr) => {
        $head
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::nodes::Sequence::new(
            $head,
            sequence!($($tail),+)
        )
    };
}
