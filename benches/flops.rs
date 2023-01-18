//With credit to original IEEE F32 reference implementation: SoftFloat
/*============================================================================

This C source file is part of the SoftFloat IEEE Floating-Point Arithmetic
Package, Release 3e, by John R. Hauser.

Copyright 2011, 2012, 2013, 2014, 2015 The Regents of the University of
California.  All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

 1. Redistributions of source code must retain the above copyright notice,
    this list of conditions, and the following disclaimer.

 2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions, and the following disclaimer in the documentation
    and/or other materials provided with the distribution.

 3. Neither the name of the University nor the names of its contributors may
    be used to endorse or promote products derived from this software without
    specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS "AS IS", AND ANY
EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE, ARE
DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE FOR ANY
DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

=============================================================================*/

use std::time::{Instant};

const defaultNaNF32UI: u32 = 0xFFFFFFFF; //on 8086, 0xFFC00000
fn signF32UI(a: u32) -> bool {
   (a >> 31) == 1
}
fn expF32UI(a: u32) -> i16 {
   ((a >> 23) & 0xFF) as i16
}
fn fracF32UI(a: u32) -> u32 {
   a & 0x007FFFFF
}
fn packToF32UI( sign: bool, exp: i16, sig: u32 ) -> u32 {
unsafe {
   ((sign as u32)<<31) +
   ((std::mem::transmute::<i16,u16>(exp) as u32) << 23) +
   sig
}}
fn isNaNF32UI( a: u32 ) -> bool {
   (((!a) & 0x7F800000) == 0) &&
   ((a & 0x007FFFFF) != 0)
}
fn softfloat_propagateNaNF32UI( uiA: u32, uiB: u32 ) -> u32 {
   return defaultNaNF32UI;
}
fn softfloat_normSubnormalF32Sig( sig: u32 ) -> (i16,u32) {
    let shiftDist = (sig.leading_zeros() - 8) as i16;
    let exp = 1 - shiftDist;
    let sig = sig << shiftDist;
    (exp, sig)
}
fn softfloat_roundPackToF32( sign: bool, mut exp: i16, mut sig: u32 ) -> f32 {
unsafe {
    let roundNearEven = false;
    let roundIncrement: u8 = 0x40; //alternatives: 0x7F, 0
    let mut roundBits: u8;
    let isTiny: bool;

    roundBits = (sig & 0x7F) as u8;
    if 0xFD <= std::mem::transmute::<i16,u16>(exp) {
        if exp < 0 {
            isTiny = (exp < -1) || (sig + (roundIncrement as u32) < 0x80000000);
            sig = softfloat_shiftRightJam32( sig, -exp );
            exp = 0;
            roundBits = (sig & 0x7F) as u8;
        } else if (0xFD < exp) || (0x80000000 <= sig + (roundIncrement as u32)) {
            let uiZ = packToF32UI( sign, 0xFF, 0 ) - ((!roundIncrement) as u32);
            return std::mem::transmute::<u32,f32>(uiZ);
        }
    }
    sig = (sig + (roundIncrement as u32)) >> 7;
    sig &= (!(! (roundBits ^ 0x40) & (roundNearEven as u8))) as u32;
    if sig == 0 { exp = 0 };
    let uiZ = packToF32UI( sign, exp, sig );
    return std::mem::transmute::<u32,f32>(uiZ);
}}
fn softfloat_shortShiftRightJam64( a: u64, dist: u8 ) -> u64
{
    return a >> dist |
           (
              ((a & ((1<<dist as u64) - 1)
              ) != 0) as u64
           );
}
fn softfloat_shiftRightJam32( a: u32, dist: i16 ) -> u32
{
    return if dist < 31 {
       a>>dist | (
          ((a<<(
             -dist
             & 31)) != 0)
       as u32)
    } else { (a != 0) as u32 };
}

fn fmul(a: f32, b: f32) -> f32 { unsafe {
    let uA: i32 = std::mem::transmute::<f32,i32>(a);
    let uiA: u32 = std::mem::transmute::<f32,u32>(a);
    let signA: bool = signF32UI(uiA);
    let mut expA: i16 = expF32UI(uiA);
    let mut sigA: u32 = fracF32UI(uiA);
    let uB: i32 = std::mem::transmute::<f32,i32>(b);
    let uiB: u32 = std::mem::transmute::<f32,u32>(b);
    let signB: bool = signF32UI(uiB);
    let mut expB: i16 = expF32UI(uiB);
    let mut sigB: u32 = fracF32UI(uiB);
    let signZ: bool = signA ^ signB;
    let magBits: u32;
    let mut expZ: i16;
    let mut sigZ: u32;
    let uiZ: u32;

    if expA == 0xFF {
        if (sigA != 0) || ((expB == 0xFF) && (sigB != 0)) {
           uiZ = softfloat_propagateNaNF32UI( uiA, uiB );
        } else {
           magBits = (std::mem::transmute::<i16,u16>(expB) as u32) | sigB;
           if magBits == 0 {
              uiZ = defaultNaNF32UI;
           } else {
              uiZ = packToF32UI( signZ, 0xFF, 0 );
           }
        }
        return std::mem::transmute::<u32,f32>(uiZ);
    }
    if expB == 0xFF {
        if sigB != 0 {
           uiZ = softfloat_propagateNaNF32UI( uiA, uiB );
        } else {
           magBits = (std::mem::transmute::<i16,u16>(expA) as u32) | sigA;
           if magBits == 0 {
              uiZ = defaultNaNF32UI;
           } else {
              uiZ = packToF32UI( signZ, 0xFF, 0 );
           }
        }
        return std::mem::transmute::<u32,f32>(uiZ);
    }

    if expA == 0 {
        if sigA == 0 {
           return std::mem::transmute::<u32,f32>(packToF32UI(signZ, 0, 0));
        };
        (expA, sigA) = softfloat_normSubnormalF32Sig( sigA );
    }
    if expB == 0 {
        if sigB == 0 {
           return std::mem::transmute::<u32,f32>(packToF32UI(signZ, 0, 0));
        }
        (expB, sigB) = softfloat_normSubnormalF32Sig( sigB );
    }
    expZ = expA + expB - 0x7F;
    sigA = (sigA | 0x00800000)<<7;
    sigB = (sigB | 0x00800000)<<8;
    sigZ = softfloat_shortShiftRightJam64( (sigA * sigB) as u64, 32 ) as u32;
    if sigZ < 0x40000000 {
       expZ -= 1;
       sigZ <<= 1;
    }
    return softfloat_roundPackToF32( signZ, expZ, sigZ );
}}

pub fn main() {
    let start = Instant::now();
    for x in 0_u32..=u32::MAX {
       let x_sign = signF32UI(x);
       let x_exp  = expF32UI(x);
       let x_frac = fracF32UI(x);
       let x_pack = packToF32UI(x_sign, x_exp, x_frac);
       assert_eq!(x, x_pack);
    }
    let t = start.elapsed();
    println!("(Rust) verify u32 pack/unpack in {} seconds", t.as_secs_f32());

    let start = Instant::now();
    for x in (0_u32..u32::MAX).step_by(12345) {
    for y in (0_u32..u32::MAX).step_by(12345) {
    unsafe {
       let xf = std::mem::transmute::<u32,f32>(x);
       let yf = std::mem::transmute::<u32,f32>(y);
       let mut msoft = fmul(xf,yf);
       if msoft.is_nan() { msoft = f32::NAN; }
       let msoft_u32 = std::mem::transmute::<f32,u32>(msoft);
       let mut minst = xf * yf;
       if minst.is_nan() { minst = f32::NAN; }
       let minst_u32 = std::mem::transmute::<f32,u32>(minst);
       assert_eq!( msoft_u32, minst_u32, "{}_u32 vs {}_u32 = {}_f32 vs {}_f32 = {} * {} = {} * {}", msoft_u32, minst_u32, msoft, minst, x, y, xf, yf );
    }}}
    let t = start.elapsed();
    println!("(Rust) 2MMMM FLOPS in {} seconds", t.as_secs_f32());
}
