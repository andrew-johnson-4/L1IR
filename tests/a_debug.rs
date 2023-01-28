use std::sync::Arc;
use l1_ir::ast::{Value};

#[test]
fn eval_literals() {
   assert_eq!(
      format!("{:?}",Value::Literal(0,1,Arc::new(vec!['a']),None)),
      r#""a""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,0,Arc::new(vec!['a']),None)),
      r#"0"#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,1,Arc::new(vec!['a']),None)),
      r#"0"#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,3,Arc::new(vec!['a','b','c']),None)),
      r#""abc""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,2,Arc::new(vec!['a','b','c']),None)),
      r#""ab""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,1,Arc::new(vec!['a','b','c']),None)),
      r#""a""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(0,0,Arc::new(vec!['a','b','c']),None)),
      r#"0"#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,3,Arc::new(vec!['a','b','c']),None)),
      r#""bc""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,2,Arc::new(vec!['a','b','c']),None)),
      r#""b""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(1,1,Arc::new(vec!['a','b','c']),None)),
      r#"0"#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(2,3,Arc::new(vec!['a','b','c']),None)),
      r#""c""#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(2,2,Arc::new(vec!['a','b','c']),None)),
      r#"0"#,
   );
   assert_eq!(
      format!("{:?}",Value::Literal(3,3,Arc::new(vec!['a','b','c']),None)),
      r#"0"#,
   );
}

#[test]
fn eval_tuples() {
   let a = Value::Literal(0,1,Arc::new(vec!['a']),None);
   let b = Value::Literal(0,1,Arc::new(vec!['b']),None);
   let cd = Value::Literal(0,2,Arc::new(vec!['c','d']),None);
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
      format!("{:?}",Value::Function("+".to_string(),None)),
      "+",
   );
   assert_eq!(
      format!("{:?}",Value::Function("-".to_string(),None)),
      "-",
   );
   assert_eq!(
      format!("{:?}",Value::Function("==".to_string(),None)),
      "==",
   );
}

#[test]
fn eval_heterogenous() {
   let a = Value::Literal(0,1,Arc::new(vec!['a']),None);
   let a0 = Value::Literal(0,0,Arc::new(vec!['a']),None);
   let bc = Value::Literal(1,3,Arc::new(vec!['a','b','c']),None);
   let f1 = Value::Function("+".to_string(),None);
   let f23 = Value::Function("-".to_string(),None);
   let t1 = Value::tuple(vec![a.clone(),f1.clone(),bc.clone()]);
   let t2 = Value::tuple(vec![a0.clone(),t1.clone(),f23.clone()]);
   assert_eq!(
      format!("{:?}",t1),
      r#"("a",+,"bc")"#
   );
   assert_eq!(
      format!("{:?}",t2),
      r#"(0,("a",+,"bc"),-)"#
   );
}


