
use log::LevelFilter;
use log::info as log_info;
use fast_log::config::Config;
use ftlog::{
    appender::{file::Period, FileAppender},
    LoggerGuard, FtLogFormat, Record,
};
use ftlog::info as ftlog_info;
use time::{Duration};

use trading_engine::logger::{
    appender::{file::Period as MyPeriod, FileAppender as MyFileAppender},
    logger::{
        self,
        LoggerGuard as MyLoggerGuard,
        LogFormat as MyLogFormat,
        Record as MyRecord,
        info as myinfo,
    },
};

fn main() {
    //bench_fast_log();
    //bench_ftlog();
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

    for _ in 0..10 {
        let now = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..1000 {
            sum += i*i;
            log_info!("sum = {}", sum); 
        }

        log::logger().flush();

        let elapsed = now.elapsed();    
        histograms.push(elapsed);
    }
    
    println!("elapsed time in fast-log: {:?}", histograms);

}


fn bench_ftlog() {
    println!("bench ftlog");

    let time_format = time::format_description::parse_owned::<1>(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]",
    ).unwrap();

    let _guard = ftlog::builder()
        .max_log_level(LevelFilter::Info)
        .time_format(time_format)
        .bounded(100_000, false)
        .root(
            FileAppender::builder()
                .path("./ftlog.log")
                .rotate(Period::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .fixed_timezone(time::UtcOffset::current_local_offset().unwrap())
        .try_init()
        .expect("logger build or set failed");

    let mut histograms = Vec::new();
    for _ in 0..10 {
        let now = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..1000 {
            sum += i*i;
            ftlog_info!("sum = {}", sum);
        }

        let elapsed = now.elapsed();
        histograms.push(elapsed);
    }
    
    println!("elapsed time for ftlog: {:?}", histograms);
}

fn bench_mylogger() {
    println!("bench mylogger");

    let _guard = logger::builder()
        .max_log_level(LevelFilter::Info)
        .bounded(100_000, false)
        //.unbounded()
        .root(
            MyFileAppender::builder()
                .path("./mylog.log")
                .rotate(MyPeriod::Day)
                .expire(Duration::days(7))
                .build(),
        )
        .fixed_timezone(time::UtcOffset::current_local_offset().unwrap())
        .try_init()
        .expect("logger build or set failed");

    let mut histograms = Vec::new();
    for _ in 0..10 {
        let now = std::time::Instant::now();
        let mut sum = 0;
        for i in 0..1000 {
            sum += i*i;
            myinfo!("sum = {}", sum);
        }
        let elapsed = now.elapsed();
        histograms.push(elapsed);
    }
    println!("elapsed time for mylog: {:?}", histograms);
}
