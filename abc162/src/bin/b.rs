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

    let mut sum = 0;
    for i in 1..=n {
        if i % 3 != 0 && i % 5 != 0 {
            sum += i;
        }
    }

    println!("{}", sum);
}
