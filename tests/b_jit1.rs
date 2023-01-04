use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,Value,LHSPart,LHSLiteralPart};
use l1_ir::opt::{JProgram};

#[test]
fn eval_add() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![0,1],
         vec![Expression::li(vec![
            LIPart::variable(0),
            LIPart::variable(1),
         ],())]
      )],
      vec![
         Expression::apply(0,vec![
            Expression::variable(0, ()),
            Expression::variable(1, ()),
         ],()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
   for y in 0..20 {
      let jval = jit.eval(&[x, y]).unwrap();
      assert_eq!(Value::from_u64(x + y), jval, "{} + {}", x, y);
   }}
}

#[test]
fn eval_fibonacci() {
   let jit = l1_fibonacci();

   for x in 0..20 {
      let jval = jit.eval(&[x]).unwrap();
      assert_eq!(Value::from_u64(rust_fibonacci(x)), jval, "fibonacci({})", x);
   }
}

fn rust_fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => rust_fibonacci(n-1) + rust_fibonacci(n-2),
    }
}fn l1_fibonacci() -> JProgram {
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
