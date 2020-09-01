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

    let b = s[0] == b'7' || s[1] == b'7' || s[2] == b'7';
    println!("{}", if b { "Yes" } else { "No" });
}
