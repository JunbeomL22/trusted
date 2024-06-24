use trading_engine::{
    logger::logger::{
        LogLevel,
        TimeZone,
        Logger,
    },
    timer,
    log_info,
    info,
};

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestStruct {
    a: i32,
    b: f64,
    //c: String,
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

    for _ in 0..iteration {
        let test_clone = test_struct.clone();
        log_info!("test", struct_log = test_clone);
    }

    let end = crate::timer::get_unix_nano();

    let elapsed = end - start;
    let elapsed_as_seconds = elapsed as f64 / 1_000_000_000.0;
    let elapsed_average = elapsed as f64 / iteration as f64;

    println!("elapsed: {:.3}s, average: {:.0}ns", elapsed_as_seconds, elapsed_average);
    info!("elapsed: {:.3}s, average: {:.0}ns", elapsed_as_seconds, elapsed_average);

    Ok(())
}