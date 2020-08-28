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

#[derive(Clone, Debug, Default)]
struct S {
    level_diff: i32,
    min_level: i32,
}

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: [Bytes;n],
    }
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let mut level_diff = 0;
        let mut min_level = 0;
        for j in 0..s[i].len() {
            if s[i][j] == b'(' {
                level_diff += 1;
            } else {
                level_diff -= 1;
                min_level = min(min_level, level_diff);
            }
        }
        v.push(S {
            level_diff,
            min_level,
        });
    }

    v.sort_unstable_by_key(|x: &S| {
        // 上り坂 → 平坦 → 下り坂 の順
        if x.level_diff > 0 {
            // 左から見た穴が浅い順
            -x.min_level
        } else if x.level_diff == 0 {
            10000000
        } else {
            // 右から見た穴が浅い順
            20000000 + x.min_level-x.level_diff
        }
    });

    let mut level = 0;
    for x in &v {
        if level + (*x).min_level < 0 {
            println!("No");
            return;
        }
        level += (*x).level_diff;
    }

    println!("{}", if level == 0 { "Yes" } else { "No" });
}
