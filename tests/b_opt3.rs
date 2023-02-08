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
               Expression::literal("5", ()),
            ],()),
            TIPart::variable(10)
         ,())
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!("(0,1,2,3,4)", format!("{:?}",jval), "for x in range(5) yield x");
}
