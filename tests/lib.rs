#![feature(old_io)]
//! This test file mostly just has tests that make sure that the macros successfully compile.

extern crate fern;
#[macro_use]
extern crate fern_macros;

use std::sync;

#[test]
fn test_log() {
    fern::local::set_thread_logger(sync::Arc::new(
        Box::new(fern::NullLogger) as fern::BoxedLogger));
    log!(&fern::Level::Info, "expected info message");
}

#[test]
fn test_levels() {
    fern::local::set_thread_logger(sync::Arc::new(
        Box::new(fern::NullLogger) as fern::BoxedLogger));
    debug!("expected debug message");
    info!("expected info message");
    warning!("expected warning message");
    severe!("expected severe message");
}

fn does_not_error() -> Result<String, String> {
    Ok("unexpected error message!".to_string())
}
fn errors() -> Result<String, String> {
    Err("expected severe message".to_string())
}

#[test]
fn test_error_logging() {
    fern::local::set_thread_logger(sync::Arc::new(
        Box::new(fern::NullLogger) as fern::BoxedLogger));
    log_error!(errors(), "expected error: {e:?}");
    log_error!(does_not_error(), "unexpected error!: {e:?}");
}

#[test]
fn test_error_then_with_error() {
    fern::local::set_thread_logger(sync::Arc::new(
        Box::new(fern::NullLogger) as fern::BoxedLogger));
    log_error_then!(errors(), return, "expected error: {e:?}");
    panic!("Should have returned!");
}

#[test]
fn test_error_then_without_error() {
    fern::local::set_thread_logger(sync::Arc::new(
        Box::new(fern::NullLogger) as fern::BoxedLogger));
    log_error_then!(does_not_error(), panic!("not expected!"),
        "unexpected error: {e:?}");
}
