#![feature(strict_provenance)]

#[cfg(feature = "cranelift")]
#[macro_use]
extern crate lazy_static;

//AST Definition
pub mod ast;

//Reference Implementation of Program Evaluation
pub mod eval;

//Intermediate Representation of Values
pub mod value;

//Optimizing Compiler, JIT and Otherwise
#[cfg(feature = "cranelift")]
pub mod opt;

//Hardcoded Equivalence Relations
pub mod recipes;
