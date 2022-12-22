use std::rc::Rc;
use l1_ir::ast::{Value};

#[test]
fn eq_literals() {
   assert_eq!(
      Value::Literal(0,1,Rc::new(vec!['a'])),
      Value::Literal(0,1,Rc::new(vec!['a'])),
   );
   assert_eq!(
      Value::Literal(0,0,Rc::new(vec!['a'])),
      Value::Literal(0,0,Rc::new(vec!['a'])),
   );
   assert_eq!(
      Value::Literal(1,1,Rc::new(vec!['a'])),
      Value::Literal(1,1,Rc::new(vec!['a'])),
   );
   assert_eq!(
      Value::Literal(0,3,Rc::new(vec!['a','b','c'])),
      Value::Literal(0,3,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(0,2,Rc::new(vec!['a','b','c'])),
      Value::Literal(0,2,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(0,1,Rc::new(vec!['a','b','c'])),
      Value::Literal(0,1,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(0,0,Rc::new(vec!['a','b','c'])),
      Value::Literal(0,0,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(1,3,Rc::new(vec!['a','b','c'])),
      Value::Literal(1,3,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(1,2,Rc::new(vec!['a','b','c'])),
      Value::Literal(1,2,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(1,1,Rc::new(vec!['a','b','c'])),
      Value::Literal(1,1,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(2,3,Rc::new(vec!['a','b','c'])),
      Value::Literal(2,3,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(2,2,Rc::new(vec!['a','b','c'])),
      Value::Literal(2,2,Rc::new(vec!['a','b','c'])),
   );
   assert_eq!(
      Value::Literal(3,3,Rc::new(vec!['a','b','c'])),
      Value::Literal(3,3,Rc::new(vec!['a','b','c'])),
   );
}

#[test]
fn eq_tuples() {
   let a = Value::Literal(0,1,Rc::new(vec!['a']));
   let b = Value::Literal(0,1,Rc::new(vec!['b']));
   let cd = Value::Literal(0,2,Rc::new(vec!['c','d']));
   assert_eq!(
      Value::Tuple(vec![]),
      Value::Tuple(vec![]),
   );
   assert_eq!(
      Value::Tuple(vec![a.clone()]),
      Value::Tuple(vec![a.clone()]),
   );
   assert_eq!(
      Value::Tuple(vec![b.clone()]),
      Value::Tuple(vec![b.clone()]),
   );
   assert_eq!(
      Value::Tuple(vec![cd.clone()]),
      Value::Tuple(vec![cd.clone()]),
   );
   assert_eq!(
      Value::Tuple(vec![a.clone(),b.clone()]),
      Value::Tuple(vec![a.clone(),b.clone()]),
   );
   assert_eq!(
      Value::Tuple(vec![a.clone(),cd.clone()]),
      Value::Tuple(vec![a.clone(),cd.clone()]),
   );
   assert_eq!(
      Value::Tuple(vec![b.clone(),cd.clone()]),
      Value::Tuple(vec![b.clone(),cd.clone()]),
   );
   assert_eq!(
      Value::Tuple(vec![a.clone(),b.clone(),cd.clone()]),
      Value::Tuple(vec![a.clone(),b.clone(),cd.clone()]),
   );
}

#[test]
fn eq_functions() {
   assert_eq!(
      Value::Function(0),
      Value::Function(0),
   );
   assert_eq!(
      Value::Function(23),
      Value::Function(23),
   );
   assert_eq!(
      Value::Function(456),
      Value::Function(456),
   );
}

#[test]
fn eq_heterogenous() {
   let a = Value::Literal(0,1,Rc::new(vec!['a']));
   let a0 = Value::Literal(0,0,Rc::new(vec!['a']));
   let bc = Value::Literal(1,3,Rc::new(vec!['a','b','c']));
   let f1 = Value::Function(1);
   let f23 = Value::Function(23);
   let t1 = Value::Tuple(vec![a.clone(),f1.clone(),bc.clone()]);
   let t2 = Value::Tuple(vec![a0.clone(),t1.clone(),f23.clone()]);
   assert_eq!(
      t1,
      t1,
   );
   assert_eq!(
      t2,
      t2,
   );
}


