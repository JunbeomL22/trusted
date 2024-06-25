use trading_engine::{
    logger::logger::{
        LogLevel,
        TimeZone,
        Logger,
    },
    timer,
    log_info,
    info,
    debug,
};
use trading_engine::utils::timer::get_unix_nano;

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct {
    a: i32,
    b: f64,
    //c: String,
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
        .with_file("logs", "test")?
        .with_console_report(false)
        .with_max_log_level(LogLevel::Info)
        .with_timezone(TimeZone::Local)
        .launch();

    let iteration = 1_000_000;
    
    let start = crate::timer::get_unix_nano();
    
    let test_struct = TestStruct {
        a: 1,
        b: 3.14,
        //c: "hello".to_string(),
    };

    let _guard = Guard {
        ts: get_unix_nano(),
    };

    for _ in 0..iteration {
        let test_clone = test_struct.clone();
        log_info!("test", struct_log = test_clone);
    }

    let end = crate::timer::get_unix_nano();

    let elapsed = end - start;
    let elapsed_as_seconds = elapsed as f64 / 1_000_000_000.0;
    let elapsed_average = elapsed as f64 / iteration as f64;

    //println!("elapsed: {:.3}s, average: {:.0}ns", elapsed_as_seconds, elapsed_average);
    debug!("elapsed: {:.3}s, average: {:.0}ns", elapsed_as_seconds, elapsed_average);

    Ok(())
}