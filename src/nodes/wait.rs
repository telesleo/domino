use std::time::Duration;

use crate::{Node, Outcome};

pub struct Wait {
    duration: Duration,
    elapsed: Duration,
}

impl Wait {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
        }
    }
}

impl<C> Node<C> for Wait {
    fn tick(&mut self, delta: Duration, _context: &mut C) -> Outcome {
        self.elapsed += delta;

        if self.elapsed >= self.duration {
            let remaining = self.elapsed - self.duration;
            Outcome::success(remaining)
        } else {
            Outcome::pending()
        }
    }

    fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }
}

#[macro_export]
macro_rules! wait {
    ($duration:expr) => {
        $crate::nodes::Wait::new($duration)
    };
}
