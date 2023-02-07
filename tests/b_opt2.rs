use l1_ir::ast::{Expression,Program};
use l1_ir::opt::{JProgram};

#[test]
fn eval_range1() {
   for x in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("range:(U64)->U64[]",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x), format!("{:?}",jval), "range({})", x);
   }
}

#[test]
fn eval_range2() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("range:(U64,U64)->U64[]",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x), format!("{:?}",jval), "range({},{})", x, y);
   }}
}

#[test]
fn eval_range3() {
   for x in 0..20 {
   for y in 0..20 {
   for z in 1..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("range:(U64,U64,U64)->U64[]",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
               Expression::unary(format!("{}",z).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x), format!("{:?}",jval), "range({},{},{})", x, y, z);
   }}}
}

/* TODO implement tuples
use l1_ir::value;
use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,TIPart,Value,Type};
use l1_ir::eval::{eval};
use l1_ir::opt::{JProgram};

#[test]
fn eval_tuple1() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![(0,Type::nominal("U64")), (1,Type::nominal("U64"))],
         vec![Expression::pattern(
            Expression::ti(vec![
               TIPart::variable(0),
               TIPart::expression(Expression::variable(1,())),
            ],()),
            vec![(
               LHSPart::Any,
               Expression::unary(b"7",())
            )],
         ())],
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
      let nval = eval(nojit.clone(),&[Value::from_u64(x),Value::from_u64(y)]).unwrap();
      let jx = value::Value::u64(x,"U64");
      let jy = value::Value::u64(y,"U64");
      let jval = jit.eval(&[jx,jy]);
      assert_eq!(nval, jval.ast(), "match ({},{})", x, y);
   }}
}

#[test]
fn eval_tuple2() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![(0,Type::nominal("U64")), (1,Type::nominal("U64"))],
         vec![Expression::pattern(
            Expression::ti(vec![
               TIPart::variable(0),
               TIPart::expression(Expression::variable(1,())),
            ],()),
            vec![(
               LHSPart::tuple(vec![
                  LHSPart::literal("0"),
                  LHSPart::Any,
               ]),
               Expression::unary(b"1",())
            ),(
               LHSPart::tuple(vec![
                  LHSPart::Any,
                  LHSPart::literal("00"),
               ]),
               Expression::unary(b"2",())
            ),(
               LHSPart::tuple(vec![
                  LHSPart::literal("000"),
                  LHSPart::literal("0000"),
               ]),
               Expression::unary(b"3",())
            ),(
               LHSPart::Any,
               Expression::unary(b"4",())
            )],
         ())],
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
      let nval = eval(nojit.clone(),&[Value::from_u64(x),Value::from_u64(y)]).unwrap();
      let jx = value::Value::u64(x,"U64");
      let jy = value::Value::u64(y,"U64");
      let jval = jit.eval(&[jx,jy]);
      assert_eq!(nval, jval.ast(), "match ({},{})", x, y);
   }}
}
*/
