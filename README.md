# Pefra

**An experimental programming language by [Paperfrogs](https://paperfrogs.dev/).**

Pefra aims for readable syntax, static typing with inference, explicit semantics, useful diagnostics, and eventually practical systems programming and numerical computing. These are design goals, not available language features.

## Current status

M0 establishes the Rust project and a small `pefra` CLI. The CLI displays help and version information. It cannot read or execute `.pf` files yet. The syntax and semantics are still proposals; see the [overview](docs/spec/000-overview.md).

## Prerequisites and build

Install stable Rust with Cargo, rustfmt, and Clippy. From the repository root:

```sh
cargo build --workspace
cargo run -p pefra -- --help
cargo run -p pefra -- --version
```

Run the project checks with:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

## Planned milestones

M1 defines an initial language specification. M2 adds a lexer, M3 a parser, M4 an interpreter, and M5 static type checking. Later milestones cover core features, tooling, native compilation, and further research.

## Project information

- **Maintainer:** [Paperfrogs](https://paperfrogs.dev/)
- **License:** [Proprietary; all rights reserved by Paperfrogs HQ](LICENSE)
- **Code of conduct:** [Community guidelines](CODE_OF_CONDUCT.md)
- **Security:** [Vulnerability reporting policy](SECURITY.md)
