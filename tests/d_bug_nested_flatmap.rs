use l1_ir::ast::{Expression,Program,LHSPart,TIPart};
use l1_ir::opt::{JProgram};

#[test]
fn bug_nested_flatmap() {
   let nojit = Program::program(
      vec![],
      vec![
            Expression::map(
               LHSPart::variable(130),
               Expression::apply("range:(U64)->U64[]",vec![
                  Expression::literal("5", ()).typed("U64"),
               ],()).typed("Value"),
               TIPart::expression(
                  Expression::map(
                     LHSPart::variable(137),
                     Expression::apply("range:(U64)->U64[]",vec![
                        Expression::literal("5", ()).typed("U64"),
                     ],()).typed("Value"),
                     TIPart::expression(
                        Expression::literal("11", ()).typed("Value"),
                     )
                  ,()).typed("Value")
               )
            ,()).typed("Value")
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("(1,1,2,1,2,3)", format!("{:?}",jval), "for x in range(5) yield x");
}
