use once_cell::sync::Lazy;
use crossbeam_channel::{Sender, Receiver, unbounded};
use std::{
    io::{Write, BufWriter},
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};
use crate::utils::timer::get_unix_nano;
use core_affinity;
use std::{
    thread,
    sync::Mutex,
};
use anyhow::{Result, anyhow};
use chrono;
use core_affinity::CoreId;
use lazy_format::lazy_format;

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

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

pub struct LoggerGuard;

impl Drop for LoggerGuard {
    fn drop(&mut self) {
        Logger::finalize();
    }
}

pub struct Logger {
    max_file_log_level: LogLevel,
    max_console_log_level: LogLevel,
    file_report: bool,
    console_report: bool,
    file: Option<File>,
    time_zone: TimeZone,
}

impl Logger {
    pub fn finalize() {
        unimplemented!("finalize")
    }

    pub fn initialize() -> Logger {
        let _ = get_unix_nano();
        Logger {
            max_file_log_level: LogLevel::Info,
            max_console_log_level: LogLevel::Info,
            file_report: false,
            console_report: false,
            file: None,
            time_zone: TimeZone::Local,
        }
    }

    pub fn with_file(mut self, file_path: &str, file_name: &str) -> Result<Logger> {
        let current_time = chrono::Local::now();
        // make a string of the current time upto sec
        let time_str = current_time.format("%Y%m%d-%H:%M:%S").to_string();
        let file_path = format!("{}/{}.log-{}", file_path, file_name, time_str);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;
        
        self.file = Some(file);
        self.file_report = true;
        Ok(self)
    }

    pub fn with_console(mut self) -> Logger {
        self.console_report = true;
        self
    }

    pub fn with_max_file_log_level(mut self, level: LogLevel) -> Logger {
        self.max_file_log_level = level;
        self
    }

    pub fn with_max_console_log_level(mut self, level: LogLevel) -> Logger {
        self.max_console_log_level = level;
        self
    }

    pub fn with_time_zone(mut self, time_zone: TimeZone) -> Logger {
        self.time_zone = time_zone;
        self
    }

    pub fn launch(self) -> LoggerGuard {
        static SENDER: Lazy<Sender<LazyMessage>> = Lazy::new(|| {
            let (sender, receiver) = unbounded();
            thread::spawn(move || {
                let mut buffer = BufWriter::new(Mutex::new(Vec::new()));
                loop {
                    let message = receiver.recv().unwrap();
                    let message = message.eval();
                    let _ = buffer.write_all(message.as_bytes());
                    let _ = buffer.flush();
                }
            });
            sender
        });
        LoggerGuard {}
    }
}

pub struct LazyMessage {
    data: Box<dyn (FnOnce() -> String) + Send + 'static>,
}

impl LazyMessage {
    pub fn new<F>(date: F) -> LazyMessage
    where
        F: (FnOnce() -> String) + Send + 'static,
    {
        LazyMessage { data: Box::new(data) }
    }

    pub fn eval(&self) -> String {
        (self.data)()
    }
}