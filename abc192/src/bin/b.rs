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
        s: Chars,
    }

    let mut yominiku = true;
    for i in 0..s.len() {
        if i % 2 == 0 && s[i].is_ascii_uppercase() {
            yominiku = false;
        }
        if i % 2 == 1 && s[i].is_ascii_lowercase() {
            yominiku = false;
        }

    }
    println!("{}", if yominiku {"Yes"} else {"No"});
}
