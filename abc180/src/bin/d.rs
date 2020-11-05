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
        mut x: u128,
        y: u128,
        a: u128,
        b: u128,
    }

    let mut ct = 0;
    while x < y {
        if x * a < x + b {
            x *= a;
            ct += 1;
        } else {
            let d = max(1, (y - x) / b);
            x += d * b;
            ct += d;
        }
    }

    println!("{}", ct - 1);
}
