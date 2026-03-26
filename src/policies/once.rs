use std::time::Duration;
use crate::Policy;

pub struct Once;

impl Policy for Once {
    fn tick(&mut self, delta: Duration, run: &mut dyn FnMut(Duration), finish: &mut dyn FnMut(Duration)) {
        run(Duration::ZERO);
        finish(delta);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn tick() {
        let mut policy = Once;
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
    fn multiple_ticks() {
        let ticks: usize = 3;
        let delta = Duration::from_secs(1);
        let mut policy = Once;

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        for _ in 0..ticks {
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
        }

        assert_eq!(run_count, ticks);
        assert_eq!(finish_count, ticks);
    }
}
