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
        v: i64,
        t: i64,
        s: i64,
        d: i64,
    }

    let hidden = v * t <= d && d <= v * s;
    println!("{}", if hidden { "No" } else { "Yes" });
}
