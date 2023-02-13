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
                  Expression::literal("25", ()).typed("U64"),
               ],()).typed("Value"),
               TIPart::expression(
                  Expression::variable(130, ()).typed("Value"),
               )
            ,()).typed("Value")
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24)", format!("{:?}",jval), "for x in range(5) yield x");
}
