use std::sync::atomic::{
    AtomicPtr,
    AtomicU16,
    Ordering,
};
use std::fmt::Debug;
use anyhow::{Result, anyhow};

pub type IdType = u16;

pub type WorkerStatus = AtomicU16;
const ID_BOUND: IdType = 15;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkerId {
    pub id: IdType, // 0 to 63
    pub id_bit: IdType, // 1 << id
    pub id_bit_neg: IdType, // !id_bit
}

impl WorkerId {
    pub fn new(id: IdType) -> Result<Self> {
        if id > ID_BOUND {
            return Err(anyhow!("WorkerId::new() id must be between 0 and 63"));
        }
        
        let id_bit = 1 << id;
        Ok(WorkerId {
            id,
            id_bit,
            id_bit_neg: !id_bit,
        })
    }

    #[inline]
    #[must_use]
    pub fn work_done_mask(&self) -> IdType { // and mask
        self.id_bit_neg
    }
}

#[derive(Debug)]
pub struct DataSpinQueue<T> 
where T: Clone + Debug + Default
{
    pub data_buffer: AtomicPtr<T>,
    worker_status: WorkerStatus, // 0 means there is nothing to work on
    full_status: IdType,
    //workers: Vec<WorkerId>, // can't be bigger than 64
    //timestamp: TimeStamp,
}

impl<T> Drop for DataSpinQueue<T> 
where 
    T: Clone + Debug + Default
{
    fn drop(&mut self) {
        unsafe {
            let _= Box::from_raw(self.data_buffer.load(Ordering::Relaxed));
        }
    }
}

impl<T> Default for DataSpinQueue<T> 
where T: Clone + Debug + Default
{
    fn default() -> Self {
        DataSpinQueue {
            data_buffer: AtomicPtr::default(),
            worker_status: WorkerStatus::new(0),
            full_status: 0,
            //workers: Vec::new(),
            //timestamp: TimeStamp::default(),
        }
    }
}

impl<T> DataSpinQueue<T> 
where T: Clone + Debug + Default
{
    pub fn new(
        data: T,
        workers: Vec<WorkerId>,
    ) -> Result<Self> {
        // there must be no duplicate worker ids
        let mut worker_ids = Vec::new();
        for worker in workers.iter() {
            if worker_ids.contains(&worker.id) {
                bail!("WorkerId::new() duplicate worker id");
            }
            worker_ids.push(worker.id);
        }

        let full_status = worker_ids.iter().fold(0, |acc, worker_id| acc | (1 << worker_id));

        Ok(DataSpinQueue {
            data_buffer: AtomicPtr::new(Box::into_raw(Box::new(data.clone()))),
            worker_status: WorkerStatus::new(0),
            full_status,
            //workers,
            //timestamp: TimeStamp {stamp: get_unix_nano()},
        })
    }

    /// If all workers are done, return the mutable pointer to the data
    /// Otherwise, None
    #[inline]
    #[must_use]
    pub fn get_mut(&self) -> Option<*mut T> {
        match self.ready_to_update() {
            true => Some(self.data_buffer.load(Ordering::Acquire)),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn ready_to_update(&self) -> bool {
        self.worker_status.load(Ordering::Acquire) == 0
    }

    #[inline]
    pub fn ready_to_work(&self, id: &WorkerId) -> bool {
        (self.worker_status.load(Ordering::Acquire) & id.id_bit) > 0
    }

    /// If worker_status's bit is zero, it means the worker is done.
    /// If the worker is done, the worker does not need to work on the data,
    /// so this function returns None. Otherwise, it returns the pointer to the data.
    #[inline]
    #[must_use]
    pub fn get(&self, id: &WorkerId) -> Option<*const T> {
        match self.ready_to_work(id) {
            true => Some(self.data_buffer.load(Ordering::Acquire)),
            _ => None,
        }
    }

    #[inline]
    // notify workers that the data is ready
    pub fn notify_all_workers(&self) {
        //let sum_of_id_bits = self.workers.iter().fold(0, |acc, worker| acc | worker.id_bit);
        self.worker_status.store(self.full_status, Ordering::Release);
    }

    #[inline]
    pub fn work_done(&self, id: &WorkerId) {
        let mask = id.work_done_mask();
        self.worker_status.fetch_and(mask, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub struct TestU64 {
        pub value: u64,
    }

    #[test]
    fn test_shared_data() -> Result<()> {
        let a = TestU64 {value: 0};
        let worker1 = WorkerId::new(0)?;
        let worker2 = WorkerId::new(3)?;
        let worker3 = WorkerId::new(6)?;
        let workers = vec![worker1.clone(), worker2.clone(), worker3.clone()];
        let shared_data = DataSpinQueue::new(
            a.clone(),
            workers.clone(),
        )?;

        let first_value = unsafe{ (*shared_data.get(&(workers[0])).unwrap()).value };
        assert_eq!(first_value, 0);

        if let Some(data) = shared_data.get_mut() {
            unsafe {
                (*data).value += 10;
            }
            shared_data.notify_all_workers();
        }
        
        let x = shared_data.get_mut();
        assert_eq!(x, None);

        let status = shared_data.worker_status.load(Ordering::Acquire);
        // check bit operation on status1
        println!("status1: {:b}", status);

        let const_data = shared_data.get(&workers[0]).unwrap();
        let const_value = unsafe { (*const_data).value };

        assert_eq!(const_value, 10);

        shared_data.work_done(&worker1);
        assert_eq!(shared_data.worker_status.load(Ordering::Acquire), 64 + 8);

        shared_data.work_done(&worker3);
        assert_eq!(shared_data.worker_status.load(Ordering::Acquire), 8);

        let is_ready = shared_data.ready_to_update();
        assert_eq!(is_ready, false);
        shared_data.work_done(&worker2);
        assert_eq!(shared_data.worker_status.load(Ordering::Acquire), 0);
        assert_eq!(shared_data.ready_to_update(), true);

        Ok(())
    }
}