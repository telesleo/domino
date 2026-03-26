use std::time::Duration;
use crate::Policy;

pub struct Repeat {
    times: usize
}

impl Repeat {
    pub fn new(times: usize) -> Self {
        Self {
            times,
        }
    }
}

impl Policy for Repeat {
    fn tick(&mut self, delta: Duration, run: &mut dyn FnMut(Duration), finish: &mut dyn FnMut(Duration)) {
        for _ in 0..self.times {
            run(Duration::ZERO)
        }
        finish(delta);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn repeat_once() {
        let mut policy = Repeat::new(1);

        let delta = Duration::from_secs(1);

        let mut run_count = 0;
        let mut finish_count = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, Duration::ZERO);
            },
            &mut |d: Duration| {
                finish_count += 1;
                assert_eq!(d, delta);
            }
        );

        assert_eq!(run_count, 1);
        assert_eq!(finish_count, 1);
    }

    #[test]
    fn repeat_three_times() {
        let times: usize = 3;
        let delta = Duration::from_secs(1);
        let mut policy = Repeat::new(times);

        let mut run_count = 0;
        let mut finish_count = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, Duration::ZERO);
            },
            &mut |d: Duration| {
                finish_count += 1;
                assert_eq!(d, delta);
            }
        );

        assert_eq!(run_count, times);
        assert_eq!(finish_count, 1);
    }

    #[test]
    fn multiple_ticks() {
        let ticks = 3;
        let times = 2;
        let delta = Duration::from_secs(1);
        let mut policy = Repeat::new(times);

        let mut run_count = 0;
        let mut finish_count = 0;

        for _ in 0..ticks {
            policy.tick(
                delta,
                &mut |_| {
                    run_count += 1
                },
                &mut |_| {
                    finish_count += 1
                }
            );
        }

        assert_eq!(run_count, ticks * times);
        assert_eq!(finish_count, ticks);
    }
}
