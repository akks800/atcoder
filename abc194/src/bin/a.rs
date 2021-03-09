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
        a: usize,
        b: usize,
    }

    let nu_kokei = a + b;
    let nu_sibo = b;
    let ans = if nu_kokei >= 15 && nu_sibo >= 8 {
        1
    } else if nu_kokei >= 10 && nu_sibo >= 3 {
        2
    } else if nu_kokei >= 3 {
        3
    } else {
        4
    };
    println!("{}", ans);
}
