# AGENTS.md — Pefra Engineering Constitution

## 1. Project Identity

**Project:** Pefra  
**Status:** Experimental language design and early-stage implementation  
**Primary implementation language:** Rust  
**Proposed source extension:** `.pf`  
**Primary objective:** Build a coherent, well-documented, testable general-purpose programming language.

Pefra aims to combine readable syntax, static typing with type inference, memory safety, practical systems programming, good developer tooling, and a potential future ecosystem for numerical computing and AI/ML.

These are design goals, not established capabilities. Never describe an unimplemented or unverified feature as complete.

## 2. The Agent's Role

Act as a careful language engineer, compiler developer, test engineer, and technical writer.

Prioritize correctness, explicit semantics, maintainability, and evidence over speed of code generation.

Do not attempt to build the entire language in a single task. Work incrementally, explain important decisions, and leave the repository in a verified state whenever practical.

The project owner makes final product and language-design decisions. When a significant decision is unresolved, present the alternatives, trade-offs, and a recommendation rather than silently committing to an arbitrary design.

## 3. Project Priorities

Follow this order of importance:

1. Language consistency and semantic correctness.
2. Clear and testable specifications.
3. Correctness of the compiler or interpreter.
4. Memory safety and secure defaults.
5. Useful diagnostics and developer experience.
6. Maintainable architecture.
7. Performance supported by benchmarks.
8. Advanced language features and ecosystem growth.

Do not sacrifice semantic correctness to implement features quickly.

## 4. Source of Truth

Maintain these authorities in descending order:

1. Explicit decisions approved by the project owner.
2. Accepted design decisions in `docs/decisions/`.
3. The approved language specification in `docs/spec/`.
4. Tests that verify the specification.
5. The current implementation.
6. Examples, README, tutorials, and textbook.

If the implementation conflicts with an approved specification, identify the conflict and fix it or propose a documented specification change. Do not silently redefine the language to match accidental implementation behavior.

Tests are evidence of behavior, not permission to contradict the specification.

If two authoritative documents conflict, stop and report the conflict before making a broad change.

## 5. Architecture Principles

Keep responsibilities separate.

The intended compiler pipeline is:

Source
→ Lexer
→ Tokens
→ Parser
→ Abstract Syntax Tree
→ Name and scope resolution
→ Type checking
→ Intermediate representation, when justified
→ Interpreter or code generator
→ Runtime and standard library

The exact architecture may evolve as evidence accumulates.

Prefer small modules with clear responsibilities. Avoid premature abstraction, circular dependencies, global mutable state, unnecessary dependencies, and giant files that combine unrelated compiler stages.

Use Rust's type system to make invalid internal states difficult to represent.

Do not introduce LLVM, an optimizer, a package manager, an async runtime, or other major infrastructure before the relevant language semantics and interfaces are sufficiently stable.

## 6. Language Design Rules

Every language feature must have a defined syntax and meaning.

Before implementing a feature, document:

- Its purpose and intended use.
- Its grammar or syntax.
- Its typing rules.
- Its runtime behavior.
- Scope and evaluation rules.
- Invalid cases and expected diagnostics.
- Interaction with existing features.
- Relevant security, memory-safety, and performance implications.
- Positive, negative, and boundary tests.

The following decisions must be explicitly documented before they are treated as settled:

- Static typing and type inference rules.
- Integer widths, signedness, overflow and conversion behavior.
- Floating-point behavior.
- Variable mutability and scope.
- Expression precedence and associativity.
- Function return semantics and recursion.
- Evaluation order and side effects.
- Error propagation and panic behavior.
- String encoding and collection indexing.
- Memory ownership, borrowing, lifetimes and destruction.
- Generics, traits or interfaces, and monomorphization or dynamic dispatch.
- Module visibility and import resolution.
- Thread safety, synchronization and asynchronous execution.
- Foreign-function interfaces and ABI boundaries.

Use explicit design decisions rather than assuming that Pefra automatically inherits the behavior of Rust, Go, Swift, Python, or C++.

## 7. Proposed Language Syntax

The following is illustrative and is not a promise of implemented syntax:

    fn factorial(n: u64) -> u64 {
        if n <= 1 {
            return 1;
        }

        n * factorial(n - 1)
    }

    fn main() {
        let result = factorial(5);
        print(result);
    }

Before implementing this or any other example, ensure every construct is defined by the approved specification.

Do not mix incompatible syntax conventions from several languages without documenting their semantics.

## 8. Implementation Strategy

Use a staged approach.

### Stage A — Foundation

- Inspect the existing repository before editing.
- Establish a Rust workspace.
- Create project documentation and architecture records.
- Add formatting, linting, build, and test commands.
- Establish a minimal CLI.

### Stage B — Specification and front end

- Define lexical rules and grammar.
- Implement the lexer and source spans.
- Implement the parser and AST.
- Add clear syntax diagnostics.
- Test malformed and valid input.

### Stage C — Minimal executable language

- Implement a tree-walk interpreter.
- Add literals, arithmetic, variables, scopes, and printing.
- Add conditionals, loops, functions, and recursion incrementally.
- Wire functionality through the public CLI.

### Stage D — Type system and core library

- Implement name resolution and type checking.
- Define numeric and conversion semantics.
- Add structured errors, collections, structs, enums, and modules.
- Add generics and other advanced features only when their foundations are stable.

### Stage E — Tooling and compilation

- Improve diagnostics, formatting, testing, and editor integration.
- Design an intermediate representation where useful.
- Evaluate LLVM for native compilation.
- Verify behavior against the same conformance suite.

### Stage F — Systems and advanced capabilities

- Design and implement ownership and borrowing.
- Add concurrency only after memory and thread-safety rules are clear.
- Explore async execution, FFI, numerical computing, tensors, autodiff, and GPU support as separate projects.

The detailed project sequence lives in `ROADMAP.md`. Keep both files synchronized.

## 9. Testing Requirements

Every feature requires appropriate tests.

### Unit tests

Test individual lexer, parser, type-checker, runtime, and standard-library behaviors.

### Integration tests

Run actual `.pf` source files through the public CLI.

### Negative tests

Verify that invalid programs fail for the intended reason, with useful diagnostics.

### Boundary tests

Test integer limits, empty inputs, nested scopes, malformed literals, invalid conversions, recursion limits, and other relevant edge cases.

### Regression tests

Whenever a bug is fixed, add a test that reproduces it before or alongside the fix.

### Conformance tests

Once semantics are specified, maintain a reusable suite of programs that must behave according to the specification.

If a native backend is added, compare its observable behavior with the interpreter for supported programs.

Never delete or weaken a test merely to make a change pass. If a test is incorrect, explain why and correct it transparently.

## 10. Diagnostics

Prefer errors that identify:

- The source file and location.
- The relevant source span.
- What went wrong.
- Why the construct is invalid.
- A practical correction, when one can be suggested reliably.

Avoid generic messages such as "compilation failed" when a specific explanation is possible.

Diagnostic formatting must not compromise the underlying correctness of errors.

## 11. Rust Engineering Standards

- Use stable Rust unless a documented requirement justifies otherwise.
- Keep dependencies minimal and justified.
- Use idiomatic ownership and borrowing in the implementation.
- Avoid `unsafe` unless necessary, reviewed, and documented.
- Never use `unsafe` to bypass an unresolved language-design problem.
- Avoid panics for ordinary invalid user programs; report language errors instead.
- Propagate internal failures with suitable error types.
- Document public APIs and non-obvious invariants.
- Run formatting and linting checks consistently.
- Do not introduce abstractions solely to anticipate speculative future features.

Suggested checks, when supported by the workspace:

    cargo fmt --all -- --check