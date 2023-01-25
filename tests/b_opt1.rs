use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,LHSPart,LHSLiteralPart};
use l1_ir::eval::{eval};
use l1_ir::opt::{JProgram};

#[test]
fn eval_add() {
   for x in 0..20 {
   for y in 0..20 {
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
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} + {}", x, y);
   }}
}

#[test]
fn eval_sub() {
   for x in 0..20 {
   for y in 0..=x {
      let nojit = Program::program(
         vec![FunctionDefinition::define(
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
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} - {}", x, y);
   }}
}

#[test]
fn eval_eq() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
          vec![FunctionDefinition::define(
             vec![0,1],
             vec![Expression::pattern(
                Expression::variable(0,()),
                vec![(
                   LHSPart::ul(
                      vec![LHSLiteralPart::variable(1)],
                      None,
                      vec![],
                   ),
                   Expression::unary(b"1",()),
                ),(
                   LHSPart::Any,
                   Expression::unary(b"0",()),
                )],
             ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} == {}", x, y);
   }}
}

#[test]
fn eval_ne() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
          vec![FunctionDefinition::define(
             vec![0,1],
             vec![Expression::pattern(
                Expression::variable(0,()),
                vec![(
                   LHSPart::ul(
                      vec![LHSLiteralPart::variable(1)],
                      None,
                      vec![],
                   ),
                   Expression::unary(b"0",()),
                ),(
                   LHSPart::Any,
                   Expression::unary(b"1",()),
                )],
             ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} != {}", x, y);
   }}
}

#[test]
fn eval_lt() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
          vec![FunctionDefinition::define(
             vec![0,1],
             vec![Expression::pattern(
                Expression::variable(1,()),
                vec![(
                   LHSPart::ul(
                      vec![LHSLiteralPart::variable(0),
                           LHSLiteralPart::literal("0")],
                      Some(2),
                      vec![],
                   ),
                   Expression::unary(b"1",()),
                ),(
                   LHSPart::Any,
                   Expression::unary(b"0",()),
                )],
             ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} < {}", x, y);
   }}
}

#[test]
fn eval_gt() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
          vec![FunctionDefinition::define(
             vec![0,1],
             vec![Expression::pattern(
                Expression::variable(0,()),
                vec![(
                   LHSPart::ul(
                      vec![LHSLiteralPart::variable(1),
                           LHSLiteralPart::literal("0")],
                      Some(2),
                      vec![],
                   ),
                   Expression::unary(b"1",()),
                ),(
                   LHSPart::Any,
                   Expression::unary(b"0",()),
                )],
             ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} > {}", x, y);
   }}
}

#[test]
fn eval_gte() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
          vec![FunctionDefinition::define(
             vec![0,1],
             vec![Expression::pattern(
                Expression::variable(1,()),
                vec![(
                   LHSPart::ul(
                      vec![LHSLiteralPart::variable(0),
                           LHSLiteralPart::literal("0")],
                      Some(2),
                      vec![],
                   ),
                   Expression::unary(b"0",()),
                ),(
                   LHSPart::Any,
                   Expression::unary(b"1",()),
                )],
             ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} >= {}", x, y);
   }}
}

#[test]
fn eval_lte() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
          vec![FunctionDefinition::define(
             vec![0,1],
             vec![Expression::pattern(
                Expression::variable(0,()),
                vec![(
                   LHSPart::ul(
                      vec![LHSLiteralPart::variable(1),
                           LHSLiteralPart::literal("0")],
                      Some(2),
                      vec![],
                   ),
                   Expression::unary(b"0",()),
                ),(
                   LHSPart::Any,
                   Expression::unary(b"1",()),
                )],
             ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} <= {}", x, y);
   }}
}

#[test]
fn eval_mul() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![FunctionDefinition::define(
            vec![0,1],
            vec![Expression::pattern(
               Expression::variable(0,()),
               vec![(
                  LHSPart::ul(
                     vec![LHSLiteralPart::literal("0")],
                     Some(2),
                     vec![],
                  ),
                  Expression::li(vec![
                     LIPart::variable(1),
                     LIPart::expression(Expression::apply(0,vec![
                        Expression::variable(2,()),
                        Expression::variable(1,()),
                     ],())),
                  ],())
               ),(
                  LHSPart::Any,
                  Expression::unary(b"0",())
               )],
            ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} * {}", x, y);
   }}
}

#[test]
fn eval_rem() {
   for x in 0..20 {
   for y in 1..20 {
      let nojit = Program::program(
         vec![
            FunctionDefinition::define(
               vec![0,1],
               vec![Expression::pattern(
                  Expression::variable(0,()),
                  vec![(
                     LHSPart::ul(
                        vec![LHSLiteralPart::variable(1)],
                        Some(2),
                        vec![],
                     ),
                     Expression::apply(0,vec![
                        Expression::variable(2,()),
                        Expression::variable(1,()),
                     ],())
                  ),(
                     LHSPart::Any,
                     Expression::variable(0,()),
                  )],
               ())],
            )

         ],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} % {}", x, y);
   }}
}

#[test]
fn eval_div() {
   for x in 0..20 {
   for y in 1..20 {
      let nojit = Program::program(
         vec![
            FunctionDefinition::define(
               vec![0,1],
               vec![Expression::pattern(
                  Expression::variable(0,()),
                  vec![(
                     LHSPart::ul(
                        vec![LHSLiteralPart::variable(1)],
                        Some(2),
                        vec![],
                     ),
                     Expression::li(vec![
                        LIPart::literal("0"),
                        LIPart::expression(Expression::apply(0,vec![
                           Expression::variable(2,()),
                           Expression::variable(1,()),
                        ],())),
                     ],())
                  ),(
                     LHSPart::Any,
                     Expression::unary(b"0",())
                  )],
               ())],
            )
         ],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit,&[]).unwrap();
      let jval = jit.eval(&[]);
      assert_eq!(nval, jval.ast(), "{} % {}", x, y);
   }}
}
