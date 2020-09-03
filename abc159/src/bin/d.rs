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
        a:[usize;n],
    }

    let mut h = vec![0i64; n + 1];
    for &val in &a {
        h[val] += 1;
    }

    fn f(x: i64) -> i64 {
        x * (x - 1) / 2
    };

    let ans: i64 = h.iter().map(|&x| f(x)).sum();
    for i in 0..n {
        let ct = h[a[i]];
        println!("{}", ans - f(ct) + f(ct - 1));
    }
}
