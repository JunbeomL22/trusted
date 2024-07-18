use std::sync::atomic::{
    AtomicU16, 
    AtomicPtr,
    Ordering,
};

#[derive(Debug)]
pub struct UpdateStatus {
    pub status: [AtomicU16; 2],
}

impl Default for UpdateStatus {
    fn default() -> Self { 
        Self {status: [AtomicU16::new(0), AtomicU16::new(0)] }
    }
}

/// writer update data from the first data
/// if a reader still reading the data (i.e., self.status.status1 > 0), then the writer will update the second data
/// when writer wants to update a data, it keeps reading the status until the status is 0
pub struct SharedData<T> {
    data: [AtomicPtr<T>; 2],
    status: UpdateStatus,
}

impl<T> SharedData<T> {
    pub fn new(data1: *mut T, data2: *mut T) -> Self {
        Self {
            data: [AtomicPtr::new(data1), AtomicPtr::new(data2)],
            status: UpdateStatus::default(),
        }
    }

    pub fn updatable_place(&self) -> usize {
        loop {
            let status1 = self.status.status[0].load(Ordering::Acquire);
            if status1 == 0 {
                return 0;
            }
            let status2 = self.status.status[1].load(Ordering::Acquire);
            if status2 == 0 {
                return 1;
            }
        }
    }

    pub fn updated(&self) -> Option<(usize, *mut T)> {
        let status1 = self.status.status[0].load(Ordering::Acquire);
        if status1 > 0 {
            return Some((0, self.data[0].load(Ordering::Relaxed)));
        }
        let status2 = self.status.status[1].load(Ordering::Acquire);
        if status2 > 0 {
            return Some((1, self.data[1].load(Ordering::Relaxed)));
        }
        None
    }

    pub fn notify_consumption_completion(&self, place: usize) {
        self.status.status[place].fetch_sub(1, Ordering::Release);
    }
        
}

impl<T> Drop for SharedData<T> {
    fn drop(&mut self) {
        let data1 = self.data[0].load(Ordering::Acquire);
        let data2 = self.data[1].load(Ordering::Acquire);
        if !data1.is_null() {
            unsafe {
                let _ = Box::from_raw(data1);
            }
        }
        if !data2.is_null() {
            unsafe {
                let _ = Box::from_raw(data2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_data() {
        let data1 = Box::into_raw(Box::new(1));
        let data2 = Box::into_raw(Box::new(2));
        let shared_data = SharedData::new(data1, data2);
        assert_eq!(shared_data.updatable_place(), 0);
        shared_data.status.status[0].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 1);
        shared_data.status.status[1].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 0);
        shared_data.status.status[0].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 1);
        shared_data.status.status[1].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 0);
        shared_data.status.status[0].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 1);
        shared_data.status.status[1].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 0);
        shared_data.status.status[0].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 1);
        shared_data.status.status[1].store(1, Ordering::Release);
        assert_eq!(shared_data.updatable_place(), 0);
    }
}