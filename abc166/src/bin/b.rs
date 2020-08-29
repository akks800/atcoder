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
        k: usize,
        da: [[Usize1];k],
    }
    let mut v = vec![1;n];
    for i in 0..k {
        for j in &da[i] {
            v[*j] = 0;
        }
    }

    let ans:usize = v.iter().sum();
    println!("{}", ans);
}

