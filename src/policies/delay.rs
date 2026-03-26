use std::time::Duration;
use crate::Policy;

pub struct Delay {
    delay: Duration,
    elapsed: Duration,
}

impl Delay {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            elapsed: Duration::ZERO,
        }
    }
}

impl Policy for Delay {
    fn tick(&mut self, delta: Duration, run: &mut dyn FnMut(Duration), finish: &mut dyn FnMut(Duration)) {
        self.elapsed += delta;
        
        if self.elapsed >= self.delay {
            run(self.delay);
            finish(self.elapsed - self.delay);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn elapsed_less_than_delay() {
        let delta = Duration::from_secs(1);
        let delay = Duration::from_secs(2);
        let mut policy = Delay::new(delay);
        
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
    fn elpased_equal_to_delay() {
        let delta = Duration::from_secs(2);
        let delay = Duration::from_secs(2);
        let mut policy = Delay::new(delay);

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, delta);
            },
            &mut |d: Duration| {
                finish_count += 1;
                assert_eq!(d, delta - delay);
            }
        );

        assert_eq!(run_count, 1);
        assert_eq!(finish_count, 1);
    }

    #[test]
    fn elapsed_greater_than_delay() {
        let delta = Duration::from_secs(3);
        let delay = Duration::from_secs(2);
        let mut policy = Delay::new(delay);

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, delay);
            },
            &mut |d: Duration| {
                finish_count += 1;
                assert_eq!(d, delta - delay);
            }
        );

        assert!(run_count > 0);
        assert_eq!(finish_count, 1);
    }

    #[test]
    fn multiple_ticks() {
        let ticks: usize = 3;

        let delta = Duration::from_secs(1);
        let delay = Duration::from_secs(2);
        let mut policy = Delay::new(delay);

        let mut elapsed = Duration::ZERO;

        for _ in 0..ticks {
            elapsed += delta;

            let mut run_count: usize = 0;
            let mut finish_count: usize = 0;

            policy.tick(
                delta,
                &mut |d: Duration| {
                    run_count += 1;
                    assert_eq!(d, delay);
                },
                &mut |d: Duration| {
                    finish_count += 1;
                    assert_eq!(d, elapsed - delay);
                },
            );

            let expect_run: bool = elapsed >= delay;

            assert_eq!(run_count > 0, expect_run);
            assert_eq!(finish_count > 0, expect_run);
        }
    }
}
