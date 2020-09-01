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
        a: [Usize1;n-1],
    }

    let mut ct = vec![0; n];
    for &x in &a {
        ct[x] += 1;
    }

    for &x in &ct {
        println!("{}", x);
    }
}
