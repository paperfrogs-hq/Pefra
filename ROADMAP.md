# Pefra — Project Roadmap

**Project status:** Experimental language design  
**Roadmap version:** 0.1  
**Current milestone:** M0 — Repository and engineering foundation  
**Implementation language:** Rust  
**Proposed source extension:** `.pf`

**M0 status:** Verified. The Rust workspace, CLI help/version, repository documentation, ignore rules, tests, and CI configuration are in place. Local formatting, Clippy, test, and build checks passed. CI has been configured but has not yet been run on GitHub. M1 has not started.

---

## 1. Vision

Pefra is an experimental general-purpose programming language intended to offer:

- Readable syntax and a productive development experience.
- Static typing with type inference.
- Explicit and predictable runtime semantics.
- Strong safety guarantees, subject to a formally designed memory model.
- Practical systems-programming capabilities.
- Useful diagnostics, testing, formatting, and editor tooling.
- A potential native-compilation backend.
- A long-term ecosystem for numerical computing and AI/ML.

These are goals. The roadmap does not imply that any capability already exists.

Pefra must earn its claims through implementation, conformance tests, benchmarks, and documented limitations.

## 2. Guiding Principles

1. Specify before implementing.
2. Build a small working language before a large incomplete one.
3. Keep the specification, tests, implementation, and textbook synchronized.
4. Prefer clear semantics over superficial syntax similarities.
5. Treat safety, concurrency, and numeric behavior as language-design problems.
6. Add advanced infrastructure only when its dependencies are stable.
7. Measure performance rather than guessing.
8. Keep milestones small enough to review and verify.

## 3. Initial Architecture

The intended architecture is:

    Source file
        |
        v
    CLI and source loader
        |
        v
    Lexer
        |
        v
    Tokens with source spans
        |
        v
    Parser
        |
        v
    Abstract Syntax Tree
        |
        v
    Name resolution
        |
        v
    Type checking
        |
        +--------------------+
        |                    |
        v                    v
    Interpreter         Future IR
                             |
                             v
                      Future LLVM backend
                             |
                             v
                      Native executable

This architecture is provisional. Adjust it when a documented engineering reason justifies the change.

The initial implementation should prioritize the lexer, parser, diagnostics, interpreter, and type checker.

## 4. Milestone Overview

### M0 — Repository and engineering foundation

**Objective:** Establish a clean, reproducible development environment.

Tasks:
- Create a Rust workspace.
- Add `AGENTS.md`, `README.md`, and this roadmap.
- Create `docs/spec/`, `docs/decisions/`, and `docs/book/`.
- Configure Git ignore rules.
- Add a minimal CLI.
- Establish formatting, linting, build, and test commands.
- Add continuous integration where practical.

Acceptance criteria:
- A clean checkout builds.
- The CLI can display help and version information.
- Tests run successfully.
- Documentation explains the project's experimental status.
- No language feature is claimed as implemented without evidence.

### M1 — Language design and specification v0.1

**Objective:** Define the smallest coherent language that can be implemented and tested.

Specify:
- Character and source encoding rules.
- Identifiers, keywords, comments, and whitespace.
- Numeric, string, Boolean, and unit literals.
- Operators, precedence, and associativity.
- Expressions and statements.
- Variables, mutability, and lexical scopes.
- Functions, parameters, return values, and recursion.
- Conditional and loop semantics.
- Integer widths, conversions, overflow, and division behavior.
- Evaluation order and side effects.
- Program entry point and basic output.
- Compile-time versus runtime errors.

Deliverables:
- Normative specification documents.
- A grammar for the initial supported syntax.
- Valid and invalid examples.
- A list of unresolved design decisions.
- A small language conformance test plan.

Acceptance criteria:
- Every construct in the MVP examples has defined semantics.
- Numeric and control-flow edge cases are explicitly addressed.
- No contradictory examples remain in the initial specification.
- The project owner has reviewed the core design.

### M2 — Lexer

**Objective:** Convert source text into a reliable token stream.

Tasks:
- Define token types.
- Track byte offsets and source locations.
- Recognize identifiers, keywords, literals, punctuation, and operators.
- Handle whitespace and comments.
- Report malformed literals and unexpected characters.
- Establish a strategy for Unicode identifiers, or explicitly restrict them.

Acceptance criteria:
- Unit tests cover every supported token.
- Invalid input produces structured lexical errors.
- Source spans identify the relevant text.
- Tokenization follows the specification exactly.

### M3 — Parser and AST

**Objective:** Convert tokens into structured programs.

Tasks:
- Define AST node types.
- Implement expression parsing.
- Enforce operator precedence and associativity.
- Parse statements, scopes, function declarations, and calls.
- Implement error recovery where practical.
- Preserve source spans for diagnostics.

Acceptance criteria:
- Valid MVP programs parse.
- AST tests verify representative structures.
- Invalid programs produce useful syntax errors.
- Parser behavior is consistent with the published grammar.

### M4 — Interpreter MVP

**Objective:** Execute a minimal Pefra program.

Initial feature set:
- Integer and Boolean values.
- Arithmetic and comparisons.
- Variable declarations and assignment.
- Lexical scopes.
- Printing.
- Conditional statements.
- Loops.
- Function declarations and calls.
- Return values.
- Recursion.

CLI target:

    pefra run examples/hello.pf

Proposed program example, subject to the approved specification:

    fn main() {
        let answer = 40 + 2;
        print(answer);
    }

Acceptance criteria:
- At least one real `.pf` file executes through the public CLI.
- Expected output is verified by an integration test.
- Function calls and recursion work.
- Runtime failures produce defined diagnostics rather than unexplained internal crashes.
- No unsupported syntax is silently accepted.

### M5 — Static type system

**Objective:** Reject invalid programs before execution wherever the specification requires compile-time checking.

Tasks:
- Define primitive types.
- Implement type checking for expressions and statements.
- Define type annotations and inference.
- Validate function arguments and return values.
- Define variable initialization and assignment rules.
- Define conversion rules.
- Specify overflow, division-by-zero, and numeric boundary behavior.
- Add type errors with source spans.

Acceptance criteria:
- Positive tests demonstrate accepted programs.
- Negative tests demonstrate rejected programs.
- Inference behavior is documented.
- Runtime and compile-time responsibilities are unambiguous.

### M6 — Core language features

**Objective:** Support reusable, expressive programs.

Implement in dependency order:
1. Strings and string operations.
2. Arrays and indexing.
3. Tuples and basic collections, if selected.
4. Structs and field access.
5. Enums and exhaustive pattern matching.
6. `Option` and `Result`-style types.
7. Modules and imports.
8. Generics.
9. Traits or interfaces, after a deliberate design decision.
10. Closures and iterators, once function and type semantics support them.

Acceptance criteria:
- Every feature has specification coverage.
- Tests include invalid usage and boundary cases.
- Examples compile or execute as documented.
- Features do not create unresolved contradictions in the type system.

### M7 — Standard library and project tooling

**Objective:** Make Pefra practical for small projects.

Potential deliverables:
- Basic I/O.
- String and collection utilities.
- File access under explicit runtime rules.
- Error-handling utilities.
- Unit-test support.
- Source formatter.
- Improved CLI diagnostics.
- Editor integration and language server.
- Project manifests and dependency management.

Acceptance criteria:
- Tool behavior is documented and tested.
- Standard-library interfaces are versioned with the language.
- Project configuration is deterministic.
- External dependencies are handled transparently.

Do not build a package registry before the module system and package format are stable.

### M8 — Compiler intermediate representation

**Objective:** Prepare the language for a native backend without destabilizing its semantics.

Tasks:
- Evaluate whether an IR is necessary for the current design.
- Define IR operations and invariants.
- Represent control flow explicitly.
- Define how types and source locations flow into the IR.
- Test IR construction and validation.
- Separate front-end semantics from backend-specific concerns.

Acceptance criteria:
- The IR has documented invariants.
- Invalid IR is detected during development.
- Existing interpreter behavior remains unchanged.
- The design is sufficient for a minimal native compilation experiment.

### M9 — Native compilation

**Objective:** Compile supported Pefra programs into native executables.

Tasks:
- Evaluate LLVM against project complexity and target requirements.
- Define supported target platforms.
- Generate LLVM IR.
- Implement function and control-flow lowering.
- Handle runtime support, linking, and target ABI requirements.
- Add optimization only when it can be verified.
- Establish native-code integration tests.

Acceptance criteria:
- A supported Pefra program produces a native executable.
- Interpreter and native output agree on the conformance suite.
- Failures in unsupported features are explicit.
- Performance comparisons use reproducible benchmarks.

### M10 — Memory model and safety

**Objective:** Establish enforceable memory-safety guarantees.

Tasks:
- Specify ownership and borrowing rules.
- Define reference validity and aliasing behavior.
- Decide whether and how lifetimes appear in syntax or type checking.
- Specify moves, copies, destructors, and resource release.
- Define the interaction between generics, closures, and ownership.
- Define unsafe and foreign-function boundaries, if any.
- Add compile-fail tests and runtime safety tests.

Acceptance criteria:
- Safety guarantees are written precisely.
- Invalid ownership patterns are rejected as specified.
- Resource destruction behavior is tested.
- Any unsafe escape hatch is limited, documented, and reviewed.

This is a major design milestone. Do not claim Rust-equivalent safety merely because the implementation uses Rust.

### M11 — Concurrency and asynchronous execution

**Objective:** Support reliable concurrent programs.

Prerequisites:
- Stable types and runtime semantics.
- A documented memory and thread-safety model.
- Defined ownership and resource-lifetime behavior.

Potential features:
- Threads.
- Message passing and channels.
- Synchronization primitives.
- Shared state with defined safety constraints.
- Structured concurrency.
- Async functions and task scheduling, if justified.

Acceptance criteria:
- Concurrency semantics are documented.
- Tests cover synchronization and relevant race conditions.
- Thread-safety guarantees are enforced rather than assumed.
- Cancellation and resource cleanup are specified before async execution is considered stable.

### M12 — Systems interoperability

**Objective:** Enable integration with existing software.

Potential features:
- C-compatible FFI.
- Explicit ABI and data-layout rules.
- System interfaces and file/network I/O.
- Cross-compilation.
- Debug symbols and useful stack traces.
- Platform-specific capabilities behind documented interfaces.

Acceptance criteria:
- Interoperability boundaries are tested.
- Unsafe operations are isolated and documented.
- Platform limitations are explicit.
- Build and linking instructions are reproducible.

### M13 — Numerical computing and AI/ML

**Objective:** Explore a meaningful numerical-computing ecosystem after the core language is stable.

Potential features:
- Numeric traits and generic arithmetic.
- Efficient multidimensional arrays and tensors.
- Linear algebra interfaces.
- SIMD support.
- Automatic differentiation.
- GPU execution and accelerator integration.
- Interoperability with established ML libraries.
- Dataset, serialization, and model-loading utilities where appropriate.

Acceptance criteria:
- APIs have explicit numeric semantics.
- Tests cover precision, shapes, overflow, and numerical edge cases.
- Benchmarks compare equivalent workloads fairly.
- GPU support is claimed only for tested configurations.

These features are long-term research goals, not requirements for the first usable language.

### M14 — Release engineering

**Objective:** Prepare a reproducible public release.

Tasks:
- Define semantic-versioning policy.
- Document supported platforms.
- Add release builds and distribution instructions.
- Publish a language reference and tutorial.
- Provide a conformance test suite.
- Document breaking changes and known limitations.
- Establish a bug-reporting and contribution workflow.

Acceptance criteria:
- A new developer can build the project from a clean checkout.
- The release process is repeatable.
- The supported feature set is accurately described.
- Known limitations and security boundaries are published.

## 5. Feature Status Policy

Every feature should have one of these statuses:

- **Proposed:** under consideration.
- **Specified:** defined in an approved specification.
- **Implemented:** present in working code.
- **Verified:** tested against documented acceptance criteria.
- **Experimental:** implemented but unstable.
- **Deferred:** deliberately postponed.

Do not use "complete" as a substitute for verification evidence.

## 6. Required Testing Strategy

Each milestone must add the tests needed for its features.

### Unit tests
Test individual components in isolation.

### Integration tests
Run real source files through the public CLI.

### Negative tests
Verify invalid source is rejected for the intended reason.

### Boundary tests
Test limits, empty inputs, malformed source, numeric boundaries, nested scopes, and resource limits.

### Regression tests
Preserve a test for every fixed bug.

### Conformance tests
Maintain a shared suite that specifies observable behavior independently of implementation details.

### Backend equivalence tests
When native compilation exists, run the same supported programs through both execution engines and compare observable results.

## 7. Documentation and Textbook

Keep the textbook in `docs/book/` and maintain a separate normative specification in `docs/spec/`.

The specification defines the language. The textbook teaches it.

The book should progressively cover:
- Getting started.
- Lexical structure and grammar.
- Types and expressions.
- Control flow, functions, and recursion.
- Collections and data structures.
- Error handling.
- Modules and generics.
- Memory and ownership.
- Concurrency.
- Standard-library usage.
- Algorithms and practical projects.
- Compiler architecture.
- Native compilation.
- Numerical computing and AI/ML.
- Appendices, exercises, and reference material.

Clearly label features that are not yet implemented. Update examples when the specification changes. Do not inflate page count with filler or present proposed syntax as tested behavior.

## 8. Milestone Workflow

For each milestone:

1. Review current repository state.
2. Read `AGENTS.md`, the relevant specification, and design decisions.
3. Identify dependencies and acceptance criteria.
4. Plan a small implementation task.
5. Implement the smallest coherent change.
6. Add positive, negative, and boundary tests as appropriate.
7. Update the specification and documentation.
8. Run formatting, linting, builds, and tests.
9. Review the Git diff.
10. Record remaining limitations and the next task.

Do not advance merely because files were generated. Advance when acceptance criteria are met.

## 9. First Three Tasks

### Task 1
Establish the repository foundation, Rust workspace, minimal CLI, project instructions, documentation directories, and automated checks.

### Task 2
Draft and review the initial normative specification. Resolve the essential syntax, type, scope, evaluation, and numeric-semantics decisions.

### Task 3
Implement the lexer and tests against the approved lexical specification.

After those tasks pass, proceed to parsing, the AST, and the first executable language vertical slice.

## 10. Explicit Non-Goals for the Initial MVP

The initial MVP does not require:
- LLVM or native compilation.
- Ownership and borrowing enforcement.
- Threads or async execution.
- Generics or advanced trait systems.
- A package registry.
- GPU or AI/ML functionality.
- Production-grade cross-compilation.
- Claims of superior performance or safety.

These features must be evaluated when their prerequisites are ready.

## 11. Success Criteria

The project is progressing successfully when:

- The language has a coherent, versioned specification.
- The implementation matches that specification.
- Small programs execute correctly.
- Invalid programs receive useful errors.
- Regression and conformance tests catch unintended changes.
- The compiler architecture remains understandable.
- Documentation accurately reflects implemented behavior.
- Advanced capabilities are added without destabilizing the core.

**The first major success is a small, well-specified language that works reliably—not a large collection of unfinished features.**
