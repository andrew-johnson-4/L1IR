
//AST Definition
pub mod ast;

//Reference Implementation of Program Evaluation
pub mod eval;

//Optimizing Compiler, JIT and Otherwise
#[cfg(feature = "cranelift")]
pub mod opt;
