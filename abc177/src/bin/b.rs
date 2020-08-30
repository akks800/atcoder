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
        s: Bytes,
        t: Bytes,
    }

    let mut ans = t.len();
    for i in 0..s.len() - t.len() + 1 {
        let mut count = 0;
        for j in 0..t.len() {
            if t[j] != s[i + j] {
                count += 1;
            }
        }
        ans = min(ans, count);
    }

    println!("{}", ans);
}
