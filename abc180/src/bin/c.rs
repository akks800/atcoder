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
        n: u64,
    }
    let mut v = Vec::new();

    for i in 1..=1000_000 {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            v.push(i);
            if n / i > i {
                v.push(n / i);
            }
        }
    }
    v.sort_unstable();
    for i in v {
        println!("{}", i);
    }
}
