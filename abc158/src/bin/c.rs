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
        a: usize,
        b: usize,
    }

    for i in 1..=1000 {
        if i * 8 / 100 == a && i * 10 / 100 == b {
            println!("{}", i);
            return;
        }
    }

    println!("{}", -1);
}
