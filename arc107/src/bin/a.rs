use proconio::input;

#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::ops::*;

#[derive(Clone, Debug, Default)]
struct Struct;

//#[proconio::fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }

    const M: u64 = 998244353;
    let f = |x: u64| x * (x + 1) / 2 % M;

    println!("{}", f(a) * f(b) % M * f(c) % M);
}
