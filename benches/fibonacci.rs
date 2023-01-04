use criterion::{black_box, criterion_group, criterion_main, Criterion};
use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,LHSPart,LHSLiteralPart};
use l1_ir::opt::{JProgram};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn l1_fibonacci() -> JProgram {
   let l1fib = Program::program(
      vec![FunctionDefinition::define(
         vec![24],
         vec![Expression::pattern(
            Expression::variable(24,()),
            vec![
               (
                  LHSPart::literal(""),
                  Expression::unary(b"0",()),
               ),
               (
                  LHSPart::literal("0"),
                  Expression::unary(b"1",()),
               ),
               (
                  LHSPart::ul(
                     vec![LHSLiteralPart::literal("00")],
                     Some(2),
                     vec![],
                  ),
                  Expression::li(vec![
                     LIPart::expression(
                        Expression::apply(0,vec![
                           Expression::variable(2,()),
                        ],()),
                     ),
                     LIPart::expression(
                        Expression::apply(0,vec![
                           Expression::li(vec![
                              LIPart::literal("0"),
                              LIPart::variable(2),
                           ],()),
                        ],()),
                     ),
                  ],()),
               ),
            ],
         ())],
      )],
      vec![
         Expression::apply(0,vec![
            Expression::variable(0, ())
         ],()),
      ],
   );
   JProgram::compile(&l1fib)
}

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("(Rust) fib 40", |b| b.iter(|| fibonacci(black_box(40))));

    let jfib = l1_fibonacci();
    c.bench_function("(L1) fib 40", |b| b.iter(|| jfib.eval(black_box(&[40])).unwrap() ));
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
