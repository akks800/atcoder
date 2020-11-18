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
        w: i32,
        stp: [(usize,usize,i32);n],
    }

    let mut v = Vec::with_capacity(n * 2);
    for i in 0..n {
        v.push((stp[i].0, stp[i].2));
        v.push((stp[i].1, -stp[i].2));
    }
    v.sort();

    let mut test = 0;
    for i in 0..v.len() {
        test += v[i].1;
        if test > w {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
