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
    }

    /*
     n個のノードがあってi個のノードがつながっているときの期待値 fn(i)

     fn(i) = i/n * (1+fn(i)) + (n-i)/n * (1+fn(i+1))
     fn(n) = 0

     n*fn(i) = i + i*fn(i) + (n-i) + (n-i)*fn(i+1)
     (n-i)*fn(i) = n + (n-i)*fn(i+1)
     fn(i) = n/(n-i) + fn(i+1)
    */

    let mut ans: f64 = 0.0;
    for i in 1..n {
        ans += n as f64 / (n - i) as f64;
    }

    println!("{}", ans);
}
