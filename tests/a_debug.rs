use std::rc::Rc;
use l1_ir::ast::{Value};

#[test]
fn eval_literals() {
   assert_eq!(
      format!("{:?}",Value::Literal(0,1,Rc::new(vec!['a']))),
      r#""a""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,0,Rc::new(vec!['a']))),
      r#""""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,1,Rc::new(vec!['a']))),
      r#""""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,3,Rc::new(vec!['a','b','c']))),
      r#""abc""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,2,Rc::new(vec!['a','b','c']))),
      r#""ab""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,1,Rc::new(vec!['a','b','c']))),
      r#""a""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,0,Rc::new(vec!['a','b','c']))),
      r#""""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,3,Rc::new(vec!['a','b','c']))),
      r#""bc""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,2,Rc::new(vec!['a','b','c']))),
      r#""b""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,1,Rc::new(vec!['a','b','c']))),
      r#""""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(2,3,Rc::new(vec!['a','b','c']))),
      r#""c""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(2,2,Rc::new(vec!['a','b','c']))),
      r#""""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(3,3,Rc::new(vec!['a','b','c']))),
      r#""""#,
   );
}

#[test]
fn eval_tuples() {
   let a = Value::Literal(0,1,Rc::new(vec!['a']));
   let b = Value::Literal(0,1,Rc::new(vec!['b']));
   let cd = Value::Literal(0,2,Rc::new(vec!['c','d']));
   assert_eq!(
      format!("{:?}",Value::tuple(vec![])),
      r#"()"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![a.clone()])),
      r#"("a",)"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![b.clone()])),
      r#"("b",)"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![cd.clone()])),
      r#"("cd",)"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![a.clone(),b.clone()])),
      r#"("a","b")"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![a.clone(),cd.clone()])),
      r#"("a","cd")"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![b.clone(),cd.clone()])),
      r#"("b","cd")"#,
   );
   assert_eq!(
      format!("{:?}",Value::tuple(vec![a.clone(),b.clone(),cd.clone()])),
      r#"("a","b","cd")"#,
   );
}

#[test]
fn eval_functions() {
   assert_eq!(
      format!("{:?}",Value::Function(0)),
      "f#0",
   );
   assert_eq!(
      format!("{:?}",Value::Function(23)),
      "f#23",
   );
   assert_eq!(
      format!("{:?}",Value::Function(456)),
      "f#456",
   );
}

#[test]
fn eval_heterogenous() {
   let a = Value::Literal(0,1,Rc::new(vec!['a']));
   let a0 = Value::Literal(0,0,Rc::new(vec!['a']));
   let bc = Value::Literal(1,3,Rc::new(vec!['a','b','c']));
   let f1 = Value::Function(1);
   let f23 = Value::Function(23);
   let t1 = Value::tuple(vec![a.clone(),f1.clone(),bc.clone()]);
   let t2 = Value::tuple(vec![a0.clone(),t1.clone(),f23.clone()]);
   assert_eq!(
      format!("{:?}",t1),
      r#"("a",f#1,"bc")"#
   );
   assert_eq!(
      format!("{:?}",t2),
      r#"("",("a",f#1,"bc"),f#23)"#
   );
}


