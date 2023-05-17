#![allow(dead_code)]
use colored::Colorize;
use std::fmt;

#[derive(Default)]
pub struct Logger {}

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger::default()
    }

    pub fn log(&self, level: LogLevel, message: String) {
        match level {
            LogLevel::Info => println!("[{}] {}", level.to_string().green(), message),
            LogLevel::Warn => println!("[{}] {}", level.to_string().yellow(), message),
            LogLevel::Error => eprintln!("[{}] {}", level.to_string().red(), message),
        }
    }

    pub fn info(&self, message: String) {
        self.log(LogLevel::Info, message)
    }

    pub fn warn(&self, message: String) {
        self.log(LogLevel::Info, message)
    }

    pub fn error(&self, message: String) {
        self.log(LogLevel::Info, message)
    }
}
