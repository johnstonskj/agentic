---
description: Guidelines for implementing `no_std` in crates.
name: no-std
context:
  applyTo: '**/*.rs'
---
# Implementing `no_std` in crates

*Principles*

* Minimize dependencies on the standard library to enable usage in `no_std` environments.
* Use feature flags to conditionally include `alloc` and `std` support.
* Guard imports and code with appropriate `cfg` attributes to ensure compatibility.

## File `Cargo.toml`

```toml
[features]
default = ["?"]
alloc = []
std = ["alloc"]
```

## File `lib.rs`

```rust
#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature must be enabled to build this crate, either directly or via the `std` feature.");

#![cfg_attr(all(feature = "alloc", not(feature = "std")), no_std)]
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
```

## Guarded Imports

```rust
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;
```

