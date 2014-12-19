fern_macros
===========

Logging macros for the fern library. This package stores a thread-local logger, and allows access to it via log!() and <level>!() macros.

log!(level, "stuff") is literally exactly the same as fern_macros::log(format!("stuff")). the debug!(), info!(), warning!(), error!() and critical!() macros just call the log!() macro with the Debug, Info, Warning, Error and Critical log levels.

Note that in order for logging_macros to function correctly, you can't rename the fern crate (using 'as'), and `fern_macros` needs to be used as plugin and link. Shown here:

```
extern crate fern;
#[phase(plugin, link)]
extern crate fern_macros;
```
