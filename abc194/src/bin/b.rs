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
        ab: [(usize,usize);n],
    }

    let mut ans = 200000;
    for i in 0..n {
        for j in 0..n {
            let atime = ab[i].0;
            let btime = ab[j].1;
            let t = if i == j {
                atime + btime
            } else {
                max(atime, btime)
            };
            ans = min(ans, t);
        }
    }

    println!("{}", ans);
}
