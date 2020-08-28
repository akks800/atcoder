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
        k: u64,
        a:[Usize1;n],
    }

    const MAX_DBL:usize = 64;
    let mut dbl = vec![vec![0;n];MAX_DBL];

    for i in 0..n {
        dbl[0][i] = a[i];
    }
    for j in 1..MAX_DBL {
        for i in 0..n {
            dbl[j][i] = dbl[j-1][dbl[j-1][i]];
        }
    }

    let mut pos = 0;
    for i in 0..MAX_DBL {
        if k & (1u64<<i) != 0 {
            pos = dbl[i][pos];
        }
    }
    println!("{}", pos+1);
}
