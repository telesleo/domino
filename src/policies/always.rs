use std::time::Duration;
use crate::Policy;

pub struct Always;

impl Policy for Always {
    fn tick(&mut self, delta: Duration, run: &mut dyn FnMut(Duration), _finish: &mut dyn FnMut(Duration)) {
        run(delta);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn tick() {
        let mut policy = Always;
        let delta = Duration::from_secs(1);

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        policy.tick(
            delta,
            &mut |d: Duration| {
                run_count += 1;
                assert_eq!(d, delta);
            },
            &mut |_| {
                finish_count += 1;
            }
        );

        assert_eq!(run_count, 1);
        assert_eq!(finish_count, 0);
    }

    #[test]
    fn multiple_ticks() {
        let ticks: usize = 3;
        let mut policy = Always;

        let mut run_count: usize = 0;
        let mut finish_count: usize = 0;

        for i in 0..3 {
            let delta = Duration::from_secs(i + 1);

            policy.tick(
                delta,
                &mut |d: Duration| {
                    run_count += 1;
                    assert_eq!(d, delta);
                },
                &mut |_| {
                    finish_count += 1;
                }
            );
        }

        assert_eq!(run_count, ticks);
        assert_eq!(finish_count, 0);
    }
}
