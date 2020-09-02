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
        m: usize,
        a: [usize;n],
    }

    let sum: usize = a.iter().sum();
    let choice = a.iter().filter(|&&x| sum <= x * 4 * m).count();
    println!("{}", if m <= choice { "Yes" } else { "No" });
}
