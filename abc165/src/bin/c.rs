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

fn dfs(a: &mut Vec<usize>, i: usize, n:usize, m: usize, bq: &Vec<Vec<(usize, usize, i32, usize)>>, score:usize) -> usize {
    let mut val_to_score = vec![0; m + 1];
    for v in &bq[i] {
        let good_val = a[v.0] + v.2 as usize;
        if good_val <= m {
            val_to_score[good_val] += v.3;
        }
    }
    let mut max_score = 0;
    for val in a[i-1]..=m {
        if val == a[i-1] || val_to_score[val] != 0 {
            a[i] = val;
            if i == n-1 {
                max_score = max(max_score, score + val_to_score[val]);
            } else {
                max_score = max(max_score,dfs(a, i+1, n, m, bq, score + val_to_score[val]));
            }
        }
    }
    max_score
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        v: [(Usize1,Usize1,i32,usize);q],
    }

    let mut bq = vec![Vec::new(); n];
    for i in 0..q {
        let b = v[i].1;
        bq[b].push(v[i]);
    }

    let mut a = vec![1; n];
    let ans = dfs(&mut a, 1, n, m, &bq, 0);

    println!("{}", ans);
}
