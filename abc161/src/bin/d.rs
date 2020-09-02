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

fn dfs(a: &mut Vec<usize>, i: usize, k: &mut usize, is_zero: bool) {
    if i == a.len() {
        if *k == 1 {
            let mut tmp = 0u64;
            for j in 0..a.len() {
                tmp *= 10;
                tmp += a[j] as u64;
            }
            println!("{}", tmp);
        }
        if *k != 0 {
            *k -= 1;
        }
    } else {
        if is_zero {
            for j in 0..=9 {
                a[i] = j;
                dfs(a, i + 1, k, j == 0);
            }
        } else {
            if a[i - 1] != 0 {
                a[i] = a[i - 1] - 1;
                dfs(a, i + 1, k, false);
            }
            a[i] = a[i - 1];
            dfs(a, i + 1, k, false);
            if a[i - 1] != 9 {
                a[i] = a[i - 1] + 1;
                dfs(a, i + 1, k, false);
            }
        }
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        mut k: usize,
    }

    let mut a = vec![0; 11];
    k += 1;
    dfs(&mut a, 1, &mut k, true);
}
