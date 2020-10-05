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

    let mut b: BTreeMap<(i32, i32), usize> = BTreeMap::new();
    let mut at = 0;
    let mut cg = 0;

    b.insert((at, cg), 1);
    for i in 0..n {
        match s[i] {
            b'A' => at += 1,
            b'T' => at -= 1,
            b'C' => cg += 1,
            b'G' => cg -= 1,
            _ => panic!(),
        }
        *b.entry((at, cg)).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, x) in b {
        ans += x * (x - 1) / 2;
    }

    println!("{}", ans);
}
