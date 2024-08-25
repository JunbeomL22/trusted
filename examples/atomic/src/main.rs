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
use trading_engine::data::{
    krx::derivative_quote::IFMSRPD0034,
    quote::QuoteSnapshot,
};
use trading_engine::types::timestamp::DateUnixNanoGenerator;
use anyhow::Result;
use time::macros::date;
use trading_engine::feature::tick::SimpleQuotes;


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

fn share_derivative_quotes(
    iterations: usize,
    num_thrds: usize,
) -> Result<()> {
    let data_buffer = QuoteSnapshot::with_quote_level(5);   

    let mut workers = vec![];
    for i in 0..num_thrds {
        workers.push(WorkerId::new(i as IdType)?);
    }
    let arc_workers = Arc::new(workers.clone());
    let spin_queue = DataSpinQueue::new(data_buffer, workers)?;
    let arc_spin_queue = Arc::new(spin_queue);
    
    let arc_clone = arc_spin_queue.clone();

    let start_time = get_unix_nano();
    let sender = std::thread::spawn(move || {
        // set core affinity
        core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
        let tr_code = IFMSRPD0034::default();
        let date_gen = &mut DateUnixNanoGenerator::from(date!(2023-12-30));
        let mut test_data_vec = b"B602F        G140KR4106V30004000020104939405656001379.70001379.500000000030000000030000300003001379.80001379.400000000040000000040000400004001379.90001379.300000000070000000050000600005001380.00001379.200000000050000000070000500007001380.10001379.1000000000500000000500005000050000009020000025920031700642000000.00000000000".to_vec();
        test_data_vec.push(255);
        let test_data = test_data_vec.as_slice();
        let mut i = 1;
        loop {
            if let Some(buffer) = arc_clone.get_mut() {
                let buffer_ref = unsafe {&mut *buffer};
                tr_code.to_quote_snapshot_buffer(test_data, buffer_ref, date_gen).unwrap();
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
        

        let handle = std::thread::spawn(move || {
            core_affinity::set_for_current(core_affinity::CoreId { id: id+1 as usize });
            let mut quotes = SimpleQuotes::default();
            let mut count = 1;
            loop {
                if let Some(data) = reader_spin_queue.get(&worker_id) {
                    quotes.update_quote_snapshot(unsafe {&*data}).unwrap();
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

fn main () {
    let iterations = 5_000_000;
    let num_thrds = 5;

    //println!("arc mutex");
    //arc_mutex_test(iterations, num_thrds);

    //println!("data spin queue");
    //data_spin_queue(iterations, num_thrds).unwrap();

    println!("share derivative quotes");
    share_derivative_quotes(iterations, num_thrds).unwrap();
}