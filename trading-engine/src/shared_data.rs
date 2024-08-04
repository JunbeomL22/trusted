use std::fmt::Debug;
use anyhow::{Result, bail};
use std::sync::{
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};

pub type IdType = u32;
pub type WorkerStatus = u32;

const ID_BOUND: IdType = 31;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkerId {
    pub id: IdType, // 0 to 63
    pub id_bit: IdType, // 1 << id
    pub id_bit_neg: IdType, // !id_bit
}

impl WorkerId {
    pub fn new(id: IdType) -> Result<Self> {
        if id > ID_BOUND {
            bail!("WorkerId::new() id must be between 0 and 63");
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
pub struct SharedData<T>
where T: Clone + Debug + Default
{
    data: RwLock<T>,
    work_status: RwLock<WorkerStatus>, // 0 means there is nothing to work on
    full_status: IdType,
    //workers: Vec<WorkerId>, // can't be bigger than 64
}

impl<T> SharedData<T> 
where 
    T: Clone + Debug + Default
{
    pub fn new(data: T, workers: Vec<WorkerId>) -> Self {
        let full_status = workers.iter().fold(0, |acc, worker| acc | worker.id_bit);
        SharedData {
            data: RwLock::new(data),
            work_status: RwLock::new(0),
            full_status,
        }
    }

    #[inline]
    #[must_use]
    pub fn ready_to_update(&self) -> bool {
        *self.work_status.read().unwrap() == 0
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&self) -> Option<RwLockWriteGuard<T>> {
        match self.ready_to_update() {
            true => Some(self.data.write().unwrap()),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub fn ready_to_read(&self, worker_id: &WorkerId) -> bool {
        *self.work_status.read().unwrap() & worker_id.id_bit == 0
    }

    #[inline]
    #[must_use]
    pub fn get(&self, worker_id: &WorkerId) -> Option<RwLockReadGuard<T>> {
        match self.ready_to_read(worker_id) {
            true => Some(self.data.read().unwrap()),
            _ => None,
        }
    }

    #[inline]
    pub fn notify_all_workers(&self) {
        *self.work_status.write().unwrap() = self.full_status;
    }

    #[inline]
    pub fn work_done(&self, worker_id: &WorkerId) {
        let mask = worker_id.work_done_mask();
        *self.work_status.write().unwrap() &= mask;
    }
}