use std::time::{Instant};
use std::process::Command;
use std::{io, io::Write};
use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,LHSPart,LHSLiteralPart};
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
         FunctionDefinition::define( // 0 = $"+"
            vec![0,1],
            vec![Expression::li(vec![
               LIPart::variable(0),
               LIPart::variable(1),
            ],())]
         ),
         FunctionDefinition::define( // 1 = $"-"
            vec![0,1],
            vec![Expression::pattern(
               Expression::variable(0,()),
               vec![
                  (
                     LHSPart::ul(
                        vec![LHSLiteralPart::variable(1)],
                        Some(2),
                        vec![],
                     ),
                     Expression::variable(2,()),
                  ),
               ],
            ())],
         ),
         FunctionDefinition::define(
            vec![24],
            vec![Expression::pattern(
               Expression::variable(24,()),
               vec![
                  (
                     LHSPart::literal(""),
                     Expression::unary(b"1",()),
                  ),
                  (
                     LHSPart::Any,
                     Expression::apply(0,vec![
                        Expression::apply(2,vec![
                           Expression::apply(1,vec![
                              Expression::variable(24,()),
                              Expression::unary(b"1",()),
                           ],()),
                        ],()),
                        Expression::apply(2,vec![
                           Expression::apply(1,vec![
                              Expression::variable(24,()),
                              Expression::unary(b"1",()),
                           ],()),
                        ],()),
                     ],()),
                  ),
               ],
            ())],
         ),
      ],
      vec![
         Expression::apply(2,vec![
            Expression::variable(0, ())
         ],()),
      ],
   );
   JProgram::compile(&l12n)
}

pub fn main() {
    let start = Instant::now();
    for _ in 0..1000000 {
    for n in 0_u64..20_u64 {
       assert_eq!(
          2_u64.pow(n as u32),
          two_pow_n(n),
       );
    }}
    let t = start.elapsed();
    println!("(Rust) 1MM 2^20 in {} seconds", t.as_secs_f32());

    let j2n = l1_two_pow_n();
    let start = Instant::now();
    for _ in 0..1000 {
    for n in 0_u64..20_u64 {
       assert_eq!(
          2_u64.pow(n as u32),
          j2n.ueval(&[n]),
       );
    }}
    let t = start.elapsed();
    println!("(L1) 1M 2^20 in {} seconds", t.as_secs_f32());

    let output = Command::new("python3")
            .arg("benches/main.py")
            .output()
            .expect("failed to execute process");
    io::stdout().write(&output.stdout).unwrap();
}
