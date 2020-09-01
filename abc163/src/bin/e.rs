use proconio::input;

#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::ops::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Struct;

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a:[i64;n],
    }

    let mut v = Vec::new();
    for i in 0..n {
        v.push((i, a[i]));
    }

    v.sort_unstable_by_key(|p| -p.1);

    // 大きい方から貪欲だと、左右どちらも同じ距離の場合に困る??

    let mut dp = vec![vec![0i64; n + 1]; n + 1]; // dp[l][r]: Aが大きい方からl+r個を、左にl個、右にr個動かした場合のスコア
    fn diffabs(x: usize, y: usize) -> i64 {
        (x as i64 - y as i64).abs()
    }

    for i in 0..n {
        for l in 0..=i {
            let r = i - l;
            dp[l + 1][r] = max(dp[l + 1][r], dp[l][r] + v[i].1 * diffabs(v[i].0, l));
            dp[l][r + 1] = max(dp[l][r + 1], dp[l][r] + v[i].1 * diffabs(n - r - 1, v[i].0));
        }
    }

    let mut ans = 0;
    for l in 0..=n {
        let r = n - l;
        ans = max(ans, dp[l][r]);
    }
    println!("{}", ans);
}
