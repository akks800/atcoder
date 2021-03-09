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
        n: usize,
        apx: [(usize, i64, usize);n],
    }

    let ans = apx
        .iter()
        .filter(|&&apx| apx.0 < apx.2)
        .map(|&apx| apx.1)
        .min();
    println!("{}", if let Some(x) = ans { x } else { -1 });
}
