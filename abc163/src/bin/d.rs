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
        n:u64,
        k:u64,
    }

    const MOD: u64 = 1000_000_007;
    let mut ans = 0u64;
    for i in k..=n + 1 {
        let min = i * (i - 1) / 2; // [0,k) の和
        let max = n * i - min;
        ans = (ans + (max - min + 1)) % MOD;
    }
    println!("{}", ans);
}
