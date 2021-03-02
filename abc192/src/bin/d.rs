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

// [l,h) の中でもっとも手前のfalseになる場所を探す. 全てのtrueはfalseより手前にある必要がある.
fn partition_point<F>(l0: u128, h0: u128, f: F) -> u128
where
    F: Fn(u128) -> bool,
{
    let mut l = l0;
    let mut h = h0;

    while h - l > 0 {
        let m = l + (h - l) / 2;
        if f(m) {
            l = m + 1;
        } else {
            h = m;
        }
    }
    return l;
}

//#[proconio::fastout]
fn main() {
    input! {
        x: Chars,
        m: u128,
    }

    let v: Vec<u128> = x.iter().map(|&ch| ch as u128 - '0' as u128).collect();

    let ans;
    if v.len() == 1 {
        if v[0] <= m {
            ans = 1;
        } else {
            ans = 0;
        }
    } else {
        let min_n = *v.iter().max().unwrap() + 1;
        let f = |n: u128| {
            let mut val = 0;
            for &xx in &v {
                val *= n;
                val += xx;
                if val > m {
                    return false;
                }
            }
            return true;
        };
        let max_n = partition_point(min_n, m + 1, f);
        ans = max_n - min_n;
    }

    println!("{}", ans);
}
