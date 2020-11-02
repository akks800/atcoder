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
        n: i64,
        k: i64,
    }
    let k = k.abs();
    let f1 = |x| x * (x + 1) / 2;
    let f2 = |x| x * (x + 1) * (x * 2 + 1) / 6;

    if k <= n {
        /*
             Σ i*(i+k)  // 0<=i<=n-k-1
               = kf1(n-k-1) + f2(n-k-1)
             Σ (n-i)*(i+n-k) // 0<=i<=k
               =  Σ (-i*i + i*(n-n+k) + n*(n-k) )
               = -f2(k) + kf1(k) + n*(n-k)*(k+1)
        */
        let a = k * f1(n - k - 1) + f2(n - k - 1);
        let b = -f2(k) + k * f1(k) + n * (n - k) * (k + 1);

        println!("{}", a * 2 + b);
    } else if k <= n * 2 - 2 {
        /*
            Σ i * (2n-k-i) // 0 <= i <= 2n-k
              = -f2(2n-k) + (2n-k)f1(2n-k)
        */
        let x = 2 * n - k;
        let b = -f2(x) + x * f1(x);
        println!("{}", b);
    } else {
        println!("{}", 0);
    }
}
