use l1_ir::ast::{Expression,Program,LHSPart,TIPart};
use l1_ir::opt::{JProgram};

#[test]
fn eval_flatmap1() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::map(
            LHSPart::variable(10),
            Expression::apply("range:(U64)->U64[]",vec![
               Expression::literal("5", ()).typed("U64"),
            ],()).typed("Value"),
            TIPart::variable(10)
         ,()).typed("Value")
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("(0,1,2,3,4)", format!("{:?}",jval), "for x in range(5) yield x");
}

#[test]
fn eval_flatmap2() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::map(
            LHSPart::variable(10),
            Expression::apply("range:(U64)->U64[]",vec![
               Expression::literal("5", ()).typed("U64"),
            ],()).typed("Value"),
            TIPart::expression(Expression::variable(10,()))
         ,()).typed("Value")
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("(0,1,2,3,4)", format!("{:?}",jval), "for x in range(5) yield x");
}

#[test]
fn eval_flatmap3() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::map(
            LHSPart::variable(10),
            Expression::apply("range:(U64)->U64[]",vec![
               Expression::literal("5", ()).typed("U64"),
            ],()).typed("Value"),
            TIPart::expression(Expression::map(
               LHSPart::variable(11),
               Expression::apply("range:(U64)->U64[]",vec![
                  Expression::variable(10,())
               ],()).typed("Value"),
               TIPart::variable(11)
            ,()).typed("Value"))
         ,()).typed("Value")
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("((),(0),(0,1),(0,1,2),(0,1,2,3))", format!("{:?}",jval), "for x in range(5) yield x");
}

#[test]
fn eval_flatmap4() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::map(
            LHSPart::variable(10),
            Expression::apply("range:(U64)->U64[]",vec![
               Expression::literal("5", ()).typed("U64"),
            ],()).typed("Value"),
            TIPart::expression(Expression::pattern(
               Expression::apply("==:(U64,U64)->U8",vec![
                  Expression::variable(10,()).typed("U64"),
                  Expression::literal("2", ()).typed("U64"),
               ],()).typed("U8"),
               vec![
                  (
                     LHSPart::literal("1"),
                     Expression::variable(10,()),
                  ),
               ],
            ()).typed("Value"))
         ,()).typed("Value")
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("(2)", format!("{:?}",jval), "for x in range(5) yield x");
}
