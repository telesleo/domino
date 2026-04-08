use std::time::Duration;

pub trait Policy {
    fn tick(&mut self, delta: Duration, run: &mut dyn FnMut(Duration), finish: &mut dyn FnMut(Duration));
}
