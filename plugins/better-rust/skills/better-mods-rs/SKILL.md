---
description: Guidelines for module layout and organization.
name: better-mods-rs
context:
  applyTo: '**/*.rs'
---
# Module Layout and Organization

This document provides guidelines for module layout and organization for Rust
source. To do so effectively, it defines a set of *sections*, logical groupings
of declaration types and their recommended placement within different kinds of
Rust source files. The following table summarizes the recommended placement of
these different sections within a set of Rust source types which are elaborated
below.

| Section           | `lib`   | `main`  | `error` | `plain` | `test`  | `macros`    |
|-------------------|---------|---------|---------|---------|---------|-------------|
| Public Macros     | Yes     | Yes(2)  | Yes     | Yes     | No      | **Yes**(3)  |
| Public Types      | Yes     | Yes(2)  | Yes     | Yes     | No      | **No**(4)   |
| Public Functions  | Yes     | No      | Yes     | Yes     | No      | **No**      |
| Private Macros    | **No**  | **No**  | **No**  | Yes     | Yes     | **No**      |
| Private Types     | Yes     | No      | No      | Yes     | Yes     | **No**      |
| Implementations   | Yes     | Yes     | Yes     | Yes     | Yes     | **No**      |
| Private Functions | Yes     | Yes     | No      | Yes     | Yes     | **No**      |
| Modules           | Yes     | Yes(1)  | No      | Yes     | Yes     | **No**      |
| Unit Tests        | **No**  | Yes     | Yes     | Yes     | **No**  | **No**      |
| Main              | **No**  | Yes(5)  | **No**  | **No**  | **No**  | **No**      |
| Test Functions    | **No**  | **No**  | **No**  | **No**  | Yes     | **No**      |

Notes:

1. In `main.rs`, modules are all declared private and are all
   typically declared at the top of the file *before* the main function. 
2. In `main.rs`, public really means `pub(crate)` for visibility within the
   binary only.
3. In `macros.rs`, public really means available for use in other modules, but
   not necessarily exported outside the crate.
4. In `macros.rs`, private declarations are hard to reference from within a
   macro, so don't.
5. Clearly this section only applies to binaries.

*Principles*

* Keep all modules small and focused.
* Prefer private modules and encapsulation.
* Group related items together in a consistent manner within modules.

## Library Modules

*Principles*

* Keep `lib.rs` minimal, primarily for defining the public API of the library.
* Organize modules hierarchically, grouping related functionality together.
  * Wherever possible consider more generic names for top-level modules to
    clearly indicate their purpose (e.g., `model`, `config`, `network`, 
    `rules`).
* Avoid placing implementation details in `lib.rs`; use submodules for that.
* Re-export key types and functions at the top level for easier consumption.

Template file: [lib.rs](./lib.rs)

## Binary Modules

*Principles*

* Keep `main.rs` minimal, primarily for application setup, configuration, and
  orchestration.
* Use `#[tokio::main]`, or equivalent, for async applications.

For example, keep all command-line argument parsing and setup in a `cli`
module, and move business logic to separate modules.

Template file: [main.rs](./main.rs)

## Error Modules

*Principles*

* The `error.rs` module should define a top-level error type for the library
  or application.
* Unless you have a specific reason not to, use the `thiserror` crate to define
  error variants.
* Provide meaningful error messages and context, including source errors.
* Include helper functions for constructing error variants if helpful.
* Ensure that error types implement standard traits like `Debug`, `Display`,
  and `Error`.
* Keep the error module focused solely on error definitions and related helpers, 
  avoiding unrelated logic.

Template file: [error.rs](./error.rs)

## Other Modules

Template file: [plain.rs](./plain.rs)

## Integration Test Modules

*Principles*

* Keep integration tests in the `tests` directory, separate from unit tests.
* Name test files descriptively to indicate the functionality being tested.
* **Only** use the library's public API for testing, avoiding reliance on 
  internal implementation details.
* Ensure tests are independent and can be run in any order.
* Leverage test helpers and fixtures to reduce duplication and improve 
  readability.
* Keep feature-gated tests in separate files, clearly indicating the feature 
  they are testing.

For more information, see the 
[*Testing guidelines for Rust projects* skill](../testing/SKILL.md). 
In particular, note that feature test files **must** start with a cfg attribute 
indicating the feature they are testing, e.g. `#![cfg(feature = "my_feature")]`.

Template file: [integration_tests.rs](./integration_tests.rs)
