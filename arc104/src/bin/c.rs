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
        n: usize,
        ab: [(i32,i32);n],
    }

    let mut dir = vec![0; n * 2]; // 方向  乗り1 降り-1 不明は0
    let mut dist = vec![0; n * 2]; // 乗ってから降りるまでの距離 不明は0

    let mut error = false;
    for (a, b) in ab {
        if a != -1 {
            error |= dir[a as usize - 1] != 0;
            dir[a as usize - 1] = 1;
        }
        if b != -1 {
            error |= dir[b as usize - 1] != 0;
            dir[b as usize - 1] = -1;
        }

        if a != -1 && b != -1 {
            dist[a as usize - 1] = b - a;
            dist[b as usize - 1] = b - a;
        }
    }
    if error {
        println!("No");
        return;
    }

    let mut dp1 = vec![vec![false; 2 * n + 1]; 2 * n]; // dp1[i][j] : i..j の範囲を同じdistで埋めることができるかどうか

    /*
        i..j の範囲を同じdistで埋めることができる  とは?
        d = (j-i)/2
        dist[k] is d or 0 // i <= k < j
        dir[k] is 1 or 0 // i <= k < i+d
        dir[k] is -1 or 0 // i+d <= k < j

        dir[k] == 1 && dir[l] == -1 && l-k == d なら、 dist[k] == dist[l] && dist[l] == d
    */
    for i in (0..n * 2).step_by(2) {
        let mut max_enter = 0;
        let mut min_leave = n * 2;
        let mut ds = BTreeSet::new();

        for j in i + 1..=n * 2 {
            if dir[j - 1] == 1 {
                max_enter = max(j - 1, max_enter);
            }
            if dir[j - 1] == -1 {
                min_leave = min(j - 1, min_leave);
            }

            if dist[j - 1] != 0 {
                ds.insert(dist[j - 1]);
            }

            let du = (j - i) / 2;
            let d = du as i32;
            let d_ok = match ds.len() {
                0 => true,
                1 => ds.contains(&d),
                _ => false,
            };

            let mid = i + du;
            let mut x = (j - i) % 2 == 0 && max_enter < mid && mid <= min_leave && d_ok;
            if x {
                for k in i..mid {
                    // 重い?
                    if dir[k] == 1 && dir[k + du] == -1 && (dist[k] != d || dist[k + du] != d) {
                        x = false;
                        break;
                    }
                }
            }
            dp1[i][j] = x;
            if dp1[i][j] {
                //println!("ok: {}-{}", i, j);
            }
        }
    }

    let mut dpn = dp1; // dpn[i][j] : 1つ以上のdp1の範囲を連結してi..jにできるか
    for d in (4..=n * 2).step_by(2) {
        for i in (0..=n * 2 - d).step_by(2) {
            for mid in (i + 2..i + d).step_by(2) {
                if dpn[i][mid] && dpn[mid][i + d] {
                    dpn[i][i + d] = true;
                }
            }
        }
    }

    println!("{}", if dpn[0][n * 2] { "Yes" } else { "No" });
}
