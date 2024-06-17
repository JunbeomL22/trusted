// The logger is a minor modification of ftlog: https://github.com/nonconvextech/ftlog/
use arc_swap::ArcSwap;
pub use log::{
    debug, error, info, log, log_enabled, logger, trace, warn, Level, LevelFilter, Record,
};

use std::borrow::Cow;
use std::fmt::Display;
use std::hash::{BuildHasher, Hash, Hasher};
use std::io::{stderr, Error as IoError, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossbeam_channel::{bounded, unbounded, Receiver, RecvTimeoutError, Sender, TrySendError};
use hashbrown::HashMap;
use log::{kv::Key, set_boxed_logger, set_max_level, Log, Metadata, SetLoggerError};
use chrono::{DateTime, Utc};
use minstant::Instant as Minstant;
//
use time::{
    OffsetDateTime,
    UtcOffset,
};

//
//
pub enum LogTimezone {
    /// local timezone
    ///
    /// Only *unix OS is supported for now
    Local,
    /// UTC timezone
    Utc,
    /// fixed timezone
    Fixed(UtcOffset),
}

pub fn local_timezone() -> UtcOffset {
    UtcOffset::current_local_offset().unwrap()
}

#[inline]
pub fn minstant_unix_nano() -> u64 {
    static ANCHOR: once_cell::sync::Lazy<minstant::Anchor> =
        once_cell::sync::Lazy::new(|| minstant::Anchor::new());
    Minstant::now().as_unix_nanos(&ANCHOR)
}

#[inline]
pub fn to_utc(time: Minstant) -> DateTime<Utc> {
    static ANCHOR: once_cell::sync::Lazy<minstant::Anchor> =
        once_cell::sync::Lazy::new(|| minstant::Anchor::new());
    DateTime::from_timestamp_nanos(time.as_unix_nanos(&ANCHOR) as i64)
}

#[inline]
pub fn duration(from: Minstant, to: Minstant) -> Duration {
    to.duration_since(from)
}

struct LogMsg {
    time: Minstant,
    msg: Box<dyn Sync + Send + Display>,
    level: Level,
    target: String,
    limit: u32,
    limit_key: u64,
}
impl LogMsg {
    fn write(
        self,
        filters: &Vec<Directive>,
        appenders: &mut HashMap<&'static str, Box<dyn Write + Send>>,
        root: &mut Box<dyn Write + Send>,
        root_level: LevelFilter,
        missed_log: &mut HashMap<u64, i64, nohash_hasher::BuildNoHashHasher<u64>>,
        last_log: &mut HashMap<u64, Minstant, nohash_hasher::BuildNoHashHasher<u64>>,
    ) {
        let writer = if let Some(filter) = filters.iter().find(|x| self.target.starts_with(x.path))
        {
            if filter.level.map(|l| l < self.level).unwrap_or(false) {
                return;
            }
            filter
                .appender
                .and_then(|n| appenders.get_mut(n))
                .unwrap_or(root)
        } else {
            if root_level < self.level {
                return;
            }
            root
        };

        let msg = self.msg.to_string();
        if msg.is_empty() {
            return;
        }

        let now = Minstant::now();

        if self.limit > 0 {
            let missed_entry = missed_log.entry(self.limit_key).or_insert_with(|| 0);
            if let Some(last) = last_log.get(&self.limit_key) {
                if duration(*last, now) < Duration::from_millis(self.limit as u64) {
                    *missed_entry += 1;
                    return;
                }
            }
            last_log.insert(self.limit_key, now);
            let delay = duration(self.time, now);

            let s = format!(
                "({} unixnano) {}ms {} {}\n",
                minstant_unix_nano(),
                delay.as_millis(),
                *missed_entry,
                msg,
            );

            if let Err(e) = writer.write_all(s.as_bytes()) {
                eprintln!("logger write message failed: {}", e);
            };

            *missed_entry = 0;

        } else {
            let delay = duration(self.time, now);
            
            let s = format!(
                "({} unixnano) {}ms {}\n",
                minstant_unix_nano(),
                delay.as_millis(),
                msg
            );

            if let Err(e) = writer.write_all(s.as_bytes()) {
                eprintln!("logger write message failed: {}", e);
            };
        }
    }
}

enum LoggerInput {
    LogMsg(LogMsg),
    Flush,
}

#[derive(Debug)]
enum LoggerOutput {
    Flushed,
    FlushError(std::io::Error),
}

/// Shared by ftlog formatter
///
/// To further reduce time spent on log macro calls, ftlog saves required data
/// and later construct log string in log thread.
///
/// `LogFormat` defines how to turn an reference to record into a box object,
/// which can be sent to log thread and later formatted into string.
///
/// Here is an example of custom formatter:
///
/// ```
/// use std::fmt::Display;
///
/// use trading_engine::logger::logger::LogFormat;
/// use log::{Level, Record};
///
/// struct MyFormatter;
/// impl LogFormat for MyFormatter {
///     fn msg(&self, record: &Record) -> Box<dyn Send + Sync + Display> {
///         Box::new(Msg {
///             level: record.level(),
///             thread: std::thread::current().name().map(|n| n.to_string()),
///             file: record.file_static(),
///             line: record.line(),
///             args: format!("{}", record.args()),
///             module_path: record.module_path_static(),
///         })
///     }
/// }
/// // Store necessary field, define how to format into string with `Display` trait.
/// struct Msg {
///     level: Level,
///     thread: Option<String>,
///     file: Option<&'static str>,
///     line: Option<u32>,
///     args: String,
///     module_path: Option<&'static str>,
/// }
///
/// impl Display for Msg {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.write_str(&format!(
///             "{}@{}||{}:{}[{}] {}",
///             self.thread.as_ref().map(|x| x.as_str()).unwrap_or(""),
///             self.module_path.unwrap_or(""),
///             self.file.unwrap_or(""),
///             self.line.unwrap_or(0),
///             self.level,
///             self.args
///         ))
///     }
/// }
/// ```
pub trait LogFormat: Send + Sync {
    /// turn an reference to record into a box object, which can be sent to log thread
    /// and then formatted into string.
    fn msg(&self, record: &Record) -> Box<dyn Send + Sync + Display>;
}

/// Default ftlog formatter
///
/// The default ftlog format is like:
/// ```text
/// INFO main [examples/ftlog.rs:27] Hello, world!
/// ```
///
/// Since ftlog cannot customize timestamp, the corresponding part is omitted.
/// The actual log output is like:
/// ```text
/// 2022-11-22 17:02:12.574+08 0ms INFO main [examples/ftlog.rs:27] Hello, world!
/// ```
pub struct LogFormatter;
impl LogFormat for LogFormatter {
    /// Return a box object that contains required data (e.g. thread name, line of code, etc.) for later formatting into string
    #[inline]
    fn msg(&self, record: &Record) -> Box<dyn Send + Sync + Display> {
        Box::new(Message {
            level: record.level(),
            thread: std::thread::current().name().map(|n| n.to_string()),
            file: record
                .file_static()
                .map(|s| Cow::Borrowed(s))
                .or_else(|| record.file().map(|s| Cow::Owned(s.to_owned())))
                .unwrap_or(Cow::Borrowed("")),
            line: record.line(),
            args: record
                .args()
                .as_str()
                .map(|s| Cow::Borrowed(s))
                .unwrap_or_else(|| Cow::Owned(format!("{}", record.args()))),
        })
    }
}

struct Message {
    level: Level,
    thread: Option<String>,
    file: Cow<'static, str>,
    line: Option<u32>,
    args: Cow<'static, str>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} {} [{}:{}] {}",
            self.level,
            self.thread.as_ref().map(|x| x.as_str()).unwrap_or(""),
            self.file,
            self.line.unwrap_or(0),
            self.args
        ))
    }
}

struct DiscardState {
    last: ArcSwap<Instant>,
    count: AtomicUsize,
}

/// A guard that flushes logs associated to a Logger on a drop
///
/// With this guard, you can ensure all logs are written to destination
/// when the application exits.
pub struct LoggerGuard {
    queue: Sender<LoggerInput>,
    notification: Receiver<LoggerOutput>,
}
impl Drop for LoggerGuard {
    fn drop(&mut self) {
        self.queue
            .send(LoggerInput::Flush)
            .expect("logger queue closed when flushing, this is a bug");
        self.notification
            .recv()
            .expect("logger notification closed, this is a bug");
    }
}
/// ftlog global logger
pub struct Logger {
    format: Box<dyn LogFormat>,
    level: LevelFilter,
    queue: Sender<LoggerInput>,
    notification: Receiver<LoggerOutput>,
    block: bool,
    discard_state: Option<DiscardState>,
    stopped: AtomicBool,
}

impl Logger {
    pub fn init(self) -> Result<LoggerGuard, SetLoggerError> {
        let guard = LoggerGuard {
            queue: self.queue.clone(),
            notification: self.notification.clone(),
        };

        set_max_level(self.level);
        let boxed = Box::new(self);
        set_boxed_logger(boxed).map(|_| guard)
    }
}

impl Log for Logger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        // already checked in log macros
        self.level >= metadata.level()
    }

    fn log(&self, record: &Record) {
        let limit = record
            .key_values()
            .get(Key::from_str("limit"))
            .and_then(|x| x.to_u64())
            .unwrap_or(0) as u32;

        let msg = self.format.msg(record);
        let limit_key = if limit == 0 {
            0
        } else {
            let mut b = hashbrown::hash_map::DefaultHashBuilder::default().build_hasher();
            if let Some(p) = record.module_path() {
                p.as_bytes().hash(&mut b);
            } else {
                record.file().unwrap_or("").as_bytes().hash(&mut b);
            }
            record.line().unwrap_or(0).hash(&mut b);
            b.finish()
        };
        let msg = LoggerInput::LogMsg(LogMsg {
            time: Minstant::now(),
            msg: msg,
            target: record.target().to_owned(),
            level: record.level(),
            limit,
            limit_key,
        });
        if self.block {
            if let Err(_) = self.queue.send(msg) {
                let stop = self.stopped.load(Ordering::SeqCst);
                if !stop {
                    eprintln!("logger queue closed when logging, this is a bug");
                    self.stopped.store(true, Ordering::SeqCst)
                }
            }
        } else {
            match self.queue.try_send(msg) {
                Err(TrySendError::Full(_)) => {
                    if let Some(s) = &self.discard_state {
                        let count = s.count.fetch_add(1, Ordering::SeqCst);
                        if s.last.load().elapsed().as_secs() >= 5 {
                            eprintln!("Excessive log messages. Log omitted: {}", count);
                            s.last.store(Arc::new(Instant::now()));
                        }
                    }
                }
                Err(TrySendError::Disconnected(_)) => {
                    let stop = self.stopped.load(Ordering::SeqCst);
                    if !stop {
                        eprintln!("logger queue closed when logging, this is a bug");
                        self.stopped.store(true, Ordering::SeqCst)
                    }
                }
                _ => (),
            }
        }
    }

    fn flush(&self) {
        self.queue
            .send(LoggerInput::Flush)
            .expect("logger queue closed when flushing, this is a bug");
        if let LoggerOutput::FlushError(err) = self
            .notification
            .recv()
            .expect("logger notification closed, this is a bug")
        {
            eprintln!("Fail to flush: {}", err);
        }
    }
}

struct BoundedChannelOption {
    size: usize,
    block: bool,
    print: bool,
}

/// Ftlog builder
///
/// ```
/// # use trading_engine::logger::appender::{FileAppender, Duration, Period};
/// # use log::LevelFilter;
/// let logger = trading_engine::logger::logger::builder()
///     // use our own format
///     .format(trading_engine::logger::logger::LogFormatter)
///     // global max log level
///     .max_log_level(LevelFilter::Info)
///     // define root appender, pass anything that is Write and Send
///     // omit `Builder::root` to write to stderr
///     .root(FileAppender::rotate_with_expire(
///         "./current.log",
///         Period::Day,
///         Duration::days(7),
///     ))
///     // ---------- configure additional filter ----------
///     // write to "ftlog-appender" appender, with different level filter
///     .filter("ftlog::appender", "ftlog-appender", LevelFilter::Error)
///     // write to root appender, but with different level filter
///     .filter("ftlog", None, LevelFilter::Trace)
///     // write to "ftlog" appender, with default level filter
///     .filter("ftlog::appender::file", "ftlog", None)
///     // ----------  configure additional appender ----------
///     // new appender
///     .appender("ftlog-appender", FileAppender::new("ftlog-appender.log"))
///     // new appender, rotate to new file every Day
///     .appender("ftlog", FileAppender::rotate("ftlog.log", Period::Day))
///     .build()
///     .expect("logger build failed");
/// ```
///
/// # Local timezone
/// For performance reason, `ftlog` only retrieve timezone info once and use this
/// local timezone offset forever. Thus timestamp in log does not aware of timezone
/// change by OS.
pub struct Builder {
    format: Box<dyn LogFormat>,
    level: Option<LevelFilter>,
    root_level: Option<LevelFilter>,
    root: Box<dyn Write + Send>,
    appenders: HashMap<&'static str, Box<dyn Write + Send + 'static>>,
    filters: Vec<Directive>,
    bounded_channel_option: Option<BoundedChannelOption>,
    timezone: LogTimezone,
}

/// Handy function to get ftlog builder
#[inline]
pub fn builder() -> Builder {
    Builder::new()
}

struct Directive {
    path: &'static str,
    level: Option<LevelFilter>,
    appender: Option<&'static str>,
}

impl Builder {
    #[inline]
    /// Create a ftlog builder with default settings:
    /// - global log level: INFO
    /// - root log level: INFO
    /// - default formatter: `LogFormatter`
    /// - output to stderr
    /// - bounded channel between worker thread and log thread, with a size limit of 100_000
    /// - discard excessive log messages
    /// - log with timestamp of local timezone
    pub fn new() -> Builder {
        Builder {
            format: Box::new(LogFormatter),
            level: None,
            root_level: None,
            root: Box::new(stderr()) as Box<dyn Write + Send>,
            appenders: HashMap::new(),
            filters: Vec::new(),
            bounded_channel_option: Some(BoundedChannelOption {
                size: 100_000,
                block: false,
                print: true,
            }),
            timezone: LogTimezone::Local,
        }
    }

    #[inline]
    /// Log with timestamp of fixed timezone
    pub fn fixed_timezone(mut self, timezone: UtcOffset) -> Builder {
        self.timezone = LogTimezone::Fixed(timezone);
        self
    }

    /// Set custom formatter
    #[inline]
    pub fn format<F: LogFormat + 'static>(mut self, format: F) -> Builder {
        self.format = Box::new(format);
        self
    }

    /// bound channel between worker thread and log thread
    ///
    /// When `block_when_full` is true, it will block current thread where
    /// log macro (e.g. `log::info`) is called until log thread is able to handle new message.
    /// Otherwises, excessive log messages will be discarded.
    ///
    /// By default, excessive log messages is discarded silently. To show how many log
    /// messages have been dropped, see `Builder::print_omitted_count()`.
    #[inline]
    pub fn bounded(mut self, size: usize, block_when_full: bool) -> Builder {
        self.bounded_channel_option = Some(BoundedChannelOption {
            size,
            block: block_when_full,
            print: false,
        });
        self
    }

    /// whether to print the number of omitted logs if channel to log
    /// thread is bounded, and set to discard excessive log messages
    #[inline]
    pub fn print_omitted_count(mut self, print: bool) -> Builder {
        self.bounded_channel_option
            .as_mut()
            .map(|o| o.print = print);
        self
    }

    /// set channel size to unbound
    ///
    /// **ATTENTION**: too much log message will lead to huge memory consumption,
    /// as log messages are queued to be handled by log thread.
    /// When log message exceed the current channel size, it will double the size by default,
    /// Since channel expansion asks for memory allocation, log calls can be slow down.
    #[inline]
    pub fn unbounded(mut self) -> Builder {
        self.bounded_channel_option = None;
        self
    }

    /// Add an additional appender with a name
    ///
    /// Combine with `Builder::filter()`, ftlog can output log in different module
    /// path to different output target.
    #[inline]
    pub fn appender(
        mut self,
        name: &'static str,
        appender: impl Write + Send + 'static,
    ) -> Builder {
        self.appenders.insert(name, Box::new(appender));
        self
    }

    /// Add a filter to redirect log to different output
    /// target (e.g. stderr, stdout, different files).
    ///
    /// **ATTENTION**: level more verbose than `Builder::max_log_level` will be ignored.
    /// Say we configure `max_log_level` to INFO, and even if filter's level is set to DEBUG,
    /// ftlog will still log up to INFO.
    #[inline]
    pub fn filter<A: Into<Option<&'static str>>, L: Into<Option<LevelFilter>>>(
        mut self,
        module_path: &'static str,
        appender: A,
        level: L,
    ) -> Builder {
        let appender = appender.into();
        let level = level.into();
        if appender.is_some() || level.is_some() {
            self.filters.push(Directive {
                path: module_path,
                appender: appender,
                level: level,
            });
        }
        self
    }

    #[inline]
    /// Configure the default log output target.
    ///
    /// Omit this method will output to stderr.
    pub fn root(mut self, writer: impl Write + Send + 'static) -> Builder {
        self.root = Box::new(writer);
        self
    }

    #[inline]
    /// Set max log level
    ///
    /// Logs with level more verbose than this will not be sent to log thread.
    pub fn max_log_level(mut self, level: LevelFilter) -> Builder {
        self.level = Some(level);
        self
    }

    #[inline]
    /// Set max log level
    ///
    /// Logs with level more verbose than this will not be sent to log thread.
    pub fn root_log_level(mut self, level: LevelFilter) -> Builder {
        self.root_level = Some(level);
        self
    }

    /// Finish building ftlog logger
    ///
    /// The call spawns a log thread to formatting log message into string,
    /// and write to output target.
    pub fn build(self) -> Result<Logger, IoError> {
        let mut filters = self.filters;
        // sort filters' paths to ensure match for longest path
        filters.sort_by(|a, b| a.path.len().cmp(&b.path.len()));
        filters.reverse();
        // check appender name in filters are all valid
        for appender_name in filters.iter().filter_map(|x| x.appender) {
            if !self.appenders.contains_key(appender_name) {
                panic!("Appender {} not configured", appender_name);
            }
        }
        let global_level = self.level.unwrap_or(LevelFilter::Info);
        let root_level = self.root_level.unwrap_or(global_level);
        if global_level < root_level {
            warn!(
                "Logs with level more verbose than {} will be ignored",
                global_level,
            );
        }

        let (sync_sender, receiver) = match &self.bounded_channel_option {
            None => unbounded(),
            Some(option) => bounded(option.size),
        };
        let (notification_sender, notification_receiver) = bounded(1);
        std::thread::Builder::new()
            .name("logger".to_string())
            .spawn(move || {
                let mut appenders = self.appenders;
                let filters = filters;

                for filter in &filters {
                    if let Some(level) = filter.level {
                        if global_level < level {
                            warn!(
                                "Logs with level more verbose than {} will be ignored in `{}` ",
                                global_level, filter.path,
                            );
                        }
                    }
                }

                let mut root = self.root;
                let mut last_log = HashMap::default();
                let mut missed_log = HashMap::default();
                let mut last_flush = Instant::now();
                let timeout = Duration::from_millis(200);
                loop {
                    match receiver.recv_timeout(timeout) {
                        Ok(LoggerInput::LogMsg(log_msg)) => {
                            log_msg.write(
                                &filters,
                                &mut appenders,
                                &mut root,
                                root_level,
                                &mut missed_log,
                                &mut last_log,
                            );
                        }
                        Ok(LoggerInput::Flush) => {
                            let max = receiver.len();
                            'queue: for _ in 1..=max {
                                if let Ok(LoggerInput::LogMsg(msg)) = receiver.try_recv() {
                                    msg.write(
                                        &filters,
                                        &mut appenders,
                                        &mut root,
                                        root_level,
                                        &mut missed_log,
                                        &mut last_log,
                                    )
                                } else {
                                    break 'queue;
                                }
                            }
                            let flush_result = appenders
                                .values_mut()
                                .chain([&mut root])
                                .find_map(|w| w.flush().err());
                            if let Some(error) = flush_result {
                                notification_sender
                                    .send(LoggerOutput::FlushError(error))
                                    .expect("logger notification failed");
                            } else {
                                notification_sender
                                    .send(LoggerOutput::Flushed)
                                    .expect("logger notification failed");
                            }
                        }
                        Err(RecvTimeoutError::Timeout) => {
                            if last_flush.elapsed() > Duration::from_millis(1000) {
                                let flush_errors = appenders
                                    .values_mut()
                                    .chain([&mut root])
                                    .filter_map(|w| w.flush().err());
                                for err in flush_errors {
                                    log::warn!("Ftlog flush error: {}", err);
                                }
                                last_flush = Instant::now();
                            };
                        }
                        Err(e) => {
                            eprintln!(
                                "sender closed without sending a Quit first, this is a bug, {}",
                                e
                            );
                        }
                    }
                }
            })?;
        let block = self
            .bounded_channel_option
            .as_ref()
            .map(|x| x.block)
            .unwrap_or(false);
        let print = self
            .bounded_channel_option
            .as_ref()
            .map(|x| x.print)
            .unwrap_or(false);
        Ok(Logger {
            format: self.format,
            level: global_level,
            queue: sync_sender,
            notification: notification_receiver,
            block,
            discard_state: if block || !print {
                None
            } else {
                Some(DiscardState {
                    last: ArcSwap::new(Arc::new(Instant::now())),
                    count: AtomicUsize::new(0),
                })
            },
            stopped: AtomicBool::new(false),
        })
    }

    /// try building and setting as global logger
    pub fn try_init(self) -> Result<LoggerGuard, Box<dyn std::error::Error>> {
        let logger = self.build()?;
        Ok(logger.init()?)
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Builder::new()
    }
}