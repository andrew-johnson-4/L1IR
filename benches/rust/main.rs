use std::time::{Instant};
use l1_ir::value::Value;
use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,Type};
use l1_ir::opt::{JProgram};

fn two_pow_n(n: u64) -> u64 {
    match n {
        0 => 1,
        n => two_pow_n(n-1) + two_pow_n(n-1),
    }
}

fn l1_two_pow_n() -> JProgram {
   let l12n = Program::program(
      vec![
         FunctionDefinition::define(
            "2^n",
            vec![(24,Type::nominal("U64"))],
            vec![Expression::pattern(
               Expression::variable(24,()).typed("U64"),
               vec![
                  (
                     LHSPart::literal("0"),
                     Expression::unary(b"1",()).typed("U64"),
                  ),
                  (
                     LHSPart::Any,
                     Expression::apply("+:(U64,U64)->U64",vec![
                        Expression::apply("2^n",vec![
                           Expression::apply("-:(U64,U64)->U64",vec![
                              Expression::variable(24,()),
                              Expression::unary(b"1",()),
                           ],()),
                        ],()),
                        Expression::apply("2^n",vec![
                           Expression::apply("-:(U64,U64)->U64",vec![
                              Expression::variable(24,()),
                              Expression::unary(b"1",()),
                           ],()),
                        ],()),
                     ],()).typed("U64"),
                  ),
               ],
            ()).typed("U64")],
         ),
      ],
      vec![
         Expression::apply("2^n",vec![
            Expression::variable(0, ())
         ],()),
      ],
   );
   JProgram::compile(&l12n)
}

pub fn main() {
    let start = Instant::now();
    for _ in 0..1000 {
    for n in 0_u64..20_u64 {
       two_pow_n(n);
    }}
    let t = start.elapsed();
    println!("(Rust) 1M 2^25 in {} seconds", t.as_secs_f32());

    let j2n = l1_two_pow_n();
    let start = Instant::now();
    for _ in 0..1000 {
    for n in 0_u64..20_u64 {
       j2n.eval(&[Value::u64(n,"U64")]);
    }}
    let t = start.elapsed();
    println!("(L1) 1M 2^25 in {} seconds", t.as_secs_f32());
}
