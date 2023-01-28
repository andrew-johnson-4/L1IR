use std::rc::Rc;
use l1_ir::ast::{Value};

#[test]
fn eq_unary() {
   assert_eq!(
      Value::unary(b"0"),
      Value::unary(b"0"),
   );
   assert_eq!(
      Value::unary(b"1"),
      Value::unary(b"1"),
   );
   assert_eq!(
      Value::unary(b"123456789"),
      Value::unary(b"123456789"),
   );
   assert_eq!(
      Value::unary(b"123456789101112131415161718192021"),
      Value::unary(b"123456789101112131415161718192021"),
   );
   assert_eq!(
      Value::unary(b"0"),
      Value::Literal(0,0,Rc::new(vec![]),None),
   );
   assert_eq!(
      Value::Literal(0,0,Rc::new(vec![]),None),
      Value::unary(b"0"),
   );
   assert_eq!(
      Value::unary(b"1"),
      Value::Literal(0,1,Rc::new(vec!['0']),None),
   );
   assert_eq!(
      Value::Literal(0,1,Rc::new(vec!['0']),None),
      Value::unary(b"1"),
   );
}

#[test]
fn eq_literals() {
   assert_eq!(
      Value::Literal(0,1,Rc::new(vec!['a']),None),
      Value::Literal(0,1,Rc::new(vec!['a']),None),
   );
   assert_eq!(
      Value::Literal(0,0,Rc::new(vec!['a']),None),
      Value::Literal(0,0,Rc::new(vec!['a']),None),
   );
   assert_eq!(
      Value::Literal(1,1,Rc::new(vec!['a']),None),
      Value::Literal(1,1,Rc::new(vec!['a']),None),
   );
   assert_eq!(
      Value::Literal(0,3,Rc::new(vec!['a','b','c']),None),
      Value::Literal(0,3,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(0,2,Rc::new(vec!['a','b','c']),None),
      Value::Literal(0,2,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(0,1,Rc::new(vec!['a','b','c']),None),
      Value::Literal(0,1,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(0,0,Rc::new(vec!['a','b','c']),None),
      Value::Literal(0,0,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(1,3,Rc::new(vec!['a','b','c']),None),
      Value::Literal(1,3,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(1,2,Rc::new(vec!['a','b','c']),None),
      Value::Literal(1,2,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(1,1,Rc::new(vec!['a','b','c']),None),
      Value::Literal(1,1,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(2,3,Rc::new(vec!['a','b','c']),None),
      Value::Literal(2,3,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(2,2,Rc::new(vec!['a','b','c']),None),
      Value::Literal(2,2,Rc::new(vec!['a','b','c']),None),
   );
   assert_eq!(
      Value::Literal(3,3,Rc::new(vec!['a','b','c']),None),
      Value::Literal(3,3,Rc::new(vec!['a','b','c']),None),
   );
}

#[test]
fn eq_tuples() {
   let a = Value::Literal(0,1,Rc::new(vec!['a']),None);
   let b = Value::Literal(0,1,Rc::new(vec!['b']),None);
   let cd = Value::Literal(0,2,Rc::new(vec!['c','d']),None);
   assert_eq!(
      Value::tuple(vec![]),
      Value::tuple(vec![]),
   );
   assert_eq!(
      Value::tuple(vec![a.clone()]),
      Value::tuple(vec![a.clone()]),
   );
   assert_eq!(
      Value::tuple(vec![b.clone()]),
      Value::tuple(vec![b.clone()]),
   );
   assert_eq!(
      Value::tuple(vec![cd.clone()]),
      Value::tuple(vec![cd.clone()]),
   );
   assert_eq!(
      Value::tuple(vec![a.clone(),b.clone()]),
      Value::tuple(vec![a.clone(),b.clone()]),
   );
   assert_eq!(
      Value::tuple(vec![a.clone(),cd.clone()]),
      Value::tuple(vec![a.clone(),cd.clone()]),
   );
   assert_eq!(
      Value::tuple(vec![b.clone(),cd.clone()]),
      Value::tuple(vec![b.clone(),cd.clone()]),
   );
   assert_eq!(
      Value::tuple(vec![a.clone(),b.clone(),cd.clone()]),
      Value::tuple(vec![a.clone(),b.clone(),cd.clone()]),
   );
}

#[test]
fn eq_functions() {
   assert_eq!(
      Value::Function("+".to_string(),None),
      Value::Function("+".to_string(),None),
   );
   assert_eq!(
      Value::Function("-".to_string(),None),
      Value::Function("-".to_string(),None),
   );
   assert_eq!(
      Value::Function("==".to_string(),None),
      Value::Function("==".to_string(),None),
   );
}

#[test]
fn eq_heterogenous() {
   let a = Value::Literal(0,1,Rc::new(vec!['a']),None);
   let a0 = Value::Literal(0,0,Rc::new(vec!['a']),None);
   let bc = Value::Literal(1,3,Rc::new(vec!['a','b','c']),None);
   let f1 = Value::Function("+".to_string(),None);
   let f23 = Value::Function("-".to_string(),None);
   let t1 = Value::tuple(vec![a.clone(),f1.clone(),bc.clone()]);
   let t2 = Value::tuple(vec![a0.clone(),t1.clone(),f23.clone()]);
   assert_eq!(
      t1,
      t1,
   );
   assert_eq!(
      t2,
      t2,
   );
}


