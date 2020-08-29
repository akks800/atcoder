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
        x: usize,
    }

    const MAX_X: usize = 200;

    let mut p = vec![0; MAX_X+1];
    for i in 0..=MAX_X {
        p[i] = i*i*i*i*i;
    }

    for i in 0..=MAX_X {
        for j in 0..=i {
            if p[i] - p[j] == x {
                println!("{} {}", i, j);
                return;
            }
            if p[i] + p[j] == x {
                println!("{} -{}", i, j);
                return;
            }
        }
    }
}
