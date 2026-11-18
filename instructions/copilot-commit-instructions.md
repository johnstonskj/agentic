---
name: conv-commits
description: Conventional Commit Guidance
---

# Conventional Commit Guidance

When constructing Git commit messagesm **always** follow the Conventional
Commits specification strictly. The template is as follows.

```text
type(scope): Summary
# <type>(<scope>)(!): <short summary>
#   │       │     │       │
#   │       │     │       └─⫸ Short summary,
#   │       │     │            not capitalized,
#   │       │     │            no period at the end,
#   │       │     │            use the imperative, present tense.
#   │       │     │
#   │       │     └─⫸ Breaking change flag.
#   │       │
#   │       └─⫸ Commit Scope: project-dependent
#   │
#   └─⫸ Commit Type: feat|fix
#                  | build|chore|docs|ops|perf|refactor|style|test
#----------------------------------MAX LENGTH (52)  ^

# Body text is usually a numbered list, which may include (but if used,
# should start with) a keyword such as [Add] etc.
# 
# Use the imperative, present tense: "change" not "changed" not 
# "changes"
#------------------------------------------------------MAX LENGTH (72)  ^

# Add Footer values (name: value) for additional information
# - BREAKING-CHANGE
# - About people: Signed-off-by, Reviewed-by, Co-authored-by
# - About commits: Amend, Ref, Revert, See-Also
# - About issues: Fix, Resolve, Close, See-Also
# - More information: Specification, See-Also
```

Note that the two blank lines in the template represent blank lines that should
be present in an actual message, and comments start with the `#` character and
continue to the end of the line.
