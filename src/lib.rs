#![feature(macro_rules)]

extern crate "simple-logging" as logging;

use std::cell;
use std::sync;
use std::io::stdio;
use logging::Logger;
use logging::Level;
use logging::IntoLogger;
use logging::LoggerOutput;

thread_local!(static DEFAULT_LOGGER: cell::RefCell<sync::Arc<Box<Logger + Sync + Send>>> = cell::RefCell::new(sync::Arc::new(LoggerOutput::Stdout.into_logger().unwrap())))

pub fn init_thread_logger(logger: sync::Arc<Box<Logger + Sync + Send>>) {
    DEFAULT_LOGGER.with(move |log| {
        *log.borrow_mut() = logger;
    });
}

pub fn log(level: &Level, msg: &str) {
    DEFAULT_LOGGER.with(|logger| {
        match logger.borrow().log(level, msg) {
            Ok(()) => (),
            // TODO: Should we store stderr_raw here, or does it not matter, since this is really totally backup.
            Err(e) => match write!(&mut stdio::stderr_raw(), "Error logging {{level: {}, msg: {}}}: {}", level, msg, e) {
                Ok(()) => (),
                Err(e2) => panic!(format!("Backup logging failed after regular logging failed. Original log: {{level: {}, msg: {}}}\nLogging error: {}\nBackup logging error: {}", level, msg, e, e2)),
            }
        };
    });
}

#[macro_export]
macro_rules! log(
    ($level:expr, $($arg:tt)*) => (
        ::logging_macros::log($level, format!($($arg)*).as_slice())
    )
)

#[macro_export]
macro_rules! debug(
    ($($arg:tt)*) => (
        log!(&::logging::Level::Debug, $($arg)*)
    )
)

#[macro_export]
macro_rules! info(
    ($($arg:tt)*) => (
        log!(&::logging::Level::Info, $($arg)*)
    )
)

#[macro_export]
macro_rules! warning(
    ($($arg:tt)*) => (
        log!(&::logging::Level::Warning, $($arg)*)
    )
)

#[macro_export]
macro_rules! severe(
    ($($arg:tt)*) => (
        log!(&::logging::Level::Severe, $($arg)*)
    )
)

#[macro_export]
macro_rules! log_error(
    ($result:expr, $($arg:tt)*) => (
        match $result {
            Ok(_) => (),
            Err(e) => severe!(format!($($arg)*, e=e)),
        }
    )
)

#[macro_export]
macro_rules! log_error_then(
    ($result:expr, $after:expr, $($arg:tt)*) => (
        match $result {
            Ok(_) => (),
            Err(e) => {
                severe!($($arg)*, e=e);
                $after;
            },
        }
    )
)
