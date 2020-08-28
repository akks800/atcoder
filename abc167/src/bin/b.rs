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
        a: i64,
        b: i64,
        _c: i64,
        k: i64,
    }
    let ans;
    if k <= a {
        ans = k;
    } else if k <= a+b {
        ans = a;
    } else {
        ans = a - (k-(a+b));
    }

    println!("{}", ans);
}

