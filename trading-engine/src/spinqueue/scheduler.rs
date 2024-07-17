use std::sync::atomic::{AtomicU32, Ordering};

pub struct Scheduler {
    value: AtomicU32, // each four bytes will be a notifier
}