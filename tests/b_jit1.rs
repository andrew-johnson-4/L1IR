use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,Value,LHSPart,LHSLiteralPart};
use l1_ir::opt::{JProgram};

#[test]
fn eval_match1() {
   let nojit = Program::program(
      vec![],
      vec![Expression::pattern(
         Expression::variable(0,()),
         vec![
            (
               LHSPart::literal("000"),
               Expression::unary(b"123",()),
            ),
            (
               LHSPart::Any,
               Expression::unary(b"321",()),
            ),
         ],
      ())],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
      let jval = jit.eval(&[x]).unwrap();
      assert_eq!(Value::from_u64(if x==3 {123} else {321}), jval, "if {}==3 then 123 else 321", x);
   }
}
#[test]
fn eval_match2() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![24],
         vec![Expression::pattern(
            Expression::variable(24,()),
            vec![
               (
                  LHSPart::literal("000"),
                  Expression::unary(b"123",()),
               ),
               (
                  LHSPart::Any,
                  Expression::unary(b"321",()),
               ),
            ],
         ())],
      )],
      vec![
         Expression::apply(0,vec![
            Expression::variable(0, ()),
         ],()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
      let jval = jit.eval(&[x]).unwrap();
      assert_eq!(Value::from_u64(if x==3 {123} else {321}), jval, "if {}==3 then 123 else 321", x);
   }
}

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

#[test]
fn eval_two_pow_n() {
   let jit = l1_two_pow_n();

   for x in 0..20 {
      let jval = jit.eval(&[x]).unwrap();
      assert_eq!(Value::from_u64(rust_two_pow_n(x)), jval, "fibonacci({})", x);
   }
}

fn rust_two_pow_n(n: u64) -> u64 {
    match n {
        0 => 1,
        n => rust_fibonacci(n-1) + rust_fibonacci(n-1),
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
                        Expression::apply(1,vec![
                           Expression::variable(24,()),
                           Expression::unary(b"1",()),
                        ],()),
                        Expression::apply(1,vec![
                           Expression::variable(24,()),
                           Expression::unary(b"1",()),
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
