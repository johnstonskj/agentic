---
description: Style and linting guidelines.
name: style
context:
  applyTo: '**/*.rs'
---
# Style and Linting

*Principles*

* Follow idiomatic Rust practices and community standards.
* Follow the Rust Style Guide and use `rustfmt` for automatic formatting.
  * Do not override format defaults without documentation and team consensus.
  * Just In Case: you **must** use 4 spaces for indentation, **never** tabs.
* Keep lines under 100 characters when possible.
* Place function and struct documentation immediately before the item using
  `///`.
* Place module-level documentation at the top of the file using `//!`.
* Avoid including redundant comments which are tautological or 
  self-demonstating; e.g. cases where it is easily parsable what the code does
  at a glance or its function name giving sufficient information as to what the
  code does.
* Use *both* `cargo check` and`cargo clippy` (just in case) to catch common
  mistakes and enforce best practices.

## Configuration

All check/clippy rules are defined in the project's root `Cargo.toml`, as defined in
the `[workspace.metadata]` section (see the 
[Cargo configuration skill](../cargo/SKILL.md)).

Do not override the rules for `rustfmt`, in order to ensure this include a file
named `rustfmt.toml` at the root of the project with the following content
(file: [rustfmt.toml](./rustfmt.toml)):

```toml
# This is an empty `rustfmt.toml` file. This means that the default configuration will
# always be used, as per-project configuration overrides any per-user configuration.
```

## Workflow

Preferred: `mise :lint`

This ensures that the code adheres to the defined linting rules and maintains a
consistent style across the project. It runs *at least* the following checks:

* cargo check
* cargo clippy
* cargo fmt
