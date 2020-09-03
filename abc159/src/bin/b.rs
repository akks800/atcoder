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
    }

    let len = s.len();
    let a = (s.len() - 1) / 2;
    let b = (0..a).all(|i| s[i] == s[a - 1 - i] && s[i] == s[a + 1 + i] && s[i] == s[len - 1 - i]);

    println!("{}", if b { "Yes" } else { "No" });
}
