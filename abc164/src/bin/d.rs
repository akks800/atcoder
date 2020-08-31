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
        s: Bytes,
    }

    const MOD: usize = 2019;
    let len = s.len();
    let mut ct = vec![0; MOD];
    let mut rem = 0usize;
    let mut mul = 1;
    ct[0] += 1;
    for i in 0..len {
        rem = (rem + (s[len - 1 - i] - b'0') as usize * mul) % MOD;
        ct[rem] += 1;
        mul = mul * 10 % MOD;
        //println!("{} {} ", len-i-1, rem);
    }

    let mut ans = 0i64;
    for i in 0..MOD {
        let c = ct[i] as i64;
        ans += c * (c - 1) / 2;
    }

    println!("{}", ans);
}
