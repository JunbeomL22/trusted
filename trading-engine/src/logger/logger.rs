use crate::utils::timer::get_unix_nano;
//
use once_cell::sync::Lazy;
use crossbeam_channel::{Sender, unbounded};
use std::{
    io::{Write, BufWriter},
    fs::{File, OpenOptions},
    path::PathBuf,
};
use core_affinity;
use std::{
    thread,
    sync::{
        Mutex, 
        atomic::{
            AtomicUsize, 
            AtomicBool,
            AtomicI32,
            Ordering},
    },
};
use anyhow::{Result, anyhow};
use chrono;

const LOG_MESSAGE_BUFFER_SIZE: usize = 1_000_000; // string length
const LOG_MESSAGE_FLUSH_INTERVAL: u64 = 1_000_000; // 1 second

pub static MAX_LOG_LEVEL: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(LogLevel::NIL.as_usize()));
pub static TIMEZONE: Lazy<AtomicI32> = Lazy::new(|| AtomicI32::new(TimeZone::Local as i32));
pub static CONSOLE_REPORT: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static LOGGER_HANDLER: Lazy<Mutex<Option<thread::JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

pub static LOG_SENDER: Lazy<Sender<LogMessage>> = Lazy::new(|| {
    let (sender, receiver) = unbounded();
    
    let mut message_queue: Vec<String> = Vec::with_capacity(LOG_MESSAGE_BUFFER_SIZE);
    let mut last_flush_time = get_unix_nano();

    *LOGGER_HANDLER.lock().unwrap() = Some(thread::spawn(move || {
        let mut writer: Option<BufWriter<File>> = None;
        while let Ok(msg) = receiver.recv() {
            match msg {
                LogMessage::LazyMessage(lazy_message) => {
                    let message = lazy_message.eval();
                    let new_msg_length = message.len();
                    let buffer_size = message_queue.len();
                    let timestamp = get_unix_nano();
                    message_queue.push(message);

                    if (buffer_size + new_msg_length > LOG_MESSAGE_BUFFER_SIZE) ||
                        (timestamp - last_flush_time > LOG_MESSAGE_FLUSH_INTERVAL) {
                        if let Some(ref mut writer) = writer {
                            let output = message_queue.join("");
                            writer.write_all(output.as_bytes()).unwrap();
                            if CONSOLE_REPORT.load(Ordering::Relaxed) {
                                println!("{}", output);
                            }

                            message_queue.clear();
                            last_flush_time = get_unix_nano();
                        } 
                    }
                },
                LogMessage::StaticString(message) => {
                    let buffer_size = message_queue.len();
                    let timestamp = get_unix_nano();
                    message_queue.push(message.to_string());

                    if (buffer_size + message.len() > LOG_MESSAGE_BUFFER_SIZE) ||
                        (timestamp - last_flush_time > LOG_MESSAGE_FLUSH_INTERVAL) {
                        if let Some(ref mut writer) = writer {
                            let output = message_queue.join("");
                            writer.write_all(output.as_bytes()).unwrap();
                            if CONSOLE_REPORT.load(Ordering::Relaxed) {
                                println!("{}", output);
                            }

                            message_queue.clear();
                            last_flush_time = get_unix_nano();
                        } 
                    } 
                },
                LogMessage::SetFile(file_name) => {
                    if let Some(ref mut writer) = writer {
                        writer.flush().unwrap();
                        let _ = writer.get_mut().sync_all();
                        *writer = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(file_name)
                            .map(|file| BufWriter::new(file))
                            .unwrap();
                    } else {
                        writer = Some(BufWriter::new(
                            OpenOptions::new()
                                .create(true)
                                .write(true)
                                .append(true)
                                .open(&file_name)
                                .map_err(|e| anyhow!("Failed to open file: {} [{}]", file_name.display(), e))
                                .unwrap()
                        ));
                    }
                },
                LogMessage::SetCore => {
                    let core_ids = core_affinity::get_core_ids().unwrap();
                    if let Some(last_core_id) = core_ids.last() {
                        core_affinity::set_for_current(*last_core_id);
                    } else {
                        panic!("No core available for logger thread")
                    }
                },
                LogMessage::Close => {
                    if let Some(ref mut writer) = writer {
                        writer.write_all(message_queue.join("").as_bytes()).unwrap();
                        writer.flush().unwrap();
                        let _ = writer.get_mut().sync_all();
                    }
                    break;
                },
            }
        }
    }));
    sender
});

pub enum TimeZone {
    Local,
    Seoul,
    Japan,
    NewYork,
}

impl TimeZone {
    pub fn as_offset_hour(&self) -> i32 {
        match self {
            TimeZone::Local => {
                let local = chrono::Local::now();
                let offset = local.offset().local_minus_utc() / 3600;
                offset as i32
            }
            TimeZone::Seoul => 9,
            TimeZone::Japan => 9,
            TimeZone::NewYork => -4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    NIL = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

impl LogLevel {
    pub fn as_usize(&self) -> usize {
        match self {
            LogLevel::NIL => 0,
            LogLevel::Error => 1,
            LogLevel::Warn => 2,
            LogLevel::Info => 3,
            LogLevel::Debug => 4,
            LogLevel::Trace => 5,
        }
    }

    pub fn from_usize(level: usize) -> Result<LogLevel> {
        match level {
            0 => Ok(LogLevel::NIL),
            1 => Ok(LogLevel::Error),
            2 => Ok(LogLevel::Warn),
            3 => Ok(LogLevel::Info),
            4 => Ok(LogLevel::Debug),
            5 => Ok(LogLevel::Trace),
            _ => {
                let error = || anyhow!("Invalid log level: {}", level);
                Err(error())
            },
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::NIL => write!(f, "Nil"),
            LogLevel::Trace => write!(f, "Trace"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Warn => write!(f, "Warn"),
        }
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Error, "legacy_text", text = msg);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Warn, "legacy_text", text = msg);
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Info, "legacy_text", text = msg);
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Debug, "legacy_text", text = msg);
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log_fn_json!($crate::LogLevel::Trace, "legacy_text", text = msg);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Warn, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_info {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Info, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Debug, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_trace {
    ($topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, $topic, $($key=$value),+);
    }};
    ($topic:expr, $struct:expr) => {{
        $crate::log_fn_json!($crate::LogLevel::Trace, $topic, $struct);
    }};
}

#[macro_export]
macro_rules! log_fn_json {
    ($level:expr, $topic:expr, $($key:ident=$value:expr),+ $(,)?) => {{
        let max_log_level = $crate::LogLevel::from_usize($crate::MAX_LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= max_log_level {
            let timestamp = $crate::timer::get_unix_nano();
            let func = move || {
                let json_obj = $crate::serde_json::json!({
                    $(
                        stringify!($key): $value,
                    )+
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let json_msg = $crate::serde_json::json!({
                    "timestamp": $crate::timer::convert_unix_nano_to_datetime_format(timestamp, timezone),
                    "level": $level.to_string(),
                    "src": format!("{}:{}", file!(), line!()),
                    "topic": $topic,
                    "data": json_obj,
                });

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};

    // In case of structs
    ($level:expr, $topic:expr, $struct:expr) => {{
        let current_level = $crate::LogLevel::from_usize($crate::LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)).unwrap();
        if $level <= current_level {
            let timestamp = $crate::timer::get_unix_nano();
            let func = move || {
                let json_obj = $crate::serde_json::to_value($struct).unwrap_or_else(|e| {
                    $crate::serde_json::json!({ "error": format!("serialization error: {}", e) })
                });
                let timezone = $crate::TIMEZONE.load(std::sync::atomic::Ordering::Relaxed);
                let json_msg = $crate::serde_json::json!({
                    "timestamp": $crate::timer::convert_unix_nano_to_datetime_format(timestamp, timezone),
                    "level": $level.to_string(),
                    "src": format!("{}:{}", file!(), line!()),
                    "topic": $topic,
                    "data": json_obj,
                });

                json_msg.to_string() + "\n"
            };

            $crate::LOG_SENDER.send($crate::LogMessage::LazyMessage($crate::LazyMessage::new(func))).unwrap();
        }
    }};
}

pub struct LoggerGuard;

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        log_trace!("logger", message="LoggerGuard is dropped");
        Logger::finalize();
    }
}

pub struct Logger {
    file_name: Option<String>,
}

impl Logger {
    pub fn finalize() {
        let _ = LOG_SENDER.send(LogMessage::Close);
        if let Some(handler) = LOGGER_HANDLER.lock().unwrap().take() {
            let _ = handler.join();
        }
    }

    pub fn initialize() -> Logger {
        let _ = get_unix_nano();
        Logger {
            file_name: None,            
        }
    }

    pub fn with_file(mut self, file_path: &str, file_name: &str) -> Result<Logger> {
        std::fs::create_dir_all(file_path)?;

        let current_time = chrono::Local::now();
        let file_name = format!("{}/{}-{}.log", file_path, file_name, current_time.format("%Y%m%d-%H%M%S"));
        self.file_name = Some(file_name);
        Ok(self)
    }

    pub fn with_console_report(self, console_report: bool) -> Logger {
        CONSOLE_REPORT.store(console_report, Ordering::Relaxed);
        self
    }

    pub fn with_max_log_level(self, level: LogLevel) -> Logger {
        MAX_LOG_LEVEL.store(level.as_usize(), Ordering::Relaxed);
        self
    }

    pub fn with_timezone(self, timezone: TimeZone) -> Logger {
        TIMEZONE.store(timezone.as_offset_hour(), Ordering::Relaxed);
        self
    }

    pub fn launch(self) -> LoggerGuard {
        let file_name = self.file_name.clone();
        let _ = LOG_SENDER.send(LogMessage::SetCore);
        if let Some(file_name) = file_name {
            let _ = LOG_SENDER.send(LogMessage::SetFile(PathBuf::from(file_name)));
        }
        LoggerGuard {}
    }
}

pub enum LogMessage {
    LazyMessage(LazyMessage),
    StaticString(&'static str),
    SetFile(PathBuf),
    SetCore,
    Close,
}

pub struct LazyMessage {
    data: Box<dyn (FnOnce() -> String) + Send + 'static>,
}

impl LazyMessage {
    pub fn new<F>(data: F) -> LazyMessage
    where
        F: (FnOnce() -> String) + Send + 'static,
    {
        LazyMessage { data: Box::new(data) }
    }

    pub fn eval(self) -> String {
        (self.data)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Clone, Serialize)]
    struct TestStruct {
        a: i32,
        b: f64,
        c: String,
    }

    #[test]
    fn test_logger() -> Result<()> {
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
            c: "hello".to_string(),
        };

        for _ in 0..iteration {
            let test_clone = test_struct.clone();
            log_info!("test", struct_log = test_clone);
        }

        let end = crate::timer::get_unix_nano();

        let elapsed = end - start;
        let elapsed_as_seconds = elapsed as f64 / 1_000_000_000.0;
        let elapsed_average = elapsed as f64 / iteration as f64;

        info!("elapsed: {}s, average: {}ns", elapsed_as_seconds, elapsed_average);

        Ok(())
    }
}