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
        mut a: i32,
        b: i32,
        mut c: i32,
        d: i32,
    }
    loop {
        c -= b;
        if c <= 0 {
            break;
        }
        a -= d;
        if a <= 0 {
            break;
        }
    }

    println!("{}", if c <= 0 {"Yes"} else {"No"});
}
