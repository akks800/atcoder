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
        sx:f64,
        sy:f64,
        gx:f64,
        gy:f64,
    }
    let x = (gx * sy + sx * gy) / (sy + gy);

    println!("{}", x);
}
