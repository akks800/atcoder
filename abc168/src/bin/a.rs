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
    }
    let h = "hon";
    let p = "pon";
    let b = "bon";
    let v = vec![p,p,h,b,h,h,p,h,p,h];

    println!("{}", v[n%10]);
}

