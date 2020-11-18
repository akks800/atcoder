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
        n: i64,
    }

    println!("{}", if n < 0 { 0 } else {n});
}

