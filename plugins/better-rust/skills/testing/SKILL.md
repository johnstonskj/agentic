---
description: Testing guidelines and best-practices.
name: testing
context:
  applyTo: '**/*.rs'
---

# Testing

*Principles*

* Tests **should** be isolated and independent.
* Tests **should** cover both typical and edge cases.
* Tests **should** be deterministic and reproducible.
* Write unit tests alongside the code they test using `#[cfg(test)]` modules and `#[test]` annotations.
* Write integration tests in the `tests/` directory with descriptive filenames; by default `test_{feature_name}.rs`.

## Unit/Integration Tests

[unit tests](./plain.rs)

[integration tests](./integration_test.rs)

**Note:** feature test files **must** start with a cfg attribute 
indicating the feature they are testing, e.g. `#![cfg(feature = "my_feature")]`.

### Workflow

Preferred: `mise :test` or `mise :nt`

This ensures that all tests are executed and that the code behaves as expected.
It runs *at least* the following checks:

* `cargo test`, or 
* `cargo nextest run`

## Code Coverage

### Configuration

A `tarpaulin.toml` file can be used to configure the behavior of the code coverage 
measurement. it should exist at the root of the project. and consist of *at least*
the following content (file: [tarpaulin.toml](./tarpaulin.toml)):

```toml
# Default features, in release mode
[default_release]
release = true

# All features, in debug mode
[all_features]
release = true
all-features = true

# Common flag settings.
[report]
out = ["Html", "Xml"]
```

To ensure that coverage metrics are met, especially during CI, the file 
`codecov.yml` should be present at the root of the each crate with *at
least* the following (file: [codecov.yml](./codecov.yml)):

```yaml
coverage:
  status:
    project:
      default:
        target: 85%
        threshold: 5%

component_management:
  individual_components:
    - component_id: error
      name: Error Management
      paths:
        - src/error.rs
```

### Workflow

Preferred: `mise :coverage`

This ensures that the code coverage is measured and reported accurately.
It runs *at least* the following checks:

* `cargo tarpaulin`
