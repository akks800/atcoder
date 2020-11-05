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

#[allow(unused_macros)]
macro_rules! chmin {
    ($dst:expr, $src:expr) => {{
        let src = $src;
        let dst = &mut $dst;
        let b = *dst > src;
        if b {
            *dst = src;
        }
        b
    }};
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        xyz: [(i32,i32,i32);n],
    }

    // dp[bit][i] 都市0からbitを経由して都市iに移動する場合の最小コスト。bitは都市0を含む
    // O(n^2*2^n)
    let mut dp = vec![vec![1000_000_000; 1 << n]; n];

    dp[0][1] = 0;
    for bit in 1..1 << n {
        for last_city in 0..n {
            if bit & (1 << last_city) != 0 {
                for city in 0..n {
                    if bit & (1 << city) == 0 {
                        let add = (xyz[city].0 - xyz[last_city].0).abs()
                            + (xyz[city].1 - xyz[last_city].1).abs()
                            + max(0, xyz[city].2 - xyz[last_city].2);

                        chmin!(dp[city][bit | (1 << city)], dp[last_city][bit] + add);
                    }
                }
            }
        }
    }
    //println!("{:?}", dp);

    let mut ans = 1000_000_000;
    for i in 1..n {
        let add =
            (xyz[0].0 - xyz[i].0).abs() + (xyz[0].1 - xyz[i].1).abs() + max(0, xyz[0].2 - xyz[i].2);
        chmin!(ans, dp[i][(1 << n) - 1] + add);
    }

    println!("{}", ans);
}
