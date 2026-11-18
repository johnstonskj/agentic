---
description: Guidelines for implementing logging and tracing.
name: logging
context:
  applyTo: '**/*.rs'
---
# Implementing Logging and Tracing

*Principles*

* Prefer structured logging over unstructured logging.
* Avoid logging sensitive information such as passwords, tokens, or PII.
* Use appropriate log levels (`error`, `warn`, `info`, `debug`, `trace`) based
  on the severity and context.
* Ensure that logging does not introduce significant performance overhead,
  especially in hot paths.
* Use logging, tracing, metrics, and other telemetry tools appropriately to
  gain insights into application behavior.

## Use the Tokio `tracing` module

Unless you have a really good reason not to, **always** use the Tokio `tracing`
module.

For top-level, or API, functions use the tracing `#[instrument]` attribute. Note
that this requires the tracing feature `attributes`. Also use `#[instrument]` or
manual spans for complex or coordination functions.

When reporting errors to the console, **always** use `tracing::error!` or 
even `log::error!`, **never** `println!`/`eprintln!`.

### Tracing in Async Code

For async code, use the `tracing-futures` crate.

### Tracing and File-based Logging

For file-based logs you can usually use the `tracing-appender` crate.

### Tracing in Tests

For test cases depending on trace/log output you can usually use the `test-log`
crate to initialize tracing in tests.

## Other Telemetry

* Usually use `opentelemetry` (and `opentelemetry_sdk`).
  * Use `tracing-opentelemetry` to emit trace data through Open Telemetry. Also, use `opentelemetry-stdout` to emit a local copy of the traces.
  * Use `opentelemetry_otlp` to connect to a sink.
* Usually encapsulate initialization in a `telemetry.rs` private module.
  * provide a common `init(...)` function, which calls `init_logging`, `init_metrics`, and `init_tracing`.
