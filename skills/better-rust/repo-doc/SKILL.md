---
description: Guidelines for Repository (non-rustdoc) documentation.
name: repo-doc
context:
  applyTo: '**/*.md'
---
# Additional Repository Documentation

## Markdown Files

Any markdown documentation must pass the markdownlint tool, which is invoked
as follows.

```sh
mise :docs:lint
```

```sh
mise :readme:lint
```

The only exceptions are wide tables which must be column aligned and so tend to
be longer than the 80 column limit and so may be wrapped in disable/enable
markers as follows. Additionally, any verbatim text which should retain it's
format but overflows the 80 column limit may disable the warning.

```md
<!-- markdownlint-disable MD013 -->
<!-- markdownlint-enable MD013 -->
```

Note also, bulleted lists must use asterisks, not hyphens as bullets, and should
use semi-colons as term/definition separators where appropriate.

```md
* one; thing one
* two; thing two
```

## Using mdbook

TBD

Steps:

1. Create a new folder `book` in the root of your project.
2. Copy the file [dot-gitignore-mdbook](./dot-gitignore-mdbook) into the `book` folder as `.gitignore`.
3. Copy the file [mise-mdbook.toml](./mise-mdbook.toml) into the `book` folder as `mise.toml`.
4. Run the command `mise install` to install the necessary tools listed in `mise.toml`.
5. Initialize a new mdbook project in the `book` folder by running the shell
   command `mise book:init`.
   1. This should create a `src` folder with a `SUMMARY.md` file inside,
   2. and a `book.toml` file in the `book` folder,
   3. which should set `build-dir = "book"`.
6. You can now build the book by running `mise book:build` in the `book` folder.
