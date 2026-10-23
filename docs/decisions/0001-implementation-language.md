# 0001 — Rust as implementation language

**Status:** Accepted for the M0 implementation

## Context

Pefra needs a maintainable implementation for its CLI and future language tools. The roadmap names Rust as the primary implementation language. This choice concerns the tools that implement Pefra, not the design of Pefra itself.

## Decision

Use stable Rust and Cargo for the initial workspace and CLI. Keep external dependencies minimal and add crates only when responsibilities justify them.

## Alternatives and trade-offs

C or C++ could provide broad compiler tooling and control, but would add manual memory-management concerns to the implementation. A higher-level language such as Python could ease prototyping, but would change distribution and runtime requirements for the CLI. Rust offers a compiled CLI, Cargo tooling, and compile-time ownership checks for the implementation. It also brings a learning curve and potentially longer build times. None of these properties guarantees Pefra program safety or performance.

## Consequences

The repository uses a Cargo workspace and stable Rust checks. Future language rules must be specified independently rather than copied from Rust. A later implementation-language change would require a new decision record.
