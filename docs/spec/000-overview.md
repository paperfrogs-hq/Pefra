# Pefra language overview

**Status:** Provisional design overview. This document does not define executable syntax or normative language semantics.

## Goals

Pefra is intended to be a readable, general-purpose language with static typing and type inference, predictable behavior, useful diagnostics, and practical systems programming capabilities. Memory safety, native compilation, and numerical computing are research and implementation goals that require explicit designs and verification.

## Current status

The repository contains a Rust workspace and a CLI that reports help and version information. There is no Pefra lexer, parser, type checker, interpreter, or compiler. No `.pf` program runs yet.

## Initial non-goals

The first language version does not require native compilation, LLVM, ownership enforcement, concurrency, generics, a package manager, a formatter, a language server, or AI/ML features. Performance and safety guarantees cannot be inferred from the implementation language.

## Provisional direction

The proposed source extension is `.pf`. The implementation is written in Rust. The intended development sequence is specification, lexer and source spans, parser and AST, a small tree-walk interpreter, then type checking. A future IR and native backend need separate decisions. The syntax examples in project discussions are illustrative and do not settle grammar or semantics.

M1 must decide source encoding, grammar, typing, numeric behavior, scope, evaluation order, errors, and the program entry point before their implementation. Decisions should be recorded and reflected in the approved specification.
