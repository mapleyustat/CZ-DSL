# rust_core

The Rust frontend for CZ-DSL.

```bash
CARGO_HOME=$PWD/../../.cargo-home cargo test --offline
cargo run -- tokenize ../../cases/NewYorkHarbor2050/new_york_harbor_scenario.cz
```

## What is implemented

- A Unicode-aware lexer.
- A recursive-descent parser for the composite declarations.
- A CMECS unit registry.

The test suite (71 unit tests plus one doc test) passes, and the CLI tokenises
real scenarios.

## What is not implemented

There is **no semantic analyser and no code generator** in Rust. An earlier draft
of a code generator lived in `src/codegen/`; it was never declared in `lib.rs`,
referenced three backend modules that were never created, depended on an AST this
crate does not define, and did not compile. It has been deleted rather than
shipped. The paper describes the analyser and generator as intended future work,
not as current capability.

If you extend this crate, note that `lib.rs` declares only `lexer`, `parser` and
`cmecs`; add a module there or it will silently never be compiled.
