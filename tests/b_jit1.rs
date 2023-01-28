use l1_ir::value::Value;
use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,LHSPart,LHSLiteralPart,Type};
use l1_ir::opt::{JProgram};

/* TODO FIXME use new fnid system
#[test]
fn eval_echo() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::variable(0, ()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
      let jval = jit.eval(&[Value::u64(x,"U64")]);
      assert_eq!(Value::u64(x,"U64"), jval, "{}", x);
   }
}

#[test]
fn eval_match1() {
   let nojit = Program::program(
      vec![],
      vec![Expression::pattern(
         Expression::variable(0,()),
         vec![
            (
               LHSPart::Any,
               Expression::unary(b"321",()),
            ),
         ],
      ())],
   );
   let jit = JProgram::compile(&nojit);

   for x in 1..20 {
      let jval = jit.eval(&[Value::u64(x,"U64")]);
      assert_eq!(Value::u64(321,"U64"), jval, "321");
   }
}

#[test]
fn eval_match2() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::pattern(
            Expression::variable(0,()).typed("U64"),
            vec![
               (
                  LHSPart::literal("3"),
                  Expression::unary(b"123",()),
               ),
               (
                  LHSPart::Any,
                  Expression::unary(b"321",()),
               ),
            ],
         ()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
      let jval = jit.eval(&[Value::u64(x,"U64")]);
      assert_eq!(Value::u64(if x==3 {123} else {321},"U64"), jval, "if {}==3 then 123 else 321", x);
   }
}

#[test]
fn eval_match3() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![(24,Type::nominal("U64"))],
         vec![Expression::pattern(
            Expression::variable(24,()).typed("U64"),
            vec![
               (
                  LHSPart::literal("3"),
                  Expression::unary(b"123",()).typed("U64"),
               ),
               (
                  LHSPart::Any,
                  Expression::unary(b"321",()).typed("U64"),
               ),
            ],
         ()).typed("U64")],
      )],
      vec![
         Expression::apply(0,vec![
            Expression::variable(0, ()),
         ],()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
      let jval = jit.eval(&[Value::u64(x,"U64")]);
      assert_eq!(Value::u64(if x==3 {123} else {321},"U64"), jval, "if {}==3 then 123 else 321", x);
   }
}

#[test]
fn eval_add() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![(0,Type::nominal("U64")), (1,Type::nominal("U64"))],
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
      let jval = jit.eval(&[Value::u64(x,"U64"), Value::u64(y,"U64")]);
      assert_eq!(Value::u64(x + y,"U64"), jval, "{} + {}", x, y);
   }}
}

#[test]
fn eval_fibonacci() {
   let jit = l1_fibonacci();

   for x in 0..20 {
      let jval = jit.eval(&[Value::u64(x,"U64")]);
      assert_eq!(Value::u64(rust_fibonacci(x),"U64"), jval, "fibonacci({})", x);
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
         vec![(24,Type::nominal("U64"))],
         vec![Expression::pattern(
            Expression::variable(24,()).typed("U64"),
            vec![
               (
                  LHSPart::literal("0"),
                  Expression::unary(b"1",()).typed("U64"),
               ),
               (
                  LHSPart::literal("1"),
                  Expression::unary(b"1",()).typed("U64"),
               ),
               (
                  LHSPart::ul(
                     vec![LHSLiteralPart::literal("2")],
                     Some(2),
                     vec![],
                  ),
                  Expression::li(vec![
                     LIPart::expression(
                        Expression::apply(0,vec![
                           Expression::variable(2,()).typed("U64"),
                        ],()),
                     ),
                     LIPart::expression(
                        Expression::apply(0,vec![
                           Expression::li(vec![
                              LIPart::literal("1"),
                              LIPart::variable(2),
                           ],()).typed("U64"),
                        ],()),
                     ),
                  ],()).typed("U64"),
               ),
            ],
         ()).typed("U64")],
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
      let jval = jit.eval(&[Value::u64(x,"U64")]);
      assert_eq!(Value::u64(rust_two_pow_n(x),"U64"), jval, "2^{}", x);
   }
}

fn rust_two_pow_n(n: u64) -> u64 {
    match n {
        0 => 1,
        n => rust_two_pow_n(n-1) + rust_two_pow_n(n-1),
    }
}
fn l1_two_pow_n() -> JProgram {
   let l12n = Program::program(
      vec![
         FunctionDefinition::define( // 0 = $"+"
            vec![(0,Type::nominal("U64")),(1,Type::nominal("U64"))],
            vec![Expression::li(vec![
               LIPart::variable(0),
               LIPart::variable(1),
            ],())]
         ),
         FunctionDefinition::define( // 1 = $"-"
            vec![(0,Type::nominal("U64")),(1,Type::nominal("U64"))],
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
                     ],()).typed("U64"),
                  ),
               ],
            ()).typed("U64")],
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
*/
