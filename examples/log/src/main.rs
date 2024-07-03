use trading_engine::{
    logger::{
        LogLevel,
        TimeZone,
        Logger,
    },
    timer,
    log_info,
    info,
};
use trading_engine::utils::timer::get_unix_nano;
use std::thread::sleep;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct {
    a: i32,
    b: f64,
    //c: String,
}

impl Default for TestStruct {
    fn default() -> Self {
        TestStruct {
            a: 0,
            b: 0.0,
            //c: "".to_string(),
        }
    }
}

struct Guard {
    ts: u64,
}

impl Drop for Guard {
    fn drop(&mut self) {
        let dropping = get_unix_nano();
        let elapsed = dropping - self.ts;
        let elapsed_as_seconds = elapsed as f64 / 1_000_000.0;
        
        println!("dropping guard: {:.3}ms", elapsed_as_seconds);
    }
}

fn main() -> Result<()> {
    let _guard = Logger::initialize()
        .with_file("logs", "message")?
        .with_console_report(false)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    let iteration = 100_000;
    
    let test_struct = TestStruct::default();

    for _ in 0..5 {
        info!("warm up")
    }

    let __guard = Guard {
        ts: get_unix_nano(),
    };

    let start = crate::timer::get_unix_nano();

    for _ in 0..iteration {
        let test_clone = test_struct.clone();
        log_info!("test", struct_log = test_clone);
    }

    let end = crate::timer::get_unix_nano();

    let elapsed = end - start;
    let elapsed_as_seconds = elapsed as f64 / 1_000_000_000.0;
    let elapsed_average = elapsed as f64 / iteration as f64;

    info!("elapsed: {:.3}s, average: {:.0}ns", elapsed_as_seconds, elapsed_average);
    println!("elapsed: {:.3}s, average: {:.0}ns", elapsed_as_seconds, elapsed_average);

    Ok(())
}
