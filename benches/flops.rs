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
   ((if sign {1_u32} else {0_u32})<<31) +
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
fn fmul(a: f32, b: f32) -> f32 { unsafe {
    let uA: i32 = std::mem::transmute::<f32,i32>(a);
    let uiA: u32 = std::mem::transmute::<f32,u32>(a);
    let signA: bool = signF32UI(uiA);
    let expA: i16 = expF32UI(uiA);
    let sigA: u32 = fracF32UI(uiA);
    let uB: i32 = std::mem::transmute::<f32,i32>(b);
    let uiB: u32 = std::mem::transmute::<f32,u32>(b);
    let signB: bool = signF32UI(uiB);
    let expB: i16 = expF32UI(uiB);
    let sigB: u32 = fracF32UI(uiB);
    let signZ: bool = signA ^ signB;
    let magBits: u32;
    //struct exp16_sig32 normExpSig;
    let expZ: i16;
    let sigZ: u32;
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

    /*------------------------------------------------------------------------
    *------------------------------------------------------------------------*/
    /*
    if ( ! expA ) {
        if ( ! sigA ) goto zero;
        normExpSig = softfloat_normSubnormalF32Sig( sigA );
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    if ( ! expB ) {
        if ( ! sigB ) goto zero;
        normExpSig = softfloat_normSubnormalF32Sig( sigB );
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    */
    /*------------------------------------------------------------------------
    *------------------------------------------------------------------------*/
    /*
    expZ = expA + expB - 0x7F;
    sigA = (sigA | 0x00800000)<<7;
    sigB = (sigB | 0x00800000)<<8;
    sigZ = softfloat_shortShiftRightJam64( (uint_fast64_t) sigA * sigB, 32 );
    if ( sigZ < 0x40000000 ) {
        --expZ;
        sigZ <<= 1;
    }
    return softfloat_roundPackToF32( signZ, expZ, sigZ );
    */
    /*
 propagateNaN:
    uiZ = softfloat_propagateNaNF32UI( uiA, uiB );
    goto uiZ;
 zero:
    uiZ = packToF32UI( signZ, 0, 0 );
 uiZ:
    uZ.ui = uiZ;
    return uZ.f;
    */
    return a * b;
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
