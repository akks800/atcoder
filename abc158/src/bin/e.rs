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
        p: usize,
        s: Bytes,
    }

    let mut ans = 0u64;
    if p == 2 || p == 5 {
        for i in 0..n {
            if (s[i] - b'0') as usize % p == 0 {
                ans += i as u64 + 1;
            }
        }
    } else {
        // hist[i] : s[j]以降を整数とみなしたとき値 % P == i となるjの値が何個あるか
        // 0<=j<=n
        let mut hist = vec![0; p];
        hist[0] = 1;
        let mut val = 0;
        let mut mul = 10;
        for i in (0..n).rev() {
            val = ((s[i] - b'0') as usize * mul + val) % p;
            mul = mul * 10 % p;
            hist[val] += 1;
        }
        for i in 0..p {
            let x = hist[i];
            if x > 0 {
                ans += x * (x - 1) / 2;
            }
        }
    }

    println!("{}", ans);
}
