use l1_ir::ast::{Expression,Program};
use l1_ir::opt::{JProgram};

#[test]
fn eval_add() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("+:(U64,U64)->U64",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x+y), format!("{:?}",jval), "{} + {}", x, y);
   }}
}

#[test]
fn eval_sub() {
   for x in 0..20 {
   for y in 0..=x {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("-:(U64,U64)->U64",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x-y), format!("{:?}",jval), "{} - {}", x, y);
   }}
}

#[test]
fn eval_eq() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("==:(U64,U64)->U8",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",(x==y) as u64), format!("{:?}",jval), "{} == {}", x, y);
   }}
}

#[test]
fn eval_ne() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("!=:(U64,U64)->U8",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",(x!=y) as u64), format!("{:?}",jval), "{} != {}", x, y);
   }}
}

#[test]
fn eval_lt() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("<:(U64,U64)->U8",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",(x<y) as u64), format!("{:?}",jval), "{} < {}", x, y);
   }}
}

#[test]
fn eval_gt() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply(">:(U64,U64)->U8",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",(x>y) as u64), format!("{:?}",jval), "{} > {}", x, y);
   }}
}

#[test]
fn eval_gte() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply(">=:(U64,U64)->U8",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",(x>=y) as u64), format!("{:?}",jval), "{} >= {}", x, y);
   }}
}

#[test]
fn eval_lte() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("<=:(U64,U64)->U8",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",(x<=y) as u64), format!("{:?}",jval), "{} <= {}", x, y);
   }}
}

#[test]
fn eval_mul() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("*:(U64,U64)->U64",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x*y), format!("{:?}",jval), "{} * {}", x, y);
   }}
}

#[test]
fn eval_rem() {
   for x in 0..20 {
   for y in 1..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("%:(U64,U64)->U64",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x%y), format!("{:?}",jval), "{} % {}", x, y);
   }}
}

#[test]
fn eval_div() {
   for x in 0..20 {
   for y in 1..20 {
      let nojit = Program::program(
         vec![],
         vec![
            Expression::apply("/:(U64,U64)->U64",vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let jval = jit.eval(&[]);
      assert_eq!(format!("{}",x/y), format!("{:?}",jval), "{} / {}", x, y);
   }}
}
