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
        k: usize,
        n: usize,
        a: [usize;n],
    }

    let mut v = Vec::with_capacity(n);
    v.push(k + a[0] - a[n - 1]);
    for i in 1..n {
        v.push(a[i] - a[i - 1]);
    }
    let &m = v.iter().max().unwrap();

    println!("{}", k - m);
}
