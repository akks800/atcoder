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

fn calc_score(ss: &Vec<usize>, s5: usize) -> usize {
    const TABLE: [usize; 6] = [1, 10, 100, 1000, 10000, 100000];
    let mut score = 0;
    for i in 0..9 {
        score += (i + 1) * TABLE[ss[i] + if i == s5 { 1 } else { 0 }];
    }
    score
}

//#[proconio::fastout]
fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    let mut ss = vec![0; 9];
    let mut tt = vec![0; 9];
    for i in 0..4 {
        ss[s[i] as usize - '1' as usize] += 1;
        tt[t[i] as usize - '1' as usize] += 1;
    }

    let mut ans: i64 = 0;
    for s5 in 0..9 {
        for t5 in 0..9 {
            if calc_score(&ss, s5) > calc_score(&tt, t5) {
                let s5p = (k - ss[s5] - tt[s5]) as i64;
                let t5p = (k - ss[t5] - tt[t5]) as i64;
                if s5 != t5 {
                    ans += s5p * t5p;
                } else {
                    ans += s5p * (t5p - 1);
                }
            }
        }
    }

    let total_cards = k as i64 * 9;
    println!(
        "{}",
        ans as f64 / ((total_cards - 8) * (total_cards - 9)) as f64
    );
}
