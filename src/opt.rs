use std::fmt::Debug;
use crate::ast::{Program};

pub fn jsweep<S: Clone + Debug>(p: Program<S>) -> Program<S> {
   p
}
