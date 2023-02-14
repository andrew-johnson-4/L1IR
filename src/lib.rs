#[macro_export]
macro_rules! dprintln {
   ( $( $x:expr ),* ) => {
      if cfg!(debug_assertions) {
         println!($(
            $x,
         )*);
         std::io::stdout().flush().expect("dprintln .flush failed");
      }
   };
}

//AST Definition
pub mod ast;

//Reference Implementation of Program Evaluation
pub mod eval;

//Intermediate Representation of Values
pub mod value;

//Optimizing Compiler, JIT and Otherwise
pub mod opt;

//Hardcoded Equivalence Relations
pub mod recipes;
