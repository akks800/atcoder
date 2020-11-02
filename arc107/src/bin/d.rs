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

const M: i64 = 998244353;

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // dp[j]: 要素を大きい方からi個選び、i番目の要素が2^-xであり、
    // 残りがj*2^yである場合の数 // x<=y
    let mut dp = vec![0; n + 1];
    dp[k] = 1;

    for i in 0..n {
        for j in 0..n {
            if i + j * 2 <= n {
                let tmp = (dp[i + j * 2] + dp[i + j]) % M;
                dp[i + j * 2] = tmp;
            }
        }
    }

    println!("{}", dp[n]);
}
