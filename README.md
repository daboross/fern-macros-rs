simple-logging-macros
===

Logging macros for the simple-logging crate. This package stores a thread-local logger, and allows access to it via a log() method and log!() and <level>!() macros.

log!(level, "stuff") is literally exactly the same as logging_macros::log(format!("stuff")). the debug!(), info!(), warning!(), error!() and critical!() macros just call the log!() macro with the Debug, Info, Warning, Error and Critical log levels.

Note that in order for logging_macros to function correctly, you need to be using the 'simple-logging' crate as "logging", and 'simple-logging-macros' as logging_macros. Shown here:

```
extern crate "simple-logging" as logging;
#[phase(plugin, link)]
extern crate "simple-logging-macros" as logging_macros;
```
