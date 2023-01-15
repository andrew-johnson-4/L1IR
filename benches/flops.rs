use std::time::{Instant};

fn fmul(a: f32, b: f32) -> f32 { unsafe {
   a * b
}}

pub fn main() {
    let start = Instant::now();
    for x in 0_u32..1000000_u32 {
    for y in 0_u32..1000000_u32 {
    unsafe {
       let xf = std::mem::transmute::<u32,f32>(x);
       let yf = std::mem::transmute::<u32,f32>(y);
       assert_eq!( fmul(xf,yf), xf * yf );
    }}}
    let t = start.elapsed();
    println!("(Rust) 2MMMM FLOPS in {} seconds", t.as_secs_f32());
}
