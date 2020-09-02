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
        a: [i64;n],
    }

    /* _dp0[i][j] : [0,i) から j 個選んで、a[i-1]を選ばなかった場合
       _dp1[i][j] : [0,i) から j 個選んで、a[i-1]を選んだ場合
       (i-1)/2 <= j <= (i+1)/2

       (i-1)/2 = b とする
       dp0[i][j-b+1] = dp0[i][j]
       dp1[i][j-b+1] = dp1[i][j]
    */

    const MIN_SUM: i64 = -1000_000_000i64 * 200000;

    let mut dp0 = vec![vec![MIN_SUM; 4]; n + 1];
    let mut dp1 = vec![vec![MIN_SUM; 4]; n + 1];
    fn f(i:usize,j:i64)->usize{
        let b =  (i as i64 - 1) >> 1;
        ( j + 1 - b ) as usize
    }

    dp0[0][f(0,0)] = 0;
    for i in 1..=n {
        let b = (i as i64 - 1) >> 1;
        for j in b..=b + 1 {
            dp0[i][f(i,j)] = max(dp0[i - 1][f(i-1,j)], dp1[i - 1][f(i-1,j)]);
            dp1[i][f(i,j)] = dp0[i - 1][f(i-1,j) - 1] + a[i - 1];
            //println!("{} {}", f(i,j), f(i-1,j));
            //println!("{} {}", dp0[i][f(i,j)], dp1[i][f(i,j)]);
        }
    }
    let j = n as i64 / 2;
    println!("{}", max(dp0[n][f(n,j)], dp1[n][f(n,j)]));
}
