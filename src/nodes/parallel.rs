use std::time::Duration;

use crate::{Node, Outcome, Status};

pub struct Parallel<L, R> {
    left: L,
    right: R,
    left_done: bool,
    right_done: bool,
}

impl<L, R> Parallel<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            left_done: false,
            right_done: false,
        }
    }
}

impl<C, L, R> Node<C> for Parallel<L, R>
where
    L: Node<C>,
    R: Node<C>,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        let mut remaining = delta;

        if !self.left_done {
            let outcome = self.left.tick(delta, context);
            match outcome.status {
                Status::Failure => return outcome,
                Status::Success => {
                    self.left_done = true;
                    remaining = remaining.min(outcome.remaining);
                }
                Status::Pending => {}
            }
        }

        if !self.right_done {
            let outcome = self.right.tick(delta, context);
            match outcome.status {
                Status::Failure => return outcome,
                Status::Success => {
                    self.right_done = true;
                    remaining = remaining.min(outcome.remaining);
                }
                Status::Pending => {}
            }
        }

        if self.left_done && self.right_done {
            Outcome::success(remaining)
        } else {
            Outcome::pending()
        }
    }

    fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
        self.left_done = false;
        self.right_done = false;
    }
}

#[macro_export]
macro_rules! parallel {
    ($head:expr) => {
        $head
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::nodes::Parallel::new(
            $head,
            parallel!($($tail),+)
        )
    };
}
