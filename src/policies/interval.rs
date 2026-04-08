use std::time::Duration;
use crate::Policy;

pub struct Interval {
    interval: Duration,
    elapsed: Duration,
}

impl Interval {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            elapsed: Duration::ZERO,
        }
    }
}

impl Policy for Interval {
    fn tick(&mut self, delta: Duration, run: &mut dyn FnMut(Duration), _finish: &mut dyn FnMut(Duration)) {
        self.elapsed += delta;

        while self.elapsed >= self.interval {
            self.elapsed -= self.interval;
            run(self.interval);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn elapsed_less_than_interval() {
        let delta = Duration::from_secs(1);
        let interval = Duration::from_secs(2);
        let mut policy = Interval::new(interval);

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        policy.tick(
            delta,
            &mut |_| {
                run_count += 1;
            },
            &mut |_| {
                finish_count += 1;
            }
        );

        assert_eq!(run_count, 0);
        assert_eq!(finish_count, 0);
    }

    #[test]
    fn elapsed_greater_than_interval() {
        let delta = Duration::from_secs(3);
        let interval = Duration::from_secs(2);
        let mut policy = Interval::new(interval);

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, interval);
            },
            &mut |_| {
                finish_count += 1;
            }
        );

        assert!(run_count > 0);
        assert_eq!(finish_count, 0);
    }

    #[test]
    fn elapsed_double_or_more_than_interval() {
        let delta = Duration::from_secs(6);
        let interval = Duration::from_secs(2);
        let mut policy = Interval::new(interval);

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, interval);
            },
            &mut |_| {
                finish_count += 1;
            }
        );

        let expected_run_count = (delta.as_secs() / interval.as_secs()) as usize;
        
        assert_eq!(run_count, expected_run_count);
        assert_eq!(finish_count, 0);
    }
}
