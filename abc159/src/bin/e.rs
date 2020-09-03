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
        h: usize,
        w: usize,
        k: usize,
        s:[Bytes;h],
    }

    // d[i][j][k] = Σs[i..j][0..k]
    let mut d = vec![vec![vec![0; w + 2]; h + 1]; h];
    for i in 0..h {
        let mut sum = 0;
        for l in 0..w {
            sum += (s[i][l] - b'0') as usize;
            d[i][i + 1][l + 1] = sum;
        }
    }
    for i in 0..h {
        for j in i + 2..=h {
            for l in 0..=w {
                d[i][j][l] = d[i][j - 1][l] + d[j - 1][j][l];
            }
        }
    }

    // step[i][j][l] :  Σs[i..j][l..l+c] <= K となる最大のc
    let mut step = vec![vec![vec![0; w + 1]; h + 1]; h];
    for i in 0..h {
        for j in i + 1..=h {
            let mut r = 0;
            // Σ s[i..j][l..l+c] = d[i][j][l+c]-d[i][j][l]
            d[i][j][w + 1] = 1000000;
            for l in 0..w {
                while d[i][j][r] - d[i][j][l] <= k {
                    r += 1;
                }
                step[i][j][l] = r - l - 1;
            }
        }
    }
/*
    for i in 0..h {
        for j in i + 1..=h {
            print!("{} {} : ", i, j);
            for l in 0..=w {
                print!("{} ", step[i][j][l]);
            }
            println!("");
        }
    }
*/

    const MAX_CT: usize = 100000;
    let mut v = Vec::with_capacity(11);
    fn dfs(
        i: usize,
        h: usize,
        w: usize,
        v: &mut Vec<(usize, usize)>,
        step: &Vec<Vec<Vec<usize>>>,
    ) -> usize {
        if i == h {
            let mut ct = v.len() - 1;
            let mut x = 0;
            while x < w {
                let mut min_step = w + 1;
                for &(i, j) in &*v {
                    min_step = min(min_step, step[i][j][x]);
                }
                if min_step == 0 {
                    return MAX_CT;
                }
                x += min_step;
                ct += 1;
            }
            ct - 1
        } else {
            let mut min_ct = MAX_CT;
            for j in i + 1..=h {
                v.push((i, j));
                min_ct = min(min_ct, dfs(j, h, w, v, step));
                v.pop();
            }
            min_ct
        }
    }

    let ans = dfs(0, h, w, &mut v, &step);
    println!("{}", ans);
}
