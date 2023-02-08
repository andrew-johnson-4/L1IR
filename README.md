# L1IR: Typed Glue

[![Crates.IO](https://img.shields.io/crates/v/l1_ir.svg)](https://crates.rs/crates/l1_ir)
[![Build](https://github.com/andrew-johnson-4/L1IR/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/L1IR)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/l1_ir/latest/l1_ir/)

Intermediate Representation of [LSTS](https://github.com/andrew-johnson-4/LSTS) [L1 Language](https://github.com/andrew-johnson-4/LSTS/blob/main/preludes/l1.tlc).
Built to support and manage JIT incremental compilation.
Efficiently build and evaluate code fragments so that you can generate even more code faster!

```
(Rust)   1M 2^20 in   4.8860006 seconds
(L1)     1M 2^20 in   3.8207040 seconds
(Python) 1M 2^20 in 130.2404361 seconds
(OCaml)  1M 2^20 in   2.5900000 seconds

note: Rust is compiled without optimizations enabled because it constant folds the entire benchmark
```

Values
* Literal Strings
* Tuples
* Functions

Gradual Types (optional)
* `T<A,B,C>` decorations for nominal accept/reject
* Regex for literal accept/reject
* Tuple/Functions for structural accept/reject
* Invariant properties accept/reject

Global AST Nodes
* Function Definitions
* Program Expressions

Expression AST Nodes
* Literal Introduction
* Tuple Introduction
* Variable Reference
* Function Reference
* Function Application
* Pattern Match
* Program Failure, Immediate Exit with possible Message

L1IR's unique contribution is that it does not presume to know everything about literal strings. Literal Values, by definition, are represented as an amalgam of Unicode Characters instead of fixed length bitstrings. This is advantageous to languages like L1 that define their own operators from scratch, but still desire to have an efficient runtime.

Things not in the AST directly
* If Expression (use a pattern)
* Struct Types (use a tuple)
* Tagged Enum Types (use tagged tuples)
* Field/Index Access (use a pattern)
* Polymorphic Functions (monomorphic definitions only)
* Stateful Closures (use a tuple with custom calling convention)
* Let Bindings (use a pattern, bound variables stick around until end of scope)
* Jumps or Loops (this is an IR for Functional Programming)
* Integers (use unary encoding "000"==3)
