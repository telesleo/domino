use std::time::Duration;

use crate::{Node, Outcome};

pub struct Interval<N> {
    duration: Duration,
    node: N,
    elapsed: Duration,
}

impl<N> Interval<N> {
    pub fn new(duration: Duration, node: N) -> Self {
        Self {
            duration,
            node,
            elapsed: Duration::ZERO,
        }
    }
}

impl<C, N> Node<C> for Interval<N>
where
    N: Node<C>,
{
    fn tick(&mut self, delta: Duration, context: &mut C) -> Outcome {
        self.elapsed += delta;

        while self.elapsed >= self.duration {
            self.elapsed -= self.duration;
            return self.node.tick(self.duration, context);
        }

        Outcome::pending()
    }

    fn reset(&mut self) {
        self.node.reset();
        self.elapsed = Duration::ZERO;
    }
}

#[macro_export]
macro_rules! interval {
    ($duration: expr, $node: expr) => {
        $crate::nodes::Interval::new($duration, $node)
    };
}
