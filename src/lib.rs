//! Logging macros for the fern library. The macros in the package use the stored thread-local
//! logger. You can log using the log!() macro to specify a level, or the debug!(), info!(),
//! warning!() and severe!() macros for each specific level.
//!
//! Note that in order for logging_macros to function correctly, you need to be using the `fern`
//! crate under the name `fern`, as shown below:
//!
//! ```no_test
//! extern crate fern;
//! #[macro_use]
//! extern crate fern_macros;
//! ```


/// Logs a message with the thread-local logger stored in fern::local.
#[macro_export]
#[experimental]
macro_rules! log(
    ($level:expr, $($arg:tt)*) => (
        {
            let msg = format!($($arg)*);
            let level = $level;
            match ::fern::local::log(level, msg.as_slice()) {
                Ok(()) => (),
                Err(e) => match write!(&mut ::std::io::stdio::stderr_raw(), "Error logging {{level: {}, msg: {}}}: {}", level, msg, e) {
                    Ok(()) => (),
                    Err(e2) => panic!(format!("Backup logging failed after regular logging failed. Original log: {{level: {}, msg: {}}}\nLogging error: {}\nBackup logging error: {}", level, msg, e, e2)),
                }
            };
        }
    )
);

/// Example the same as `log!()`, but specifies the `Level::Debug` logging level.
#[macro_export]
#[experimental]
macro_rules! debug(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Debug, $($arg)*)
    )
);

/// Example the same as `log!()`, but specifies the `Level::Info` logging level.
#[macro_export]
#[experimental]
macro_rules! info(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Info, $($arg)*)
    )
);

/// Example the same as `log!()`, but specifies the `Level::Warning` logging level.
#[macro_export]
#[experimental]
macro_rules! warning(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Warning, $($arg)*)
    )
);

/// Example the same as `log!()`, but specifies the `Level::Severe` logging level.
#[macro_export]
#[experimental]
macro_rules! severe(
    ($($arg:tt)*) => (
        log!(&::fern::Level::Severe, $($arg)*)
    )
);

/// Logs an error message with the thread logger, if an error occurs.
///
/// This macro requires a Result<_, Show>, and will log as severe error message with the given
/// argument format if the result happens to end up being Err(e)
/// Example usage would be something like:
/// ```
/// let writer = std::io::stdio::stdout();
/// log_error!(writer.write("hi"), return, "Failed to write to stream: {e}");
/// ```
/// The above statement would log "Failed to write to stream: <error text>" if writing to stdout
/// failed.
#[macro_export]
#[experimental]
macro_rules! log_error(
    ($result:expr, $($arg:tt)*) => (
        match $result {
            Ok(_) => (),
            Err(e) => severe!($($arg)*, e=e),
        }
    )
);

/// Exactly the same as log_error!(), except also has an 'after' statement which will be run after
/// logging the severe error message.
///
/// Example usage would be something like:
/// ```
/// let writer = std::io::stdio::stdout();
/// log_error_then!(writer.write("hi"), return, "Failed to write to stream: {e}");
/// ```
/// If writing to stdout failed, the above statement would log "Failed to write to stream:
/// <error text>", then return from the current function prematurely.
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
