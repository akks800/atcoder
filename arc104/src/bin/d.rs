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
        k: usize,
        m: u64,
    }

    /*
     * 1,2,3...,N  の各整数をそれぞれ 0 個以上 K 個以下含むような空でない多重集合であって、平均が x であるものの個数
     *
     * 1-x, 2-x, ... N-x の各整数をそれぞれ 0 個以上 K 個以下含む空でない多重集合で  平均が0 であるものの個数
     * +側の合計と -側の合計を足すと0になるものの個数
     *
     */
    // 片側の合計の上限 k*(N/2)^2/2 125000 ぐらい // 計算量 12500000
    let n2 = n/2;
    let len = k * n2 * (n2+1) / 2;
    let mut v = vec![vec![0u64; len + 11]; 101]; // v[i][j] : 1..=i の各整数を0～k個足した合計がjになるものの個数
    v[0][0] = 1;

    for i in 1..=n {
        for j in 0..=len {
            // v[i][j] = Σ v[i-1][j-m*i]  // 0<=m<=k
            // v[i][j-i] = Σ v[i-1][j-i-m*i]  // 0<=m<=k
            //           = v[i][j]-v[i-1][j]+v[i-1][j-(k+1)*i]
            let mut x = if j >= i { v[i][j - i] } else { 0 };
            x += v[i - 1][j];
            if j >= (k + 1) * i {
                x = x + m - v[i - 1][j - (k + 1) * i];
            }
            v[i][j] = x % m;
        }
    }
    /*
        for i in 0..10 {
            println!( "{:?}", &v[i][0..10]);
        }
    */

    let k64 = k as u64;
    for x in 0..n {
        let l = x;
        let r = n - x - 1;

        let mut ans = 0;
        for i in 1..=len {
            // +側の合計が i になる場合の数 : v[r][i]
            // -側の合計が -i になる場合の数 : v[l][i]
            ans += v[l][i] * v[r][i];
            ans %= m;
        }
        println!("{}", (ans * (k64 + 1) + k64) % m);
    }
}
