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
        a: [i128;n],
    }

    let sum_a: i128 = a.iter().sum();
    let sum_a2: i128 = a.iter().map(|&x| x * x).sum();

    println!("{}", n as i128 * sum_a2 - sum_a * sum_a);
}
