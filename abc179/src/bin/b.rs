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
        d: [(usize,usize);n],
    }

    let mut ans = "No";
    for i in 0..n-2 {
        if d[i].0 == d[i].1 && d[i+1].0 == d[i+1].1 && d[i+2].0 == d[i+2].1 {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
