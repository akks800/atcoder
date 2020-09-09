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
        n:u64,
        a:u64,
        b:u64,
    }

    let ab = a + b;
    let repeat = n / ab;
    let remain = n - ab * repeat;

    println!("{}", a * repeat + min(remain, a));
}
