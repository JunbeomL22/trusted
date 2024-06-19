use log::LevelFilter;
use log::info as log_info;
use fast_log::config::Config;
use ftlog::{
    appender::{file::Period, FileAppender},
    LoggerGuard, FtLogFormat, Record,
};
use ftlog::{info as ftlog_info, debug as ftlog_debug};
use time::Duration;

const MESSAGE_NUM: usize = 10000;
const ITER_NUM: usize = 20;
fn main() {
    bench_ftlog();
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
