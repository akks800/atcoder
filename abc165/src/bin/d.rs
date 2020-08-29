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
        a: u64,
        b: u64,
        n: u64,
    }

    if b-1 <= n {
        // x = b-1
        println!("{}", (a*(b-1))/b);
                
    } else {
        // x = n
        println!("{}", (a*n)/b);
    }
}
