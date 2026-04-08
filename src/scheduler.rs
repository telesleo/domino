use std::time::Duration;
use super::process::Process;

pub struct Scheduler<T> {
    processes: Vec<Process<T>>,
}

impl<T> Scheduler<T> {
    pub fn new() -> Self {
        Self { processes: Vec::new() }
    }

    pub fn finished(&self) -> bool {
        self.processes.is_empty()
    }

    pub fn add(&mut self, process: Process<T>) -> &mut Process<T> {
        self.processes.push(process);
        self.processes.last_mut().unwrap()
    }

    pub fn tick(&mut self, delta: Duration, context: &mut T) {
        self.processes.retain_mut(|process| {
            process.tick(delta, context);
            !process.finished()
        });
    }
}
