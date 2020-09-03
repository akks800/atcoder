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
        x: usize,
        y: usize,
    }

    fn distance(p: usize, q: usize) -> usize {
        ((p as isize) - (q as isize)).abs() as usize
    }

    let mut d = vec![0; n];
    for i in 1..n {
        for j in i + 1..=n {
            let d0 = j - i; // XYを通らない場合
            let d1 = distance(i, x) + distance(y, j) + 1; // XYを通る場合
            d[min(d0, d1)] += 1;
        }
    }
    for i in 1..n {
        println!("{}", d[i]);
    }
}
