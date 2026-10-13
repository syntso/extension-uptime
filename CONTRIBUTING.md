# Contributing

## Branches

- `main` holds released code only. Every commit on it is a version.
- `dev` is where work comes together. Branch from it, and open pull requests into it.
- Name branches after the change: `feat/memory-usage`, `fix/busybox-uptime`.

Pull requests into `dev` are squash-merged, so the pull request title becomes the commit.

## Commits

[Conventional Commits](https://www.conventionalcommits.org), one line, no body.

```text
feat: report the number of logged-in users
fix: read uptime lines without a user count
ci: cache the cargo registry
```

- Types: `feat`, `fix`, `refactor`, `perf`, `test`, `docs`, `build`, `ci`, `chore`, `style`, `revert`.
- Scope: usually none.
- Subject: imperative, starts lowercase, no period, the whole line at most 72 characters.
- A breaking change gets a `!`: `feat!: rename the check action`.

`git config core.hooksPath .githooks` turns on the local check.

## Extension

- `extension.toml` is the manifest the registry reads. Bump `version` with every release, and keep `capabilities` to what the code calls.
- The SDK is pinned to a commit of [extension-api](https://github.com/syntso/extension-api). Move the pin on purpose, in its own pull request.
- Action ids start with `uptime.`. A new action that changes a host is `write`, not `read`.

## Code

- One main type per file, named after it. A concept with several files gets its own folder.
- Private by default. `pub` only for what the crate offers to others.
- Errors with `thiserror`. No `unwrap()` outside tests.
- Full words in names: `connection`, not `conn`.

## Shape of code

- No comments and no doc comments. Names, types and small functions carry the meaning.
- A blank line between logical steps; intermediate results get names.
- Guards first, the main path flat below.
- Imports grouped `std`, external crates, `crate::`.

## Before you push

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release --target wasm32-wasip2
```

CI runs the same commands on macOS, Windows and Linux, plus `cargo deny check` for dependency licenses and a check that the extension builds into a valid component.
