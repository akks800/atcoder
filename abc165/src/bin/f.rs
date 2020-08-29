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
fn partition_point<F>(l0: usize, h0: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
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

const MAX_A: usize = 1000_000_001;

fn dfs(
    a: &Vec<usize>,
    v: &Vec<Vec<usize>>,
    i: usize,
    parent: usize,
    dp: &mut Vec<usize>,
    ans: &mut Vec<usize>,
) {
    let pos = partition_point(0, dp.len(), |pos| dp[pos] < a[i]);
    if pos == dp.len() {
        dp.push(MAX_A);
    }
    let last_val = dp[pos];
    dp[pos] = a[i];
    
    ans[i] = dp.len()-1;
    for &next_i in &v[i] {
        if next_i != parent {
            dfs(a, v, next_i, i, dp, ans);
        }
    }
    
    dp[pos] = last_val;
    if last_val == MAX_A {
        dp.pop();
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        uv: [(Usize1,Usize1);n-1],
    }

    let mut v = vec![Vec::new(); n];
    for x in &uv {
        v[x.0].push(x.1);
        v[x.1].push(x.0);
    }

    let mut dp = Vec::new();
    dp.push(0);
    let mut ans = vec![0; n];

    dfs(&a, &v, 0, n, &mut dp, &mut ans);
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
