use std::time::Duration;

use crate::Status;

pub struct Outcome {
    pub status: Status,
    pub remaining: Duration,
}

impl Outcome {
    pub const fn pending() -> Self {
        Self {
            status: Status::Pending,
            remaining: Duration::ZERO,
        }
    }
    
    pub const fn success(remaining: Duration) -> Self {
        Self {
            status: Status::Success,
            remaining,
        }
    }

    pub const fn failure() -> Self {
        Self {
            status: Status::Failure,
            remaining: Duration::ZERO,
        }
    }
}
