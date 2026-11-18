---
description: Rustdoc guidelines for API documentation.
name: better-rustdoc
context:
  applyTo: '**/*.rs'
---
# API Documentation

*Principles*

* Document all public APIs with rustdoc (`///` comments) following the 
  [API Guidelines](https://rust-lang.github.io/api-guidelines/).
  * Place module-level documentation at the top of the file using `//!`.
* Include clear and concise comments for each function, struct, enum, and
complex logic.
* Avoid including redundant comments which are tautological or 
  self-demonstating; e.g. cases where it is easily parsable what the code
  does at a glance or its function name giving sufficient information as
  to what the code does.
* Document error conditions, panic scenarios, and safety considerations.
* Use `#[doc(hidden)]` to hide implementation details from public
  documentation.
* Provide examples using the `?` operator instead of `unwrap()` or the
  deprecated `try!` macro.

## Crate-Level Documentation

TBD

### Feature Documentation

If your library has a large number of, or interaction between, features then
consider using the [document-features](https://crates.io/crates/document-features)
crate to keep feature descriptions in your `Cargo.toml` file and include them
in your library module documentation.

### Include Examples/Benchmarks

TBD

## Function/Method Documentation

TBD

## Consider non-*prose* Content

In addition to prose documentation, consider including non-prose content such
as diagrams, tables, and domain-specific examples and mappings to improve the
clarity and comprehensibility of your documentation. This can help users
quickly understand complex concepts and the relationships between different
components of your crate.

For example, you might include a diagram showing the relationships between
different modules, a table summarizing configuration options, or a mapping
of domain-specific concepts to your crate's types and functions. In general,
this requires a `/doc` directory in your crate where you can place additional
assets such as image files, and then be able to reference them via repository
URLs.

You may include additional scripts and stylesheets into generated
rustdoc to include LaTeX mathematical expressions, as another example. To do
this you will need to place *at least* an HTML header file in the `/doc` 
directory and reference it in `Cargo.toml` as shown below.

```toml
[package.metadata.docs.rs]
rustdoc-args = ["--html-in-header", "doc/header.html"]
```

## Consider a `doc-private` feature

Adding a `doc-private` feature allows you to include documentation for items
that are meant to be private to the crate but still want to be documented for
internal use. This can be useful for developers working on the crate itself,
without exposing these details to the public API. Such a feature should have
zero dependencies, and introduced into `Cargo.toml` as shown below.

```toml
[features]
doc-private = []
```

To implement this, you can conditionally include documentation using both the
rustdoc enabled `#[cfg(doc)]` and a custom feature flag like `doc-private`.
For example:

```rust
#[cfg_attr(
  all(cfg(doc), feature = "doc-private"), 
  doc = "This is an internal function, only documented when the `doc-private` feature is enabled."
)]
fn internal_function() {
    // implementation
}
```

For macros, you can use a similar approach but you set `macro_export`
conditionally as rustdoc only generates documentation for exported macros.

```rust
#[cfg_attr(all(cfg(doc), feature = "doc-private"), macro_export)]
/// This is an internal macro, only documented when the `doc-private` feature is enabled.
macro_rules! internal_macro {
    () => {
        // implementation
    };
}
```