---
description: Using the bacon watch-mode tool in Rust projects.
name: use-bacon
context:
  applyTo: '**/bacon.toml'
---
# Using Bacon for Watch Mode

TBD

```sh
bacon
```

## Install

Ensure the following is included in your project level `mise.toml`.

```toml
[tools]
"cargo:bacon" = "latest"
```

## Configuration

The file [bacon.toml](./bacon.toml) contains the configuration for the bacon
tool, place a copy of it at the root of your project to customize the behavior
of bacon.
