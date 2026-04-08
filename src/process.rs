use std::time::Duration;
use super::Policy;

pub struct Process<T> {
    policy: Box<dyn Policy>,
    action: Box<dyn FnMut(Duration, &mut T)>,
    pub finished: bool,
    pub next: Vec<Process<T>>,
}

impl<T> Process<T> {
    pub fn new<P, F>(policy: P, action: F) -> Self
    where
        P: Policy + 'static,
        F: FnMut(Duration, &mut T) + 'static,
    {
        Self {
            policy: Box::new(policy),
            action: Box::new(action),
            finished: false,
            next: Vec::new(),
        }
    }

    pub fn finished(&self) -> bool {
        self.finished && self.next.is_empty()
    }

    pub fn then(&mut self, process: Process<T>) -> &mut Process<T> {
        self.next.push(process);
        self.next.last_mut().unwrap()
    }

    pub fn tick(&mut self, delta: Duration, context: &mut T) {
        if self.finished {
            self.tick_next(delta, context);
            return;
        }

        let mut remainder = Duration::ZERO;

        self.policy.tick(
            delta,
            &mut |delta| (self.action)(delta, context),
            &mut |delta| {
                self.finished = true;
                remainder = delta;
            },
        );

        if self.finished {
            self.tick_next(remainder, context);
        }
    }

    fn tick_next(&mut self, delta: Duration, context: &mut T) {
        self.next.retain_mut(|process| {
            process.tick(delta, context);
            !process.finished()
        });
    }
}
