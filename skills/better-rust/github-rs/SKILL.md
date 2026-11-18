---
description: GitHub configuration for Rust projects.
name: github-rs
---
# GitHub Configuration for Rust Projects


```text
.github/
├─ workflows/
│  ├─ rust.yml
│  ├─ security-audit.yml
│  └─ typos.yml
├─ dependabot.yml
└─ mergify.yml
```

*Principles*

* Keep CI workflows modular and focused on specific tasks.
  * break down complex workflows into smaller, manageable steps, visualize
    dependencies and execution flow.
* Ensure code quality and security through dedicated workflows and tools.
* Regularly review and update workflows to adapt to project changes and
  improvements.
* Automate dependency management with Dependabot.
  * Use Mergify to streamline pull request management.

## Rust Workflow

File: [rust.yml](./rust.yml)

## Security Audit Workflow

The security audit workflow ensures that the Rust project dependencies and 
code are regularly checked for vulnerabilities and potential security issues.
It relies on the `cargo audit` tool to perform security checks on the project's
dependencies and `cargo deny` to enforce license policies.

File: [security-audit.yml](./security-audit.yml)

To control the behavior of the cargo deny tool you set policies in the 
`deny.toml` configuration file. A typical configuration is included in the file
[deny.toml](./deny.toml).

## Typos Workflow

File: [typos.yml](./typos.yml)

## Dependabot & Mergify

```yaml
updates:
  - package-ecosystem: cargo
    directory: "/"
    schedule:
      interval: daily
    open-pull-requests-limit: 10
```

Files: [dependabot.yml](./dependabot.yml) and [mergify.yml](./mergify.yml)

## Additional Tools

TBD