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
        s: Bytes,
    }

    let mut r = Vec::with_capacity(n);
    let mut g = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);

    for i in 0..n {
        match s[i] {
            b'R' => r.push(i),
            b'G' => g.push(i),
            b'B' => b.push(i),
            _ => panic!(),
        }
    }

    let mut ans = r.len() as u64 * g.len() as u64 * b.len() as u64;
    for ri in 0..r.len() {
        for gi in 0..g.len() {
            let rpos = r[ri] as i64;
            let gpos = g[gi] as i64;
            let bpos1 = rpos - (rpos - gpos) * 2;
            let bpos2 = gpos - (gpos - rpos) * 2;
            if 0 <= bpos1 && bpos1 < n as i64 && s[bpos1 as usize] == b'B' {
                ans -= 1;
            }
            if 0 <= bpos2 && bpos2 < n as i64 && s[bpos2 as usize] == b'B' {
                ans -= 1;
            }
            if (rpos + gpos) % 2 == 0 {
                let bpos3 = (rpos + gpos) / 2;
                if s[bpos3 as usize] == b'B' {
                    ans -= 1;
                }
            }
        }
    }

    println!("{}", ans);
}
