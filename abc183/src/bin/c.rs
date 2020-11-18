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

// 現在 pos に居て、visited は訪問済み
fn dfs(
    k: usize,
    t: &Vec<Vec<usize>>,
    n: usize,
    pos: usize,
    visited: usize,
    time: usize,
    visited_all: usize,
) -> usize {
    if visited == visited_all {
        if pos == 0 && time == k {
            1
        } else {
            0
        }
    } else {
        let mut ans = 0;
        for i in 0..n {
            let b = 1 << i;
            if visited & b == 0 {
                ans += dfs(k, t, n, i, visited | b, time + t[pos][i], visited_all);
            }
        }
        ans
    }
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize;n];n],
    }

    println!("{}", dfs(k, &t, n, 0, 0, 0, (1 << n) - 1));
}
