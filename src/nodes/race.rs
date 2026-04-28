use std::time::Duration;

use crate::{Node, Outcome, Status};

pub struct Race<L, R> {
    left: L,
    right: R,
}

impl<L, R> Race<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
        }
    }
}

impl<C, L, R> Node<C> for Race<L, R>
where
    L: Node<C>,
    R: Node<C>,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        let left_outcome = self.left.tick(delta, context);

        if left_outcome.status != Status::Pending {
            return left_outcome;
        }

        let right_outcome = self.right.tick(delta, context);
        
        if right_outcome.status != Status::Pending {
            return right_outcome;
        }

        Outcome::pending()
    }

    fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
    }
}

#[macro_export]
macro_rules! race {
    ($head:expr) => {
        $head
    };

    ($head:expr, $($tail:expr),+ $(,)?) => {
        $crate::nodes::Race::new(
            $head,
            race!($($tail),+)
        )
    };
}
