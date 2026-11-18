---
description: Using mise for Rust environment and tool management.
name: mise
context:
  applyTo: '**/mise.toml'
---
# Mise-en-place Environment and Tool Management

> mise installs a project's tools, sets its environment variables, and runs its
> tasks from one mise.toml that works in your shell, editor, and CI.

As such it is an excellent tool to manage your build environment and tools and
define reusable tasks for common workflows. This is my preferred approach to
environment configuration and task management (no more Makefiles).

## Installing mise Itself

Either:

```bash
❯ curl -fsSL https://mise.run | sh
```

or

```bash
❯ brew install mise
```

## Configuration

### Workspace Projects

Copy the configuration from the file 
[mise-workspace-root.toml](./mise-workspace-root.toml)
to the root of your workspace and name it simply `mise.toml`. You will need to
rename the members of the `config_roots` array to match the actual directories
of your workspace crates.

Now, copy the file [mise-workspace-crate.toml](./mise-workspace-crate.toml) to
a file named `.mise/rust-package.toml` in your project. Each member crate in
the workspace then has a `mise.toml` which is a symlink to this file. Now each
member crate will use the shared configuration *actually* defined in
`mise.toml`.

### Individual Crate Projects

Copy the file [mise-single-crate.toml](./mise-single-crate.toml) to the root of
the project and name it `mise.toml`.
