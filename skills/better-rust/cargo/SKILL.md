---
description: Cargo configuration.
name: cargo
context:
  applyTo: '**/Cargo.toml'
---
# Cargo Configuration

Cargo is the Rust package manager and build system. It manages dependencies,
builds projects, runs tests, and generates documentation. This skill provides
guidance on configuring Cargo for both workspace and single crate projects,
as well as setting up Rustdoc and linting configurations.

*Principles*

* Use `Cargo.toml` to define project metadata, dependencies, and features.
* Keep `Cargo.lock` under version control for applications, but not for
  libraries.
* Use semantic versioning for dependencies to ensure compatibility.
* Include comprehensive metadata: `description`, `license`, `repository`,
  `keywords`, `categories`.
* Leverage Cargo features to enable optional functionality and conditional
  compilation.
* Use workspaces to manage multi-crate projects efficiently.

## Workspace Projects

Modern cargo workspaces allow you to manage multiple related crates in a single
repository efficiently. By pushing common metadata, dependencies, and
configuration to the workspace level, you can reduce duplication and ensure
consistency across all member crates.

Ensure the following in the *workspace* `Cargo.toml`:

1. Ensure the `workspace.resolver` is set to `"3"` to enable the new feature
   resolver for dependencies.
2. Define common package metadata at the workspace level to avoid repetition
   in each member crate. This includes fields like `edition`, `homepage`, 
  `license`, and   `repository`. These are under the key `workspace.package`.
3. Define common dependencies at the workspace level to avoid duplication
   across member crates. These are under the key `workspace.dependencies`.
4. Define all linting configurations at the workspace level to ensure consistent
   code quality standards across all member crates. See section 
   [#lint-configuration].
   1. Core Rust lints are defined under `workspace.lints.rust`.
   2. Rustdoc lints are defined under `workspace.lints.rustdoc`.
   3. Clippy lints are defined under `workspace.lints.clippy`.

See template file: [Cargo-workspace-root.toml](./Cargo-workspace-root.toml)

Ensure the following in each *member crate* `Cargo.toml`:

1. Include crate-specific metadata
   1. Required: name `name`, `version`, `description`, `documentation`, 
   2. Recommended: `categories`, and `keywords`.
   1. Note that `authors` is now deprecated.
   2. The key `readme` is unecessary if it can be inferred automatically from
      the project structure.
2. Common crate metadata defined at the workspace level should be included
   using the form `{name}.workspace = true`.
3. Include rustdoc configuration under the key `package.metadata.docs.rs`;
   see section [#rustdoc-configuration].
4. Include crate features in the usual manner.
5. Include any crate-specific dependencies in the usual manner.
6. Include linting configuration using the key `lints` and the key/value
   pair `workspace = true` to indicate that the crate should use the
   workspace-level lint configuration.

See template file: [Cargo-workspace-crate.toml](./Cargo-workspace-crate.toml)

## Single Crate Projects

Ensure the following in the `Cargo.toml`:

1. Include crate metadata
   1. Required: name `name`, `version`, `description`, `documentation`, 
      `edition`, `license`, and `repository`.
   2. Recommended: `categories`, `homepage`, and `keywords`.
   1. Note that `authors` is now deprecated.
   2. The key `readme` is unecessary if it can be inferred automatically from
      the project structure.
2. Include rustdoc configuration under the key `package.metadata.docs.rs`;
   see section [#rustdoc-configuration].
3. Include crate features in the usual manner.
4. Include any crate-specific dependencies in the usual manner.
5. Include linting configuration using the key `lints`. See section 
   [#lint-configuration].

See template file: [Cargo-crate.toml](./Cargo-crate.toml)

## Rustdoc Configuration

The targets array specifies which targets to build documentation for. Setting
it to an empty array means only the default target ("x86_64-unknown-linux-gnu")
will be documented. Unless you have different documentation on different
targets this can be a performance improvement.

```rust
targets = []
```

You can also select which features to include in your documentation, in general
its best to include all features to ensure comprehensive documentation.

```rust
all-features = true
```

Use the following to add custom stylesheets/scripts to the generated HTML files.
For example to include KaTeX for rendering mathematical expressions, or specific
web fonts, etc. Note that the paths should be relative to the workspace root
directory.

```rust
rustdoc-args = [ "--html-in-header", "doc-src/header.html" ]
```

## Lint Configuration

TBD