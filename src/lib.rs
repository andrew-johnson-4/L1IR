#![feature(strict_provenance)]

#[macro_export]
macro_rules! dprintln {
   ( $( $x:expr ),* ) => {
      println!($(
        $x,
      )*);
      std::io::stdout().flush().expect("some error message");
   };
}

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
