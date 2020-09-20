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

    let ss:String = s.iter().collect();
    if s[s.len()-1] == 's' {
        println!("{}es", ss);

    } else {
        println!("{}s", ss);
    }

}