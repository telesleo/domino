use std::time::Duration;

use crate::{Node, Outcome, Status};

pub struct Repeat<N> {
    times: u32,
    node: N,
}

impl<N> Repeat<N> {
    pub fn new(times: u32, node: N) -> Self {
        Self {
            times,
            node,
        }
    }
}

impl<C, N> Node<C> for Repeat<N>
where
    N: Node<C>,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        for _ in 0..self.times {
            let outcome = self.node.tick(delta, context);

            if outcome.status != Status::Pending {
                return outcome;
            }
        }

        Outcome::success(delta)
    }

    fn reset(&mut self) {
        self.node.reset();
    }
}

#[macro_export]
macro_rules! repeat {
    ($times: expr, $node: expr) => {
        $crate::nodes::Repeat::new($times, $node)
    };
}
