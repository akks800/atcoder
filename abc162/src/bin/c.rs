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


fn gcd( a:usize, b:usize ) -> usize {
    if a < b {
        gcd(b,a)
    } else {
        let k = a/b;
        if a - k*b == 0 {
            b
        } else {
            gcd( b, a-k*b)
        }
    }
}


//#[proconio::fastout]
fn main() {
    input! {
        k:usize,
    }

    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            let ab = gcd(a,b);
            for c in 1..=k {
                ans += gcd(ab,c);
            }
        }
    }

    println!("{}", ans);
}
