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
        mut x: u64,
    }

    let n500 = x / 500;
    x -= n500 * 500;
    let n5 = x / 5;

    println!("{}", n500 * 1000 + n5 * 5);
}
