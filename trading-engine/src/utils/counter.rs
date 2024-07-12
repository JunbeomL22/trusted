// Atomic counter
use anyhow::{anyhow, Result};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

#[derive(Debug)]
pub struct CounterUsize {
    start: usize,
    end: usize,
    counter: AtomicUsize,
}

impl CounterUsize {
    pub fn new(start: usize, end: usize) -> Self {
        CounterUsize {
            start,
            end,
            counter: AtomicUsize::new(start),
        }
    }

    pub fn next(&self) -> Result<usize> {
        let res = self.counter.fetch_add(1, Ordering::SeqCst);
        if res > self.end {
            let lazy_error = || {
                anyhow!(
                    "CounterUsize: counter overflow, start: {}, end: {}",
                    self.start,
                    self.end
                )
            };
            Err(lazy_error())
        } else {
            Ok(res)
        }
    }
}

#[derive(Debug)]
pub struct CounterU64 {
    start: u64,
    end: u64,
    counter: AtomicU64,
}

impl CounterU64 {
    pub fn new(start: u64, end: u64) -> Self {
        CounterU64 {
            start,
            end,
            counter: AtomicU64::new(start),
        }
    }

    pub fn next(&self) -> Result<u64> {
        let res = self.counter.fetch_add(1, Ordering::SeqCst);
        if res > self.end {
            let lazy_error = || {
                anyhow!(
                    "CounterU64: counter overflow, start: {}, end: {}",
                    self.start,
                    self.end
                )
            };
            Err(lazy_error())
        } else {
            Ok(res)
        }
    }
}

#[derive(Debug)]
pub struct UnboundedCounterUsize {
    counter: AtomicUsize,
}

impl UnboundedCounterUsize {
    pub fn new(_start: usize) -> Self {
        UnboundedCounterUsize {
            counter: AtomicUsize::new(_start),
        }
    }

    pub fn next(&self) -> usize {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }
}

#[derive(Debug)]
pub struct UnboundedCounterU64 {
    counter: AtomicU64,
}

impl UnboundedCounterU64 {
    pub fn new(_start: u64) -> Self {
        UnboundedCounterU64 {
            counter: AtomicU64::new(_start),
        }
    }

    pub fn next(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_counter_usize() -> Result<()> {
        let counter = CounterUsize::new(0, 10);
        assert_eq!(counter.next()?, 0);
        assert_eq!(counter.next()?, 1);
        assert_eq!(counter.next()?, 2);
        assert_eq!(counter.next()?, 3);

        Ok(())
    }

    #[test]
    fn test_counter_u64() -> Result<()> {
        let counter = CounterU64::new(3, 5);
        assert_eq!(counter.next()?, 3);
        assert_eq!(counter.next()?, 4);
        assert_eq!(counter.next()?, 5);

        Ok(())
    }

    #[test]
    fn test_counter_usize_threaded() -> Result<()> {
        let counter = Arc::new(CounterUsize::new(0, usize::MAX));
        let mut handles = vec![];
        for _ in 0..3 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..30000 {
                    let _ = counter_clone.next();
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(counter.next()?, 90000);

        Ok(())
    }

    #[test]
    fn test_counter_u64_threaded() -> Result<()> {
        let counter = Arc::new(UnboundedCounterU64::new(0));
        let mut handles = vec![];
        for _ in 0..3 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..30000 {
                    counter_clone.next();
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(counter.next(), 90000);

        Ok(())
    }
}
