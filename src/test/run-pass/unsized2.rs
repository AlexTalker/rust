// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// ignore-lexer-test FIXME #15879

// Test sized-ness checking in substitution.

// Unbounded.
fn f1<X: ?Sized>(x: &X) {
    f1::<X>(x);
}
fn f2<X>(x: &X) {
    f1::<X>(x);
    f2::<X>(x);
}

// Bounded.
trait T {}
fn f3<X: T+?Sized>(x: &X) {
    f3::<X>(x);
}
fn f4<X: T>(x: &X) {
    f3::<X>(x);
    f4::<X>(x);
}

// Self type.
trait T2 {
    fn f() -> Box<Self>;
}
struct S;
impl T2 for S {
    fn f() -> Box<S> {
        box S
    }
}
fn f5<X: ?Sized+T2>(x: &X) {
    let _: Box<X> = T2::f();
}
fn f6<X: T2>(x: &X) {
    let _: Box<X> = T2::f();
}

trait T3 {
    fn f() -> Box<Self>;
}
impl T3 for S {
    fn f() -> Box<S> {
        box S
    }
}
fn f7<X: ?Sized+T3>(x: &X) {
    // This is valid, but the unsized bound on X is irrelevant because any type
    // which implements T3 must have statically known size.
    let _: Box<X> = T3::f();
}

trait T4<X> {
    fn m1(x: &T4<X>);
    fn m2(x: &T5<X>);
}
trait T5<X: ?Sized> {
    // not an error (for now)
    fn m1(x: &T4<X>);
    fn m2(x: &T5<X>);
}

trait T6<X: T> {
    fn m1(x: &T4<X>);
    fn m2(x: &T5<X>);
}
trait T7<X: ?Sized+T> {
    // not an error (for now)
    fn m1(x: &T4<X>);
    fn m2(x: &T5<X>);
}

// The last field in a struct or variant may be unsized
struct S2<X: ?Sized> {
    f: X,
}
struct S3<X: ?Sized> {
    f1: int,
    f2: X,
}
enum E<X: ?Sized> {
    V1(X),
    V2{x: X},
    V3(int, X),
    V4{u: int, x: X},
}

pub fn main() {
}
