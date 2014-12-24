#![feature(macro_rules)]

//! Logging macros for the fern library. This package stores a thread-local logger, and allows access to it via log!() and <level>!() macros.
//!
//! Note that in order for logging_macros to function correctly, you can't rename the fern crate (using 'as'), and `fern_macros` needs to be used as plugin and link. Shown here:
//!
//! ```
//! extern crate fern;
//! #[phase(plugin, link)]
//! extern crate fern_macros;
//! ```

extern crate fern;

use std::cell;
use std::sync;
use std::io::stdio;
use fern::ArcLogger;
use fern::Level;
use fern::OutputConfig;

thread_local!(static DEFAULT_LOGGER: cell::RefCell<ArcLogger> = cell::RefCell::new(sync::Arc::new(OutputConfig::Stdout.into_logger().unwrap())));

#[experimental]
pub fn init_thread_logger(logger: ArcLogger) {
    DEFAULT_LOGGER.with(move |log| {
        *log.borrow_mut() = logger;
    });
}

#[experimental]
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

/// Logs a message with the thread logger.
#[macro_export]
#[experimental]
macro_rules! log(
    ($level:expr, $($arg:tt)*) => (
        ::fern_macros::log($level, format!($($arg)*).as_slice())
    )
);

/// Logs a debug message with the thread logger.
#[macro_export]
#[experimental]
macro_rules! debug(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Debug, $($arg)*)
    )
);

/// Logs an informational message with the thread logger.
#[macro_export]
#[experimental]
macro_rules! info(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Info, $($arg)*)
    )
);

/// Logs a warning message with the thread logger.
#[macro_export]
#[experimental]
macro_rules! warning(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Warning, $($arg)*)
    )
);

/// Logs a severe message with the thread logger.
#[macro_export]
#[experimental]
macro_rules! severe(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Severe, $($arg)*)
    )
);

/// Logs an error message with the thread logger, if an error occurs.
///
/// This macro requires a Result<_, Show>, and will log as severe error message with the given argument format if the result happens to end up being Err(e)
#[macro_export]
#[experimental]
macro_rules! log_error(
    ($result:expr, $($arg:tt)*) => (
        match $result {
            Ok(_) => (),
            Err(e) => severe!(format!($($arg)*, e=e)),
        }
    )
);

/// Exactly the same as log_error!(), except also has an 'after' statement which will be run after logging the severe error message.
#[macro_export]
#[experimental]
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
);
