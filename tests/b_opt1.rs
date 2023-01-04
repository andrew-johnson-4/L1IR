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
      let nval = eval(nojit).unwrap();
      let jval = jit.eval().unwrap();
      assert_eq!(nval, jval, "{} + {}", x, y);
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
      let nval = eval(nojit).unwrap();
      let jval = jit.eval().unwrap();
      assert_eq!(nval, jval, "{} - {}", x, y);
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
      let nval = eval(nojit).unwrap();
      let jval = jit.eval().unwrap();
      assert_eq!(nval, jval, "{} == {}", x, y);
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
      let nval = eval(nojit).unwrap();
      let jval = jit.eval().unwrap();
      assert_eq!(nval, jval, "{} != {}", x, y);
   }}
}
