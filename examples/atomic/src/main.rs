use std::sync::atomic::{
    AtomicU64,
    AtomicPtr,
    Ordering,
};
use criterion::black_box;
use std::sync::Arc;
use parking_lot::Mutex;
use trading_engine::spinqueue::{
    WorkerId,
    DataSpinQueue,
    IdType,
};

use trading_engine::{
    get_unix_nano,  
    TimeStamp,
    Real,
};
use anyhow::Result;


#[derive(Debug, Clone, Default)]
pub struct TestU64 {
    value: u64
}

static ATOMIC_U64: AtomicU64 = AtomicU64::new(0);
static ATOMIC_PTR_U64: AtomicPtr<TestU64> = AtomicPtr::new(std::ptr::null_mut());

fn crude_atomic_test(
    iterations: usize,
    num_thrds: usize,
    order: Ordering,
) {
    let mut handles = vec![];
    let start = get_unix_nano();
    for _ in 0..num_thrds {
        let handle = std::thread::spawn(move || {
            for _ in 0..iterations {
                ATOMIC_U64.fetch_add(1, order);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;
    
    println!(
        "ordering: {:?} iterations: {} num_thrds: {} elapsed_sec: {} average_nano: {}",
        order,
        iterations,
        num_thrds,
        elapsed_sec,
        average_nano_over_iteration,
    );
}

fn arc_atomic_test(
    iterations: usize,
    num_thrds: usize,
    order: Ordering,
) {
    let arc_atomic = Arc::new(AtomicU64::new(0));
    let arc_clone = arc_atomic.clone();
    let mut handles = vec![];
    let start = get_unix_nano();
    for _ in 0..num_thrds {
        let arc_clone = arc_clone.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..iterations {
                arc_clone.fetch_add(1, order);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1000_000.0;
    
    println!(
        "ordering: {:?} iterations: {} num_thrds: {} elapsed_sec: {} average_nano: {}",
        order,
        iterations,
        num_thrds,
        elapsed_sec,
        average_nano_over_iteration,
    );
}

pub fn static_atomic_ptr_test(
    iterations: usize,
    num_thrds: usize,
    order: Ordering,
) {
    let mut handles = vec![];
    let start = get_unix_nano();
    for _ in 0..num_thrds {
        let handle = std::thread::spawn(move || {
            for _ in 0..iterations {
                let new_ptr = Box::into_raw(Box::new(TestU64 {value: 1}));
                ATOMIC_PTR_U64.store(new_ptr, order);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;
    
    println!(
        "ordering: {:?} iterations: {} num_thrds: {} elapsed_sec: {} average_nano: {}",
        order,
        iterations,
        num_thrds,
        elapsed_sec,
        average_nano_over_iteration,
    );
}

fn arc_atomic_ptr_test(
    iterations: usize,
    num_thrds: usize,
    order: Ordering,
) {
    let arc_atomic = Arc::new(AtomicPtr::new(std::ptr::null_mut()));
    let arc_clone = arc_atomic.clone();
    let mut handles = vec![];
    let start = get_unix_nano();
    for _ in 0..num_thrds {
        let arc_clone = arc_clone.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..iterations {
                let new_ptr = Box::into_raw(Box::new(TestU64 {value: 1}));
                arc_clone.store(new_ptr, order);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;
    
    println!(
        "ordering: {:?} iterations: {} num_thrds: {} elapsed_sec: {} average_nano: {}",
        order,
        iterations,
        num_thrds,
        elapsed_sec,
        average_nano_over_iteration,
    );
}

fn arc_mutex_test(
    iterations: usize,
    num_thrds: usize,
) {
    
    let arc_mutex = Arc::new(Mutex::new(0));
    let arc_clone = arc_mutex.clone();
    let mut handles = vec![];
    let start = get_unix_nano();
    for _ in 0..num_thrds {
        let arc_clone = arc_clone.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..iterations {
                let mut data = arc_clone.lock();
                *data += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;
    
    println!(
        "ordering: {:?} iterations: {} num_thrds: {} elapsed_sec: {} average_nano: {}",
        "not given",
        iterations,
        num_thrds,
        elapsed_sec,
        average_nano_over_iteration,
    );
}

fn data_spin_queue(
    iterations: usize,
    num_thrds: usize,
) -> Result<()> {
    let data = TestU64 {value: 0};
    let mut workers = vec![];
    for i in 0..num_thrds {
        workers.push(WorkerId::new(i as IdType)?);
    }
    let arc_workers = Arc::new(workers.clone());
    let spin_queue = DataSpinQueue::new(data, workers)?;
    let arc_spin_queue = Arc::new(spin_queue);
    
    let arc_clone = arc_spin_queue.clone();
    
    let start_time = get_unix_nano();
    let sender = std::thread::spawn(move || {
        // set core affinity
        core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
        let mut i = 1;
        loop {
            if let Some(data) = arc_clone.get_mut() {
                unsafe { (*data).value = i as u64; }
                arc_clone.notify_all_workers();
                i += 1;
                if i > iterations {
                    break;
                }
            }
        }
    });

    let mut handles = vec![];
    for id in 0..num_thrds {
        let reader_spin_queue = arc_spin_queue.clone();
        let worker_id = arc_workers[id].clone();
        let mut move_value = TestU64::default();
        let handle = std::thread::spawn(move || {
            core_affinity::set_for_current(core_affinity::CoreId { id: id+1 as usize });
            let mut count = 1;
            loop {
                if let Some(data) = reader_spin_queue.get(&worker_id) {
                    move_value.value = unsafe { (*data).value};
                    reader_spin_queue.work_done(&worker_id);
                    count += 1;
                    if count > iterations {
                        break;
                    }
                }
            }
            //println!("worker_id: {:?}", sum);
        });
        handles.push(handle);
    }

    sender.join().unwrap();
    for handle in handles {
        handle.join().unwrap();
    }
    
    let end_time = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start_time};
    let end_stamp = TimeStamp {stamp: end_time};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;
    let nano_diff = end_time - start_time;
    
    println!(
        "ordering: {:?} iterations: {} num_thrds: {} elapsed_sec: {} average_nano: {}",
        "not given",
        iterations,
        num_thrds,
        elapsed_sec,
        average_nano_over_iteration,
    );

    Ok(())    
}

fn only_mutate_test(
    iterations: usize,
) -> Result<()> {
    let data = TestU64 {value: 0};
    let mut workers = vec![];
    for i in 0..5 {
        workers.push(WorkerId::new(i as IdType)?);
    }
    let arc_workers = Arc::new(workers.clone());
    let spin_queue = DataSpinQueue::new(data, workers)?;
    let arc_spin_queue = Arc::new(spin_queue);
    

    let start = get_unix_nano();
    let arc_clone = arc_spin_queue.clone();
    
    let sender = std::thread::spawn(move || {
        // set core affinity
        core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
        let mut i = 1;
        loop {
            let check = arc_clone.ready_to_update();
            let ptr = arc_clone.data_buffer.load(Ordering::Acquire);
            unsafe { (*ptr).value = black_box(i as u64); }
            arc_clone.notify_all_workers();
            i += 1;
            if i > iterations {
                break;
            }
        }
    });

    sender.join().unwrap();
    
    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;

    println!(
        "ordering: {:?} iterations: {} elapsed_sec: {} average_nano: {}",
        "not given",
        iterations,
        elapsed_sec,
        average_nano_over_iteration,
    );

    Ok(())
}

fn only_receive(
    iterations: usize,
    num_thrds: usize,
) -> Result<()> {
    let data = TestU64 {value: 0};
    let mut workers = vec![];
    for i in 0..num_thrds {
        workers.push(WorkerId::new(i as IdType)?);
    }
    let arc_workers = Arc::new(workers.clone());
    let spin_queue = DataSpinQueue::new(data, workers)?;
    let arc_spin_queue = Arc::new(spin_queue);
    

    let start = get_unix_nano();
    let arc_clone = arc_spin_queue.clone();
    
    let mut handles = vec![];
    for id in 0..num_thrds {
        let reader_spin_queue = arc_spin_queue.clone();
        let worker_id = arc_workers[id].clone();
        let mut move_value = TestU64::default();
        let handle = std::thread::spawn(move || {
            core_affinity::set_for_current(core_affinity::CoreId { id: id as usize });
            let mut count = 1;
            loop {
                let ptr = reader_spin_queue.data_buffer.load(Ordering::Acquire);
                move_value.value = unsafe { (*ptr).value};
                reader_spin_queue.work_done(&worker_id);
                count += 1;
                if count > iterations {
                    break;
                }
            }
            //println!("worker_id: {:?}", sum);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    let end = get_unix_nano();
    let st_stamp = TimeStamp {stamp: start};
    let end_stamp = TimeStamp {stamp: end};
    let elapsed_sec = end_stamp.diff_in_secs(st_stamp);
    let average_milli_over_iteration = elapsed_sec / ((iterations / 1000)) as Real;
    let average_nano_over_iteration = average_milli_over_iteration * 1_000_000.0;
    
    println!(
        "ordering: {:?} iterations: {} elapsed_sec: {} average_nano: {}",
        "not given",
        iterations,
        elapsed_sec,
        average_nano_over_iteration,
    );

    Ok(())    
}

fn main () {
    let iterations = 100_000;
    let num_thrds = 4;
    /*
    println!("static crude atomic");
    crude_atomic_test(iterations, num_thrds, Ordering::Relaxed);
    crude_atomic_test(iterations, num_thrds, Ordering::Acquire);
    crude_atomic_test(iterations, num_thrds, Ordering::Release);
    crude_atomic_test(iterations, num_thrds, Ordering::AcqRel);
    crude_atomic_test(iterations, num_thrds, Ordering::SeqCst);

    println!("arc atomic");
    arc_atomic_test(iterations, num_thrds, Ordering::Relaxed);
    arc_atomic_test(iterations, num_thrds, Ordering::Acquire);
    arc_atomic_test(iterations, num_thrds, Ordering::Release);
    arc_atomic_test(iterations, num_thrds, Ordering::AcqRel);
    arc_atomic_test(iterations, num_thrds, Ordering::SeqCst);

    println!("static atomic ptr");
    static_atomic_ptr_test(iterations, num_thrds, Ordering::Relaxed);
    //static_atomic_ptr_test(iterations, num_thrds, Ordering::Acquire);
    static_atomic_ptr_test(iterations, num_thrds, Ordering::Release);
    //static_atomic_ptr_test(iterations, num_thrds, Ordering::AcqRel);
    static_atomic_ptr_test(iterations, num_thrds, Ordering::SeqCst);

    println!("arc atomic ptr");
    arc_atomic_ptr_test(iterations, num_thrds, Ordering::Relaxed);
    //arc_atomic_ptr_test(iterations, num_thrds, Ordering::Acquire);
    arc_atomic_ptr_test(iterations, num_thrds, Ordering::Release);
    //arc_atomic_ptr_test(iterations, num_thrds, Ordering::AcqRel);
    arc_atomic_ptr_test(iterations, num_thrds, Ordering::SeqCst);
     */
    println!("arc mutex");
    arc_mutex_test(iterations, num_thrds);

    println!("data spin queue");
    data_spin_queue(iterations, num_thrds).unwrap();

    println!("only mutate");
    only_mutate_test(iterations).unwrap();

    println!("only receive");
    only_receive(iterations, num_thrds).unwrap();
    

}