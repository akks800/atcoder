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

fn gcd(a: u64, b: u64) -> u64 {
    if a < b {
        gcd(b, a)
    } else {
        let k = a / b;
        if a - k * b == 0 {
            b
        } else {
            gcd(b, a - k * b)
        }
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        v: [u64;n],
    }

    {
        // not coprime の判定
        let mut g = v[0];
        for i in 1..n {
            g = gcd(g, v[i]);
        }
        if g != 1 {
            println!("not coprime");
            return;
        }
    }

    const MAX_A: usize = 1000_000;
    let mut hurui = vec![0; MAX_A + 1];
    let mut ahist = vec![0; MAX_A + 1];
    for i in 0..n {
        ahist[v[i] as usize] += 1;
    }

    let mut have_common_div = false;
    for i in 2..=MAX_A {
        let mut common_div_count = 0;
        if hurui[i] == 0 {
            for j in (i..=MAX_A).step_by(i) {
                common_div_count += ahist[j];
                hurui[j] = 1;
            }
            if common_div_count >= 2 {
                have_common_div = true;
                //println!("i={}, common_div_count={}", i, common_div_count);
            }
        }
    }

    if have_common_div {
        println!("setwise coprime");
    } else {
        println!("pairwise coprime");
    }
}
