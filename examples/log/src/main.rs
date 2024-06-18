
use log::LevelFilter;
use log::info as log_info;
use fast_log::config::Config;
use ftlog::{
    appender::{file::Period, FileAppender},
    LoggerGuard, FtLogFormat, Record,
};
use ftlog::{info as ftlog_info, debug as ftlog_debug};
use time::{Duration};

const MESSAGE_NUM: usize = 10000;
const ITER_NUM: usize = 20;
const SLEEP_TIME_MS: u64 = 1;

use trading_engine::logger::{
    appender::{file::Period as MyPeriod, FileAppender as MyFileAppender},
    logger::{
        self,
        LoggerGuard as MyLoggerGuard,
        LogFormat as MyLogFormat,
        Record as MyRecord,
        info as myinfo,
        debug as mydebug,
    },
};
// elapsed time for mylog: [26.2232ms, 37.4444ms, 46.1864ms, 29.6724ms, 25.7455ms]
// elapsed time for ftlog: [39.4703ms, 33.5159ms, 30.1036ms, 31.4281ms, 30.5874ms]
fn main() {
    #[cfg(feature = "fast_log")]
    bench_fast_log();
    #[cfg(feature = "ftlog")]
    bench_ftlog();
    #[cfg(feature = "mylogger")]
    bench_mylogger();
}

fn bench_fast_log() {
    println!("bench fast_log");

    fast_log::init(
        Config::new()
            .level(LevelFilter::Info)
            .file("./fast_log.log")
            .chan_len(Some(1024 * 1024 * 10)),
    ).unwrap();

    let mut histograms = Vec::new();

    for _ in 0..ITER_NUM {
        let now = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..MESSAGE_NUM {
            sum += i;
            log_info!("sum = {}", sum); 
        }

        log::logger().flush();

        let elapsed = now.elapsed();
        let elapsed_as_nanos = elapsed.as_nanos();
        let divided = elapsed_as_nanos / MESSAGE_NUM as u128;
        let divided_duration = std::time::Duration::from_nanos(divided as u64);

        histograms.push(divided_duration);
        //std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME_MS));
    }
    
    println!("elapsed time in fast-log: {:?}", histograms);

}


fn bench_ftlog() {
    println!("bench ftlog");

    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    ).unwrap();

    let _guard = ftlog::builder()
        .max_log_level(LevelFilter::Debug)
        .time_format(time_format)
        .bounded(100_000, false)
        .root(
            FileAppender::builder()
                .path("./ftlog.log")
                .rotate(Period::Minute)
                .expire(Duration::seconds(10))
                .build(),
        )
        .fixed_timezone(time::UtcOffset::current_local_offset().unwrap())
        .try_init()
        .expect("logger build or set failed");

    let mut histograms = Vec::new();
    for _ in 0..ITER_NUM {
        let now = std::time::Instant::now();
        for i in 0..MESSAGE_NUM {
            //sum += i;
            ftlog_info!("i: {}", i);
            //ftlog_debug!("sum = {}", sum);
        }

        let elapsed = now.elapsed();
        let elapsed_as_nanos = elapsed.as_nanos();
        let divided = elapsed_as_nanos / MESSAGE_NUM as u128;
        let divided_duration = std::time::Duration::from_nanos(divided as u64);

        histograms.push(divided_duration);
        //std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME_MS));
    }
    
    println!("elapsed time for ftlog: {:?}", histograms);
}

fn bench_mylogger() {
    println!("bench mylogger");

    let _guard = logger::builder()
        .max_log_level(LevelFilter::Debug)
        .bounded(100_000, false)
        //.unbounded()
        .root(
            MyFileAppender::builder()
                .path("./mylog.log")
                .rotate(MyPeriod::Minute)
                .expire(Duration::seconds(10))
                .build(),
        )
        .fixed_timezone(time::UtcOffset::current_local_offset().unwrap())
        .try_init()
        .expect("logger build or set failed");

    let mut histograms = Vec::new();
    for _ in 0..ITER_NUM {
        let now = std::time::Instant::now();
        for i in 0..MESSAGE_NUM {
            myinfo!("i: {}", i);
        }
        
        let elapsed = now.elapsed();
        let elapsed_as_nanos = elapsed.as_nanos();
        let divided = elapsed_as_nanos / MESSAGE_NUM as u128;
        let divided_duration = std::time::Duration::from_nanos(divided as u64);

        histograms.push(divided_duration);
        //std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME_MS));
    }

    println!("elapsed time for mylog: {:?}", histograms);
}
